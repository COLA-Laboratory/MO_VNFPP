use rand::prelude::*;

use crate::optimisation::operators::*;

use super::standard::StdEvaluate;

// *** Binary Initialisation *** //
pub struct BinaryInitialisation {
    solution_length: usize,
}

impl BinaryInitialisation {
    pub fn new(solution_length: usize) -> BinaryInitialisation {
        BinaryInitialisation { solution_length }
    }
}

impl InitPop<()> for BinaryInitialisation {
    fn apply(&self, pop_size: usize) -> Vec<Solution<()>> {
        let mut rng = rand::thread_rng();
        let mut population = Vec::new();

        for _ in 0..pop_size {
            let mut new_solution = Vec::new();

            for _ in 0..self.solution_length {
                if rng.gen() {
                    new_solution.push(Some(()));
                } else {
                    new_solution.push(None);
                }
            }

            population.push(new_solution);
        }

        population
    }
}

// *** Binary Evaluation *** //
pub struct BinaryEvaluate<'a> {
    dc: &'a FatTree,
    vnfs: Vec<VNF>,
    std_evaluate: StdEvaluate,
}

impl<'a> BinaryEvaluate<'a> {
    pub fn new(
        dc: &FatTree,
        qm: QueueingModel,
        services: Vec<Service>,
        vnfs: Vec<VNF>,
        aa_services: Vec<bool>,
        vnf_limits: Vec<Option<usize>>,
    ) -> BinaryEvaluate {
        let std_evaluate = StdEvaluate::new(qm, services, vnfs.clone(), aa_services, vnf_limits);

        BinaryEvaluate {
            dc,
            vnfs,
            std_evaluate,
        }
    }
}

impl<'a> Evaluate<()> for BinaryEvaluate<'a> {
    fn get_number_objectives(&self) -> usize {
        3
    }

    fn apply(&mut self, solution: &Solution<()>) -> Constraint<Vec<f64>> {
        let solution = self.to_normal_solution(solution);
        self.std_evaluate.apply(&solution)
    }
}

impl<'a> BinaryEvaluate<'a> {
    fn to_normal_solution(&self, solution: &Solution<()>) -> Solution<VNF> {
        let mut num_assigned = vec![0; self.dc.num_servers()];
        let num_vnfs = self.vnfs.len();
        let vm_k = self.dc.vms_per_server;

        let mut normal_solution = vec![None; self.dc.num_vms()];

        for i in 0..solution.len() {
            let server_id = (i as f64 / num_vnfs as f64).floor() as usize;
            let server_asgnd = num_assigned[server_id];
            let vnf_id = i % num_vnfs;

            if solution[i].is_some() {
                if server_asgnd < vm_k {
                    let vm = self.vnfs[vnf_id];
                    let pos = server_id * vm_k + server_asgnd;

                    normal_solution[pos] = Some(vm);
                }

                num_assigned[server_id] += 1;
            }
        }

        normal_solution
    }
}
