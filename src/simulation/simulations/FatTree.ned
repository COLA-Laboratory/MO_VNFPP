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

package simulations;

import simulations.src.VNF;
import simulations.src.VSwitch;
import simulations.src.EdgeSwitch;
import simulations.src.AggregateSwitch;
import simulations.src.CoreSwitch;
import simulations.src.Controller;

network FatTree
{
    parameters:
        // m-port n-tree
        int k; // num_ports
        int vm_k;

        int p_sdn;

        int num_core = int((k / 2) * (k / 2));
        int num_pods = k;
        int num_agg = int(num_pods * (k / 2));
        int num_edge = int(num_pods * (k / 2));
        int num_servers = int(num_edge * (k / 2));
        int num_vms = int(num_servers * vm_k);

        string vnf_placements;

//        @statistic[Time_In_System_Core](title="Time in System - Core"; source=core_completed; record=stats; interpolationmode=none);
        @statistic[Time_In_System_VNF](title="Time in System - VNF"; source=vnf_completed; record=stats; interpolationmode=none);
        @statistic[Energy_Consumption_Total](title="Total Energy"; source=sum(core_energy_cost) + sum(agg_energy_cost) + sum(edge_energy_cost) + sum(server_energy_cost); record=last; interpolationmode=none);
        @statistic[Packet_Dropped_Total](title="Total Dropped"; source=service_packet_loss; record=stats; interpolationmode=none);
      
//        @statistic[Rec_Vnf](title="Arrival Rate - Vnf"; source=vnf_received_count; record=vector; interpolationmode=none);	
//        @statistic[Mean_Waiting_Vnf](title="Mean Waiting Time - Vnf"; source=vnf_mean_waiting; record=vector; interpolationmode=none);
//        @statistic[Variance_Waiting_Vnf](title="Variance Waiting Time - Vnf"; source=vnf_variance_waiting; record=vector; interpolationmode=none);
        
//        @statistic[Rec_Server](title="Arrival Rate - Server"; source=server_received_count; record=vector; interpolationmode=none);	
//        @statistic[Mean_Waiting_Server](title="Mean Waiting Time - Server"; source=server_mean_waiting; record=vector; interpolationmode=none);
//        @statistic[Variance_Waiting_Server](title="Variance Waiting Time - Server"; source=server_variance_waiting; record=vector; interpolationmode=none);
        
//        @statistic[Rec_Edge](title="Arrival Rate - Edge"; source=edge_received_count; record=vector; interpolationmode=none);	
//        @statistic[Mean_Waiting_Edge](title="Mean Waiting Time - Edge"; source=edge_mean_waiting; record=vector; interpolationmode=none);
//        @statistic[Variance_Waiting_Edge](title="Variance Waiting Time - Edge"; source=edge_variance_waiting; record=vector; interpolationmode=none);
        
//        @statistic[Rec_Agg](title="Arrival Rate - Agg"; source=agg_received_count; record=vector; interpolationmode=none);	
//        @statistic[Mean_Waiting_Agg](title="Mean Waiting Time - Agg"; source=agg_mean_waiting; record=vector; interpolationmode=none);
//        @statistic[Variance_Waiting_Agg](title="Variance Waiting Time - Agg"; source=agg_variance_waiting; record=vector; interpolationmode=none);
        
//        @statistic[Rec_Core](title="Arrival Rate - Core"; source=core_received_count; record=vector; interpolationmode=none);	
//        @statistic[Mean_Waiting_Core](title="Mean Waiting Time - Core"; source=core_mean_waiting; record=vector; interpolationmode=none);
//        @statistic[Variance_Waiting_Core](title="Variance Waiting Time - Core"; source=core_variance_waiting; record=vector; interpolationmode=none);
        
//        @statistic[Mean_Waiting_Total](title="Mean Waiting Time - Total"; source=overall_mean_waiting; record=stats; interpolationmode=none);
//        @statistic[Variance_Waiting_Total](title="Variance Waiting Time - Total"; source=overall_variance_waiting; record=stats; interpolationmode=none);	
        
//        @statistic[Energy_Consumption_Core](title="Core Energy"; source=core_energy_cost; record=sum; interpolationmode=none);
//		@statistic[Energy_Consumption_Agg](title="Agg Energy"; source=agg_energy_cost; record=sum; interpolationmode=none);
//		@statistic[Energy_Consumption_Edge](title="Edge Energy"; source=edge_energy_cost; record=sum; interpolationmode=none);
//		@statistic[Energy_Consumption_Server](title="Server Energy"; source=server_energy_cost; record=sum; interpolationmode=none);

        @statistic[Rec_VNF](title="Arrival Rate - VNF"; source=vnf_received_count; record=stats, vector; interpolationmode=none);
        @statistic[Rec_Server](title="Arrival Rate - Server"; source=server_received_count; record=stats, vector; interpolationmode=none);
        @statistic[Rec_Edge](title="Arrival Rate - Edge"; source=edge_received_count; record=stats, vector; interpolationmode=none);
        @statistic[Rec_Agg](title="Arrival Rate - Aggregate"; source=agg_received_count; record=stats, vector; interpolationmode=none);
        @statistic[Rec_Core](title="Arrival Rate - Core"; source=core_received_count; record=stats, vector, vector; interpolationmode=none);
        @statistic[Rec_SDN](title="Arrival Rate - SDN"; source=sdn_received_count; record=stats, vector; interpolationmode=none);
        	
        @statistic[Drop_VNF](title="Dropped Percent - VNF"; source=vnf_dropped_count; record=stats, vector; interpolationmode=none);
        @statistic[Drop_Server](title="Dropped Percent - Server"; source=server_dropped_count; record=stats, vector; interpolationmode=none);
        @statistic[Drop_Edge](title="Dropped Percent - Edge"; source=edge_dropped_count; record=stats, vector; interpolationmode=none);
        @statistic[Drop_Agg](title="Dropped Percent - Aggregate"; source=agg_dropped_count; record=stats, vector; interpolationmode=none);
        @statistic[Drop_Core](title="Dropped Percent - Core"; source=core_dropped_count; record=stats, vector; interpolationmode=none);
        @statistic[Drop_SDN](title="Dropped Percent - SDN"; source=sdn_dropped_count; record=stats, vector; interpolationmode=none);

//        @statistic[Proc_VNF](title="Queueing Time - VNF"; source=vnf_msg_processed; record=stats; interpolationmode=none);
//        @statistic[Proc_Server](title="Queueing Time - Server"; source=server_msg_processed; record=v; interpolationmode=none);
//        @statistic[Proc_Edge](title="Queueing Time - Edge"; source=edge_msg_processed; record=stats; interpolationmode=none);
//        @statistic[Proc_Agg](title="Queueing Time - Aggregate"; source=agg_msg_processed; record=stats; interpolationmode=none);
//        @statistic[Proc_Core](title="Queueing Time - Core"; source=core_msg_processed; record=stats; interpolationmode=none);
//        @statistic[Proc_SDN](title="Queueing Time - SDN"; source=sdn_msg_processed; record=stats; interpolationmode=none);

//        @statistic[Hop_Count](title="Hop Count"; source="msg_hop_count"; record=stats; interpolationmode=none);

    types:
        channel Channel extends ned.DelayChannel
        {
            delay = 0ms;
        }

    submodules:
        vms[num_vms]: VNF;
        servers[num_servers]: VSwitch;
        edge[num_edge]: EdgeSwitch;
        aggregate[num_agg]: AggregateSwitch;
        core[num_core]: CoreSwitch;
        sdn: Controller;
    connections:

        for i=0..num_vms-1 {
            vms[i].gate++ <--> Channel <--> servers[int(floor(i/vm_k))].gate++;
        }

        for i=0..num_servers-1 {
            servers[i].gate++ <--> Channel <--> edge[int(floor(i/(k/2)))].gate++;
        }

        for i=0..num_servers-1 {
            servers[i].sdn_gate <--> Channel <--> sdn.gate++;
        }

        for i=0..num_edge - 1, for j=0..num_agg - 1 {
            edge[i].gate++ <--> Channel <--> aggregate[j].gate++ if j >= floor(i/(k /2)) * (k/2) && j < floor(i/(k/2)) * (k/2) + (k/2);
        }

        for i=0..num_agg - 1, for j=0..num_core - 1 {
            aggregate[i].gate++ <--> Channel <--> core[j].gate++ if j >= ((i % (k / 2)) * (k / 2)) && j < ((i % (k / 2)) * (k / 2) + (k/2));
        }
}
