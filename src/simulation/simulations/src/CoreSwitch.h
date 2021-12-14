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

#ifndef CORESWITCH_H_
#define CORESWITCH_H_

#include <string>
#include <omnetpp.h>
#include "ISwitch.h"

using namespace omnetpp;
using std::vector;

namespace simulation {

class CoreSwitch : public ISwitch
{
  protected:
    void initSwitch();
    void registerReceivedSignal();
    void registerDroppedSignal();
    void handleMessage(cMessage *msg);
    void end();

    void sendDown(DestMessage *dmsg);
    void scheduleArrival(int service_id);

    simsignal_t completed_signal;
    simsignal_t msg_hop_cnt_signal;

    std::vector<double> service_arr_rates;
    std::vector<std::vector<std::vector<int>>> vnf_placements;
};

} //namespace

#endif /* CORESWITCH_H_ */
