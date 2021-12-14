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

#ifndef __NFV_FATTREE_VNF_H_
#define __NFV_FATTREE_VNF_H_

#include <omnetpp.h>
#include "ISwitch.h"

using namespace omnetpp;

namespace simulation {

static double total_wait = 0, sq_total_wait = 0;
static long int total_n = 0;

class VNF : public ISwitch
{
  protected:
    void initSwitch();
    void handleMessage(cMessage *msg);
    void registerReceivedSignal();
    void registerDroppedSignal();
    void sendUp(cMessage *msg);
    void end();

    void scheduleProduction(int service_id);
    int selectDestination(int id, std::vector<int> candidates, int k, int k_vm);

    simsignal_t completed_signal;
    simsignal_t msg_hop_cnt_signal;
    simsignal_t packet_loss_signal;

    simsignal_t overall_mean_signal;
    simsignal_t overall_variance_signal;

    int num_services;
    std::vector<int> num_instances;
    double base_prod_rate;
    std::vector<std::vector<std::vector<int>>> vnf_placements;

  private:
    std::vector<std::vector<int>> vnf_chains;

    static bool is_set;
    static int set_id;
    static std::vector<unsigned long> created_count;
    static std::vector<unsigned long> succeeded_count;
};

} //namespace

#endif
