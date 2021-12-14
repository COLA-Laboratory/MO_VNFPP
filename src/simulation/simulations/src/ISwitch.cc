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

#include "ISwitch.h"

namespace simulation {

Register_Abstract_Class(ISwitch);

void ISwitch::initialize() {

    cModule* network = getParentModule();
    k = network->par("k");
    k_vm = network->par("vm_k");

    id = getIndex();

    queue = cQueue();
    queue_length = par("queue_length");

    double service_rate = getServiceRate();

    if (service_rate > 0) {
        inter_service_time = SimTime(1 / service_rate);
    } else {
        inter_service_time = 0;
    }

    idle_cost = par("energy_idle");
    busy_cost = par("energy_busy");

    last_idle = simTime();
    total_active = 0;

    initSwitch();
}

DestMessage* ISwitch::popMessageAndSchedule() {
    DestMessage *dmsg = check_and_cast<DestMessage *>(queue.pop());

    auto waiting_time = simTime() - dmsg->getQueued();
    emit(processed_signal, waiting_time);

    sum_wait += waiting_time.dbl();
    sq_sum_wait += waiting_time.dbl() * waiting_time.dbl();
    n += 1;

    if (!queue.isEmpty())
        scheduleProcessingEvent();

    return dmsg;
}

DestMessage* ISwitch::storeMessageAndSchedule(cMessage *msg) {

    num_msg_received++;

    if (queue.isEmpty())
        scheduleProcessingEvent();

    DestMessage *dmsg = check_and_cast<DestMessage *>(msg);

    if (queue.getLength() >= queue_length) {
        num_msg_dropped++;
        delete dmsg;
        return nullptr;
    }

    queue.insert(dmsg);

    dmsg->setQueued(simTime());
    dmsg->setHopCount(dmsg->getHopCount() + 1);

    return dmsg;
}

double ISwitch::getServiceRate() {
    return par("service_rate");
}

void ISwitch::sendUp(cMessage *msg) {

    // First k/2 ports are towards the edge
    auto port = intuniform(k / 2, k - 1);

    send(msg, "gate$o", port);
}

void ISwitch::scheduleProcessingEvent() {
    simtime_t service_rate = exponential(inter_service_time);

    cMessage *process_msg_evt = new cMessage("process", 2);
    scheduleAt(simTime() + service_rate, process_msg_evt);
}

void ISwitch::finish() {

    if (!isIdle()) {
        total_active = total_active + (simTime() - last_idle);
    }

    if (num_msg_received != 0) {
        registerReceivedSignal();
        emit(received_cnt_signal, num_msg_received / simTime());
    }

    if (num_msg_received != 0) {
        registerDroppedSignal();
        emit(dropped_cnt_signal,
                ((double) num_msg_dropped) / ((double) num_msg_received));
    }

    if (num_msg_received == 0) {
        emit(energy_signal, 0);
        end();
        return;
    }

    auto avg_busy = total_active / simTime();

    emit(energy_signal, avg_busy * busy_cost + (1 - avg_busy) * idle_cost);

    auto mean = sum_wait / n;

    emit(mean_waiting_signal, mean);
    emit(variance_waiting_signal, sq_sum_wait / n - (mean * mean));

    end();
}

bool ISwitch::isIdle() {
    return queue.isEmpty();
}

} //namespace
