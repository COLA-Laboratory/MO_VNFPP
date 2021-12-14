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

#ifndef __SIMULATION_ISWITCH_H_
#define __SIMULATION_ISWITCH_H_

#include <omnetpp.h>
#include "DestMessage_m.h"

using namespace omnetpp;

namespace simulation {

class ISwitch : public cSimpleModule
{
  protected:

    void initialize();
    virtual void finish();

    virtual bool isIdle();
    virtual void initSwitch() = 0;
    virtual void registerReceivedSignal() = 0;
    virtual void registerDroppedSignal() = 0;
    virtual void end() = 0;

    DestMessage* storeMessageAndSchedule(cMessage *msg);
    DestMessage* popMessageAndSchedule();

    virtual double getServiceRate();
    virtual void sendUp(cMessage *msg);

    simtime_t inter_service_time;

    int id, k, k_vm;
    int num_msg_received = 0;
    int num_msg_dropped = 0;

    double sum_wait = 0, sq_sum_wait = 0;
    long int n = 0;

    double idle_cost, busy_cost;

    simtime_t last_idle;
    simtime_t total_active;

    cQueue queue;
    int queue_length;

    simsignal_t received_cnt_signal;
    simsignal_t dropped_cnt_signal;
    simsignal_t processed_signal;
    simsignal_t energy_signal;

    simsignal_t mean_waiting_signal;
    simsignal_t variance_waiting_signal;

 private:
    void scheduleProcessingEvent();
};

} //namespace

#endif
