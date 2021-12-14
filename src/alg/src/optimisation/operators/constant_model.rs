use crate::optimisation::operators::constraint_aware::expand_solution;
use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;
use crate::{fat_tree::FatTree, model::iterate_route};

pub struct ConstantEvaluate {
    qm: QueueingModel,
    dc: FatTree,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    do_balance: bool,
    do_cnstr_handling: bool,
}

impl ConstantEvaluate {
    pub fn new(
        qm: QueueingModel,
        fat_tree: &FatTree,
        services: &Vec<Service>,
        vnfs: &Vec<VNF>,
        do_balance: bool,
        do_cnstr_handling: bool,
    ) -> ConstantEvaluate {
        ConstantEvaluate {
            qm,
            dc: fat_tree.clone(),
            services: services.clone(),
            vnfs: vnfs.clone(),
            do_balance,
            do_cnstr_handling,
        }
    }
}

impl Evaluate<Service> for ConstantEvaluate {
    fn get_number_objectives(&self) -> usize {
        3
    }

    fn apply(&mut self, solution: &Solution<Service>) -> Constraint<Vec<f64>> {
        let no_aa = vec![false; self.services.len()];
        let no_mi = vec![None; self.vnfs.len()];

        let (sequences, placements) = expand_solution(
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
        let (_, _, energy, _) = self.qm.evaluate(&self.services, &placements, &routes);

        // Constant waiting times + packet losses
        let mut latencies = Vec::with_capacity(self.services.len());
        let mut pls = Vec::with_capacity(self.services.len());

        let wt = 0.1;
        let pk = 0.995; // 0.5% packet drop rate at component

        for (_, route) in routes {
            let mut node_pk = vec![0.0; route.len()]; // Probability a packet survives to this node
            let mut node_pv = vec![0.0; route.len()]; // Probability of visiting this node
            node_pv[0] = 1.0;
            node_pk[0] = 1.0;

            iterate_route(&route, |curr| {
                node_pk[curr] = node_pk[curr] * (1.0 - pk);

                let num_next = route[curr].next_nodes.len();
                if num_next == 0 {
                    pls.push(1.0 - node_pk[curr]);
                }

                for node in &route[curr].next_nodes {
                    node_pk[*node] += node_pk[curr] / num_next as f64;
                    node_pv[*node] += node_pv[curr] / num_next as f64;
                }
            });

            let mut latency = 0.0;
            for i in 1..route.len() {
                latency = latency + wt * node_pv[i];
            }
            latencies.push(latency);
        }

        let latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let pl = pls.iter().sum::<f64>() / pls.len() as f64;

        Constraint::Feasible(vec![latency, pl, energy])
    }
}
