//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see http://www.gnu.org/licenses/.
// 

#include "VNF.h"
#include "Utility.h"
#include <algorithm>

namespace simulation {

Define_Module(VNF);

bool VNF::is_set = false;
int VNF::set_id;

std::vector<unsigned long> VNF::created_count;
std::vector<unsigned long> VNF::succeeded_count;

void VNF::initSwitch() {
    completed_signal = registerSignal("vnf_completed");
    processed_signal = registerSignal("vnf_msg_processed");
    energy_signal = registerSignal("ignore_energy_cost"); // Servers use energy, not VNFs

    mean_waiting_signal = registerSignal("vnf_mean_waiting");
    variance_waiting_signal = registerSignal("vnf_variance_waiting");

    msg_hop_cnt_signal = registerSignal("msg_hop_count");

    cModule* network = getParentModule();

    // Retrieve service arrival rates
    base_prod_rate = par("service_prod_rate");

    // Parse VNF placement
    auto vnf_placements_str = network->par("vnf_placements").stringValue();

    auto result = Utility::parseVnfPlacements(vnf_placements_str);

    vnf_placements = result.services;
    num_instances = result.count;

    num_services = vnf_placements.size();

    // Produce initial messages for each service
    for (int i = 0; i < num_services; i++)
        scheduleProduction(i);

    // Got to change at bottom too, only triggers for VNFs that have received some packets
    if (!is_set) {
        packet_loss_signal = registerSignal("service_packet_loss");

        overall_mean_signal = registerSignal("overall_mean_waiting");
        overall_variance_signal = registerSignal("overall_variance_waiting");

        for (auto i = 0; i < num_services; i++) {
            created_count.push_back(0);
            succeeded_count.push_back(0);
        }

        is_set = true;
        set_id = id;
    }
}

void VNF::registerReceivedSignal() {
    received_cnt_signal = registerSignal("vnf_received_count");
}

void VNF::registerDroppedSignal() {
    dropped_cnt_signal = registerSignal("vnf_dropped_count");
}

void VNF::handleMessage(cMessage *msg) {

    if (msg->isSelfMessage() && !strcmp(msg->getName(), "new")) {

        int service_id = msg->getKind();
        auto service = vnf_placements[service_id];

        delete msg;

        // Produce a new message for service i
        // Assign the message destinations randomly from applicable destinations

        auto *dmsg = new DestMessage();
        dmsg->setHopCount(0);
        dmsg->setProduced(simTime());
        dmsg->setIsExternalMessage(false);
        dmsg->setPosition(1); // Already visited the producing VNF
        dmsg->setServiceId(service_id);

        auto candidates = service[dmsg->getPosition()];

        dmsg->setDestination(selectDestination(id, candidates, k, k_vm));

        sendUp(dmsg);
        scheduleProduction(service_id);

        created_count[service_id] += 1;

    } else if (msg->isSelfMessage() && !strcmp(msg->getName(), "process")) {

        delete msg;

        auto dmsg = popMessageAndSchedule();
        dmsg->setPosition(dmsg->getPosition() + 1);

        auto service = vnf_placements[dmsg->getServiceId()];

        if (isIdle()) {
            auto vm_msg = new cMessage("vnf_idle");
            vm_msg->setKind(id % k_vm);
            sendUp(vm_msg);
        }

        // Check if the message has finished the service
        if (!dmsg->getIsExternalMessage()
                && dmsg->getPosition() == service.size()) {

            simtime_t travel_time = simTime() - dmsg->getProduced();

            dmsg->setHasFinished(true);

            emit(completed_signal, travel_time);
            emit(msg_hop_cnt_signal, dmsg->getHopCount());

            total_wait += travel_time.dbl();
            sq_total_wait += travel_time.dbl() * travel_time.dbl();
            total_n += 1;

            succeeded_count[dmsg->getServiceId()] += 1;

            delete dmsg;
            return;
        }

        auto candidates = service[dmsg->getPosition()];

        dmsg->setDestination(selectDestination(id, candidates, k, k_vm));

        // Otherwise forward it towards its destination
        sendUp(dmsg);

    } else {
        storeMessageAndSchedule(msg);

        auto vm_msg = new cMessage("vnf_busy");
        vm_msg->setKind(id % k_vm);
        sendUp(vm_msg);
    }
}

int VNF::selectDestination(int self, std::vector<int> candidates, int k,
        int k_vm) {

    // Find nearest VNFs
    std::vector<int> destinations;

    auto min_hops = INT_MAX;

    for (int i = 0; i < candidates.size(); i++) {
        auto hops = 3;

        for (int j = 0; j <= 2; j++) {
            auto div = k_vm * pow((k / 2), j);

            if (floor(self / div) == floor(candidates[i] / div)) {
                hops = j;
                break;
            }
        }

        if (hops < min_hops) {
            destinations.clear();
            min_hops = hops;
        }

        if (hops == min_hops) {
            destinations.push_back(candidates[i]);
        }
    }

    // Choose the nearest numerically or the smaller value
    int destination = 0;
    int min_dist = INT_MAX;

    for (int i = 0; i < destinations.size(); i++) {
        int dist = abs(self - destinations[i]);

        if (dist < min_dist || (dist == min_dist && destinations[i] < destination)) {
            min_dist = dist;
            destination = destinations[i];
        }
    }

    return destination;
}

void VNF::sendUp(cMessage *msg) {
    send(msg, "gate$o", 0);
}

void VNF::scheduleProduction(int service_id) {

    std::vector<int> init_vms = vnf_placements[service_id][0];

    // Check if we are the right VNF for the production
    std::vector<int>::iterator it;
    it = find(init_vms.begin(), init_vms.end(), id);

    if (it == init_vms.end()) {
        return;
    }

    // External traffic is divided evenly between the initial VNFs for each service
    auto service_prod = base_prod_rate / ((double) num_instances[service_id]);

    if (service_prod == 0) {
        return;
    }

    auto inter_production_time = SimTime(1 / service_prod);

    simtime_t production_rate = exponential(inter_production_time);

    cMessage *msg_arr_evt = new cMessage("new", service_id);
    scheduleAt(simTime() + production_rate, msg_arr_evt);
}

void VNF::end() {
    if (id == set_id) {
        for (auto i = 0; i < num_services; i++) {
            auto packets_kept = (long double) succeeded_count[i] / (long double) created_count[i];

            emit(packet_loss_signal,  1 - packets_kept);
        }

        auto mean = total_wait / total_n;

        emit(overall_mean_signal, mean);
        emit(overall_variance_signal, sq_total_wait / total_n - (mean * mean));
    }
}

} //namespace
