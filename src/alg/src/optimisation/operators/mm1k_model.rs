use crate::optimisation::operators::constraint_aware::expand_solution;
use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;

pub struct MM1KEvaluate {
    qm: QueueingModel,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    aa_services: Vec<bool>,
    vnf_limits: Vec<Option<usize>>,
    do_balance: bool,
    do_cnstr_handling: bool,
}

impl MM1KEvaluate {
    pub fn new(
        qm: QueueingModel,
        services: Vec<Service>,
        vnfs: Vec<VNF>,
        aa_services: Vec<bool>,
        vnf_limits: Vec<Option<usize>>,
        do_balance: bool,
        do_cnstr_handling: bool,
    ) -> MM1KEvaluate {
        MM1KEvaluate {
            qm,
            services,
            vnfs,
            aa_services,
            vnf_limits,
            do_balance,
            do_cnstr_handling,
        }
    }
}

impl Evaluate<Service> for MM1KEvaluate {
    fn get_number_objectives(&self) -> usize {
        3
    }

    fn apply(&mut self, solution: &Solution<Service>) -> Constraint<Vec<f64>> {
        let (sequences, solution) = expand_solution(
            &solution,
            &self.qm.dc,
            &self.services,
            &self.aa_services,
            &self.vnf_limits,
            self.do_balance,
            self.do_cnstr_handling,
        );

        let mut routes = Vec::new();
        for (s_id, sequence) in sequences {
            let route = find_routes(sequence, &self.qm.dc);
            routes.push((s_id, route));
        }

        let dc = &self.qm.dc;

        if !is_feasible(
            &solution,
            dc,
            &self.vnfs,
            &self.aa_services,
            &self.vnf_limits,
        ) {
            return Constraint::Infeasible;
        }

        let (latencies, packet_losses, energy, _) =
            self.qm.evaluate_once(&self.services, &solution, &routes);

        let avg_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let avg_packet_loss = packet_losses.iter().sum::<f64>() / packet_losses.len() as f64;

        Constraint::Feasible(vec![avg_latency, avg_packet_loss, energy])
    }
}
