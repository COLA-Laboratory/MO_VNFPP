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

#include "Controller.h"

namespace simulation {

Define_Module(Controller);

void Controller::initSwitch() {
    processed_signal = registerSignal("sdn_msg_processed");
    energy_signal = registerSignal("sdn_energy_cost");
}

void Controller::registerReceivedSignal() {
    received_cnt_signal = registerSignal("sdn_received_count");
}

void Controller::registerDroppedSignal() {
    dropped_cnt_signal = registerSignal("sdn_dropped_count");
}

void Controller::handleMessage(cMessage *msg) {
    if (msg->isSelfMessage()) {

        delete msg;

        auto dmsg = popMessageAndSchedule();

        int port = dmsg->getSrcServer();
        send(dmsg, "gate$o", port);

        if (isIdle())
            total_active = total_active + (simTime() - last_idle);

    } else {

        if (isIdle())
            last_idle = simTime();

        storeMessageAndSchedule(msg);
    }
}

void Controller::end() {
    // Do nothing
}

} //namespace
