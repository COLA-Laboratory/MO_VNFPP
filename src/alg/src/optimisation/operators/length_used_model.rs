use crate::fat_tree::FatTree;
use crate::optimisation::operators::constraint_aware::expand_solution;
use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;

pub struct LengthUsedEvaluate {
    dc: FatTree,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    do_balance: bool,
    do_cnstr_handling: bool,
}

impl LengthUsedEvaluate {
    pub fn new(
        fat_tree: &FatTree,
        services: &Vec<Service>,
        vnfs: &Vec<VNF>,
        do_balance: bool,
        do_cnstr_handling: bool,
    ) -> LengthUsedEvaluate {
        LengthUsedEvaluate {
            dc: fat_tree.clone(),
            services: services.clone(),
            vnfs: vnfs.clone(),
            do_balance,
            do_cnstr_handling,
        }
    }
}

impl Evaluate<Service> for LengthUsedEvaluate {
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

        // Approximate number of used servers
        let num_vms = solution.iter().filter(|vm| vm.is_some()).count();
        let perc_used = num_vms as f64 / solution.len() as f64;

        let sum_len = routes.iter().map(|(_, route)| route.len()).sum::<usize>();
        let avg_len: f64 = sum_len as f64 / routes.len() as f64;

        Constraint::Feasible(vec![perc_used, avg_len])
    }
}
