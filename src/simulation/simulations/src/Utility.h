/*
 * Utility.h
 *
 *  Created on: Aug 15, 2018
 *      Author: joebillingsley
 */

#ifndef SRC_UTILITY_H_
#define SRC_UTILITY_H_

#include <vector>
#include <string>
#include <omnetpp.h>

using namespace std;
using namespace omnetpp;

namespace Utility {

struct VNF_Placements {
    vector<vector<vector<int>>> services;
    vector<int> count;
};

VNF_Placements parseVnfPlacements(const char * vnf_placements_str);

}

#endif /* SRC_UTILITY_H_ */
