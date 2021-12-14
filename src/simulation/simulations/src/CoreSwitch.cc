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

#include "CoreSwitch.h"
#include "Utility.h"

namespace simulation {

Define_Module(CoreSwitch);

void CoreSwitch::initSwitch() {

    completed_signal = registerSignal("core_completed");
    msg_hop_cnt_signal = registerSignal("msg_hop_count");
    processed_signal = registerSignal("core_msg_processed");
    energy_signal = registerSignal("core_energy_cost");

    mean_waiting_signal = registerSignal("core_mean_waiting");
    variance_waiting_signal = registerSignal("core_variance_waiting");

    cModule* network = getParentModule();

    // Retrieve service arrival rates
    cStringTokenizer tokenizer_prod_rates(par("service_arr_rates"));
    service_arr_rates = tokenizer_prod_rates.asDoubleVector();

//    int num_services = service_arr_rates.size();

    // Parse VNF placement
//    auto vnf_placements_str = network->par("vnf_placements").stringValue();

    // 'Receive' initial messages for each service
    for (int i = 0; i < service_arr_rates.size(); i++)
        scheduleArrival(i);

//    vnf_placements = Utility::parseVnfPlacements(vnf_placements_str,
//            num_services);
}

void CoreSwitch::registerReceivedSignal() {
    received_cnt_signal = registerSignal("core_received_count");
}

void CoreSwitch::registerDroppedSignal() {
    dropped_cnt_signal = registerSignal("core_dropped_count");
}

void CoreSwitch::handleMessage(cMessage *msg) {

    if (msg->isSelfMessage() && !strcmp(msg->getName(), "new")) {

        delete msg;

        // Produce a new message for service i
        // Assign the message destinations randomly from applicable destinations

        int service_id = msg->getKind();
        auto service = vnf_placements[service_id];

        auto *dmsg = new DestMessage();
        dmsg->setHopCount(0);
        dmsg->setProduced(simTime());
        dmsg->setIsExternalMessage(true);

        for (int i = 0; i < service.size(); i++) {
            auto destinations = service[i];
            auto rng = intuniform(0, destinations.size() - 1);

            dmsg->setDestination(destinations[rng]);
        }

        sendDown(dmsg);
        scheduleArrival(service_id);

    } else if (msg->isSelfMessage() && !strcmp(msg->getName(), "process")) {

        delete msg;
        auto dmsg = popMessageAndSchedule();

        // Check if the message has finished the service
        if (dmsg->getIsExternalMessage() && dmsg->getHasFinished()) {

            simtime_t travel_time = simTime() - dmsg->getProduced();

            emit(completed_signal, travel_time);
            emit(msg_hop_cnt_signal, dmsg->getHopCount());

            delete dmsg;
            return;
        }

        // Otherwise forward it towards its destination
        sendDown(dmsg);

        if (isIdle())
            total_active = total_active + (simTime() - last_idle);

    } else {
        if (isIdle())
            last_idle = simTime();

        storeMessageAndSchedule(msg);
    }
}

void CoreSwitch::sendDown(DestMessage *dmsg) {
    int destination = dmsg->getDestination();
    int port = floor(destination / ((k / 2) * (k / 2) * k_vm));

    send(dmsg, "gate$o", port);
}

void CoreSwitch::scheduleArrival(int service_id) {
    auto service_arr_rate = service_arr_rates[service_id];

    // External traffic is divided evenly between the core switches
    service_arr_rate = service_arr_rate / ((k / 2) * (k / 2));

    if (service_arr_rate == 0) {
        return;
    }

    auto inter_arrival_time = SimTime(1 / service_arr_rate);

    simtime_t production_rate = exponential(inter_arrival_time);

    cMessage *msg_arr_evt = new cMessage("new", service_id);
    scheduleAt(simTime() + production_rate, msg_arr_evt);
}

void CoreSwitch::end() {
    // Do nothing
}

} /* namespace canonical_tree */
