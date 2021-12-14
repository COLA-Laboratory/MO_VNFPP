/*
 * Utility.cpp
 *
 *  Created on: Aug 15, 2018
 *      Author: joebillingsley
 */

#include "Utility.h"
#include <map>
#include <string>
#include <tuple>

namespace Utility {

VNF_Placements parseVnfPlacements(const char * vnf_placements_str) {

    cStringTokenizer a_tokenizer_placements(vnf_placements_str, ";");

    std::map<int, int> num_instances;
    while (a_tokenizer_placements.hasMoreTokens()) {

        auto service_def = a_tokenizer_placements.nextToken();

        cStringTokenizer service_id(service_def, ":");
        auto s_id_str = service_id.nextToken();
        auto s_id = std::stoi(s_id_str);

        // Insert doesn't override the existing value
        num_instances.insert({s_id, 0});
        num_instances[s_id] ++;
    }

    cStringTokenizer b_tokenizer_placements(vnf_placements_str, ";");

    auto vnf_placements = vector<vector<vector<int>>>();
    auto ret_num_instances = vector<int>();

    while (b_tokenizer_placements.hasMoreTokens()) {

        auto service_def = b_tokenizer_placements.nextToken();

        cStringTokenizer service_id(service_def, ":");
        auto s_id_str = service_id.nextToken();
        auto s_id = std::stoi(s_id_str);

        cStringTokenizer tokenizer_vnfs(service_id.nextToken(), ",");

        auto service = vector<vector<int>>();
        while (tokenizer_vnfs.hasMoreTokens()) {
            int position = std::stoi(tokenizer_vnfs.nextToken());

            // All of the positions that the first VNF can be in
            vector<int> dummy;
            dummy.push_back(position);

            service.push_back(dummy);
        }

        vnf_placements.push_back(service);
        ret_num_instances.push_back(num_instances[s_id]);
    }

    return VNF_Placements{vnf_placements, ret_num_instances};
}
}

