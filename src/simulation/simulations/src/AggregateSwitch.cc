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

#include "AggregateSwitch.h"

namespace simulation {

Define_Module(AggregateSwitch);

void AggregateSwitch::initSwitch() {
    processed_signal = registerSignal("agg_msg_processed");
    energy_signal = registerSignal("agg_energy_cost");

    mean_waiting_signal = registerSignal("agg_mean_waiting");
    variance_waiting_signal = registerSignal("agg_variance_waiting");
}

void AggregateSwitch::registerReceivedSignal() {
    received_cnt_signal = registerSignal("agg_received_count");
}

void AggregateSwitch::registerDroppedSignal() {
    dropped_cnt_signal = registerSignal("agg_dropped_count");
}

void AggregateSwitch::handleMessage(cMessage *msg) {
    int num_in_pod = (k / 2) * (k / 2) * k_vm;

    int lb = floor(id / (k / 2)) * num_in_pod;
    int ub = lb + num_in_pod - 1;

    if (msg->isSelfMessage()) {

        delete msg;

        auto dmsg = popMessageAndSchedule();

        if (dmsg->getHasFinished()) {
            sendUp(dmsg);
            return;
        }

        int destination = dmsg->getDestination();

        if (destination < lb || destination > ub) {
            sendUp(dmsg);
        } else {
            int port = floor((destination - lb) / ((k / 2) * k_vm));
            send(dmsg, "gate$o", port);
        }

        if (isIdle())
            total_active = total_active + (simTime() - last_idle);

    } else {

        if (isIdle())
            last_idle = simTime();

        storeMessageAndSchedule(msg);
    }
}

void AggregateSwitch::end() {
    // Do nothing
}

} //namespace
