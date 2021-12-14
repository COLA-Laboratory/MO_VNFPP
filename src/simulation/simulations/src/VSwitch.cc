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

#include "VSwitch.h"

namespace simulation {

Define_Module(VSwitch);

void VSwitch::initSwitch() {
    processed_signal = registerSignal("server_msg_processed");
    energy_signal = registerSignal("server_energy_cost");

    mean_waiting_signal = registerSignal("server_mean_waiting");
    variance_waiting_signal = registerSignal("server_variance_waiting");

    vnfs_idle = new bool[k_vm];

    for(int i = 0; i < k_vm; i++)
        vnfs_idle[i] = true;
}

void VSwitch::registerReceivedSignal() {
    received_cnt_signal = registerSignal("server_received_count");
}

void VSwitch::registerDroppedSignal() {
    dropped_cnt_signal = registerSignal("server_dropped_count");
}

void VSwitch::handleMessage(cMessage *msg) {

    cModule* network = getParentModule();
    int p_sdn = network->par("p_sdn");

    int lb = id * k_vm;
    int ub = lb + k_vm - 1;

    if (msg->isSelfMessage()) {

        delete msg;

        auto dmsg = popMessageAndSchedule();

        if (dmsg->getHasFinished()) {
            sendUp(dmsg);
            return;
        }

        int destination = dmsg->getDestination();

        if (destination < lb || destination > ub) {
            auto rng = intuniform(0, 99);

            if (!dmsg->getVisitedSDN() && rng < p_sdn) {
                send(dmsg, "sdn_gate$o");
            } else {
                int port = k_vm;
                send(dmsg, "gate$o", port);
            }

            if (isIdle()) {
                total_active = total_active + (simTime() - last_idle);
            }

        } else {
            int port = destination - lb;
            send(dmsg, "gate$o", port);
        }

    } else if (!strcmp(msg->getName(), "vnf_idle")) {
        vnfs_idle[msg->getKind()] = true;

        if (isIdle()) {
            total_active = total_active + (simTime() - last_idle);
        }

        delete msg;

    } else if (!strcmp(msg->getName(), "vnf_busy")) {
        vnfs_idle[msg->getKind()] = false;
        delete msg;
    } else {

        if (isIdle()) {
            last_idle = simTime();
        }

        auto *dmsg = storeMessageAndSchedule(msg);

        if (dmsg != nullptr) {
            dmsg->setSrcServer(id);
        }
    }
}

bool VSwitch::isIdle() {

    auto all_idle = true;

    for (int i = 0; i < k_vm; i++) {
        all_idle = all_idle && vnfs_idle[i];
    }

    return all_idle && queue.getLength() == 0;
}

void VSwitch::sendUp(cMessage *msg) {
    send(msg, "gate$o", k_vm);
}

void VSwitch::end() {
    // Do nothing
}

} //namespace
