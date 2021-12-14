use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::mapping::find_sequences;
use rand::prelude::*;

use crate::optimisation::operators::*;
use crate::service::{Service, VNF};

// *** Uniform Initialisation *** //
pub struct UniformInitialisation<X: Clone> {
    characters: Vec<X>,
    solution_length: usize,
}

impl<X: Clone> UniformInitialisation<X> {
    pub fn new(characters: Vec<X>, solution_length: usize) -> UniformInitialisation<X> {
        UniformInitialisation {
            characters,
            solution_length,
        }
    }
}

impl<X: Clone> InitPop<X> for UniformInitialisation<X> {
    fn apply(&self, pop_size: usize) -> Vec<Solution<X>> {
        let mut rng = rand::thread_rng();
        let mut population = Vec::new();

        // Equal probability of the None character as a VNF character
        let p_none = 1.0 as f64 / (self.characters.len() + 1) as f64;

        for _ in 0..pop_size {
            let mut new_solution = Vec::new();

            for _ in 0..self.solution_length {
                if rng.gen::<f64>() < p_none {
                    new_solution.push(None);
                } else {
                    let idx = rng.gen_range(0, self.characters.len());
                    new_solution.push(Some(self.characters[idx].clone()));
                }
            }

            population.push(new_solution);
        }

        population
    }
}

// *** Uniform Mutation *** //
pub struct UniformMutation<X: Clone> {
    characters: Vec<X>,
    mr: f64,
}

impl<X: Clone> UniformMutation<X> {
    pub fn new(characters: Vec<X>, mr: f64) -> UniformMutation<X> {
        UniformMutation { characters, mr: mr }
    }
}

impl<X: Clone> Mutation<X> for UniformMutation<X> {
    fn apply(&self, solution: &Solution<X>) -> Solution<X> {
        let mut rng = rand::thread_rng();
        let mut new_solution = Vec::with_capacity(solution.len());

        let p_none = 1.0 as f64 / (self.characters.len() + 1) as f64;

        for character in solution {
            // Mutation rate
            if rng.gen::<f64>() < self.mr {
                // Place None character or otherwise
                if rng.gen::<f64>() < p_none {
                    new_solution.push(None);
                } else {
                    let idx = rng.gen_range(0, self.characters.len());
                    new_solution.push(Some(self.characters[idx].clone()));
                }
            } else {
                new_solution.push(character.clone());
            }
        }

        new_solution
    }
}

/*** Uniform Crossover ***/
pub struct UniformCrossover {
    pc: f64,
}

impl UniformCrossover {
    pub fn new(pc: f64) -> UniformCrossover {
        UniformCrossover { pc }
    }
}

impl<X: Clone> Crossover<X> for UniformCrossover {
    fn apply(&self, parent_one: &Solution<X>, parent_two: &Solution<X>) -> Vec<Solution<X>> {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() > self.pc {
            return vec![parent_one.clone(), parent_two.clone()];
        }

        let length = parent_one.len();

        let mut child_a = Vec::with_capacity(length);
        let mut child_b = Vec::with_capacity(length);

        for i in 0..length {
            if random() {
                child_a.push(parent_one[i].clone());
                child_b.push(parent_two[i].clone());
            } else {
                child_a.push(parent_two[i].clone());
                child_b.push(parent_one[i].clone());
            }
        }

        vec![child_a, child_b]
    }
}

/*** One-Point Crossover ***/
pub struct OnePointCrossover {
    pc: f64,
}

impl<X: Clone> Crossover<X> for OnePointCrossover {
    fn apply(&self, parent_one: &Solution<X>, parent_two: &Solution<X>) -> Vec<Solution<X>> {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() > self.pc {
            return vec![parent_one.clone(), parent_two.clone()];
        }

        let length = parent_one.len();

        let mut child_a = Vec::with_capacity(length);
        let mut child_b = Vec::with_capacity(length);

        let point = rng.gen_range(0, length);

        for i in 0..length {
            if i < point {
                child_a.push(parent_one[i].clone());
                child_b.push(parent_two[i].clone());
            } else {
                child_a.push(parent_two[i].clone());
                child_b.push(parent_one[i].clone());
            }
        }

        vec![child_a, child_b]
    }
}

// *** Standard Evaluation *** //
pub struct StdEvaluate {
    qm: QueueingModel,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    aa_services: Vec<bool>,
    vnf_limits: Vec<Option<usize>>,
}

impl<'a> StdEvaluate {
    pub fn new(
        qm: QueueingModel,
        services: Vec<Service>,
        vnfs: Vec<VNF>,
        aa_services: Vec<bool>,
        vnf_limits: Vec<Option<usize>>,
    ) -> StdEvaluate {
        StdEvaluate {
            qm,
            services,
            vnfs,
            aa_services,
            vnf_limits,
        }
    }
}

impl<'a> Evaluate<VNF> for StdEvaluate {
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

        evaluate_solution(
            solution,
            &routes,
            &self.vnfs,
            &self.services,
            &mut self.qm,
            &self.vnf_limits,
            &self.aa_services,
        )
    }
}
