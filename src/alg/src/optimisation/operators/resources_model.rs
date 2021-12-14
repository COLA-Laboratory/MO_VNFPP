use crate::optimisation::operators::constraint_aware::expand_solution;
use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;
use crate::{
    fat_tree::FatTree,
    model::{iterate_route, NodeType},
};

pub struct ResourcesEvaluate {
    qm: QueueingModel,
    dc: FatTree,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    do_balance: bool,
    do_cnstr_handling: bool,
}

impl ResourcesEvaluate {
    pub fn new(
        qm: QueueingModel,
        fat_tree: &FatTree,
        services: &Vec<Service>,
        vnfs: &Vec<VNF>,
        do_balance: bool,
        do_cnstr_handling: bool,
    ) -> ResourcesEvaluate {
        ResourcesEvaluate {
            qm,
            dc: fat_tree.clone(),
            services: services.clone(),
            vnfs: vnfs.clone(),
            do_balance,
            do_cnstr_handling,
        }
    }
}

impl Evaluate<Service> for ResourcesEvaluate {
    fn get_number_objectives(&self) -> usize {
        2
    }

    fn apply(&mut self, solution: &Solution<Service>) -> Constraint<Vec<f64>> {
        let no_aa = vec![false; self.services.len()];
        let no_mi = vec![None; self.vnfs.len()];

        let (sequences, solution) = expand_solution(
            &solution,
            &self.dc,
            &self.services,
            &no_aa,
            &no_mi,
            self.do_balance,
            self.do_cnstr_handling,
        );

        let mut routes = Vec::new();
        for (s_id, sequence) in sequences {
            let route = find_routes(sequence, &self.dc);
            routes.push((s_id, route));
        }

        // Use accurate model to get energy consumption
        let (_, _, energy, _) = self.qm.evaluate(&self.services, &solution, &routes);

        let mut utilizations = Vec::with_capacity(self.services.len());

        for (_, route) in routes {
            let mut node_pv = vec![0.0; route.len()]; // Probability of visiting this node
            node_pv[0] = 1.0;

            iterate_route(&route, |curr| {
                let num_next = route[curr].next_nodes.len();
                for node in &route[curr].next_nodes {
                    node_pv[*node] += node_pv[curr] / num_next as f64;
                }
            });

            let mut utilization = 0.0;
            for i in 1..route.len() {
                let node = &route[i];

                utilization += node_pv[i]
                    * match node.node_type {
                        NodeType::Component => 1.0,
                        NodeType::VNF(_) => {
                            // Get parent server
                            let server_id = node.node_id / self.dc.vms_per_server; // Division of two integers gets rounded down

                            let min_vm_id = server_id * self.dc.vms_per_server;
                            let max_vm_id = (server_id + 1) * self.dc.vms_per_server;

                            // Count number of used VMs
                            solution[min_vm_id..max_vm_id]
                                .iter()
                                .filter(|&x| x.is_some())
                                .count() as f64
                        }
                    };
            }

            utilizations.push(utilization);
        }

        let avg_path_utilization = utilizations.iter().sum::<f64>() / utilizations.len() as f64;

        Constraint::Feasible(vec![avg_path_utilization, energy])
    }
}
