use std::collections::HashMap;

use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;

use super::mapping::find_sequences;

fn get_counts(services: &Vec<Service>, num_vms: usize) -> (HashMap<&VNF, f64>, usize) {
    // Find target and maximum
    let mut vnf_counts = HashMap::new();

    for service in services {
        let mut arr_rate = service.prod_rate;

        for vnf in &service.vnfs {
            let num_copies = f64::ceil(arr_rate / vnf.service_rate);

            if vnf_counts.contains_key(vnf) {
                *vnf_counts.get_mut(vnf).unwrap() += num_copies;
            } else {
                vnf_counts.insert(vnf, num_copies);
            }

            arr_rate = arr_rate;
        }
    }
    let sum_counts = vnf_counts.values().sum::<f64>();
    let max_copies = f64::ceil(num_vms as f64 / sum_counts) as usize;

    (vnf_counts, max_copies)
}

// *** Previous Initialisation *** //
pub struct PreviousInitialisation {
    services: Vec<Service>,
    solution_length: usize,
}

impl PreviousInitialisation {
    pub fn new(services: Vec<Service>, solution_length: usize) -> PreviousInitialisation {
        PreviousInitialisation {
            services,
            solution_length,
        }
    }
}

impl InitPop<VNF> for PreviousInitialisation {
    fn apply(&self, pop_size: usize) -> Vec<Solution<VNF>> {
        let mut population = Vec::new();

        let (vnf_counts, max_copies) = get_counts(&self.services, self.solution_length);

        for _ in 0..pop_size {
            // Roulette wheel allocation
            let mut rng = thread_rng();
            let mult = rng.gen_range(1, max_copies + 1);

            let mut roulette: Vec<&VNF> = vnf_counts
                .iter()
                .flat_map(|(&key, &val)| vec![key; (val as usize) * mult])
                .collect();

            let mut end = roulette.len();

            let mut vms = vec![None; self.solution_length];
            let mut alloc_i = 0;

            while end != 0 && alloc_i < vms.len() {
                let idx = rng.gen_range(0, end);

                vms[alloc_i] = Some(roulette[idx].clone());
                roulette[idx] = roulette[end - 1];

                end = end - 1;
                alloc_i = alloc_i + 1;
            }

            population.push(vms);
        }

        population
    }
}

// *** Previous Mutation *** //
pub struct PreviousMutation {
    services: Vec<Service>,
    p_mut: f64,
}

impl PreviousMutation {
    pub fn new(services: Vec<Service>, p_mut: f64) -> PreviousMutation {
        PreviousMutation { services, p_mut }
    }
}

impl Mutation<VNF> for PreviousMutation {
    fn apply(&self, solution: &Solution<VNF>) -> Solution<VNF> {
        let mut rng = rand::thread_rng();
        let mut new_solution = solution.clone();

        let (vnf_counts, _) = get_counts(&self.services, solution.len());

        let roulette: Vec<&VNF> = vnf_counts
            .into_iter()
            .flat_map(|(key, val)| vec![key; val as usize])
            .collect();

        for i in 0..solution.len() {
            let rand: f64 = random();

            if rand < self.p_mut {
                let vnf_id = rng.gen_range(0, solution.len() + 1);

                if vnf_id < roulette.len() {
                    new_solution[i] = Some(roulette[vnf_id].clone());
                } else {
                    new_solution[i] = None;
                }
            }
        }

        new_solution
    }
}

pub struct PreviousEvaluate {
    qm: QueueingModel,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    aa_services: Vec<bool>,
    vnf_limits: Vec<Option<usize>>,
}

impl<'a> PreviousEvaluate {
    pub fn new(
        qm: QueueingModel,
        services: Vec<Service>,
        vnfs: Vec<VNF>,
        aa_services: Vec<bool>,
        vnf_limits: Vec<Option<usize>>,
    ) -> PreviousEvaluate {
        PreviousEvaluate {
            qm,
            services,
            vnfs,
            aa_services,
            vnf_limits,
        }
    }
}

impl Evaluate<VNF> for PreviousEvaluate {
    fn get_number_objectives(&self) -> usize {
        3
    }

    fn apply(&mut self, solution: &Solution<VNF>) -> Constraint<Vec<f64>> {
        let sequences = find_sequences(solution, &self.services);

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
            self.qm.evaluate_mm1(&self.services, &solution, &routes);

        let avg_latency = latencies
            .iter()
            .filter(|&&f| !f.is_nan() && f.is_finite())
            .sum::<f64>()
            / latencies.len() as f64;
        let avg_packet_loss = packet_losses.iter().sum::<f64>() / packet_losses.len() as f64;

        Constraint::Feasible(vec![avg_latency, avg_packet_loss, energy])
    }
}
