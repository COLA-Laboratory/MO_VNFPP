use crate::fat_tree::FatTree;
use crate::optimisation::operators::mapping::find_routes;
use crate::optimisation::operators::*;
use crate::service::*;
use rand::prelude::*;
use std::collections::VecDeque;

// *** Constraint Aware Initialisation *** //
pub struct CA_Initialisation {
    services: Vec<Service>,
    solution_length: usize,
}

impl CA_Initialisation {
    pub fn new(services: Vec<Service>, solution_length: usize) -> CA_Initialisation {
        CA_Initialisation {
            services,
            solution_length,
        }
    }
}

impl InitPop<Service> for CA_Initialisation {
    fn apply(&self, pop_size: usize) -> Vec<Solution<Service>> {
        let mut population = Vec::new();
        let mut rng = rand::thread_rng();

        let min_length = self.services.iter().map(|s| s.vnfs.len()).sum::<usize>();

        for i in 0..pop_size {
            let mut new_solution = Vec::new();
            let prop = i as f64 / pop_size as f64;

            // Add all service instances
            let mut num_placed = 0;

            let num_instances =
                ((prop * self.solution_length as f64) / min_length as f64).ceil() as usize;

            for service in &self.services {
                for _ in 0..num_instances {
                    new_solution.push(Some(service.clone()));
                    num_placed = num_placed + 1;
                }
            }

            // Fill in remaining spaces with None
            for _ in num_placed..self.solution_length {
                new_solution.push(None);
            }

            new_solution.shuffle(&mut rng);

            population.push(new_solution)
        }

        population
    }
}

// *** Constraint Aware Mutation *** //
pub struct CA_Mutation {
    services: Vec<Service>,
    pm: f64,
}

impl CA_Mutation {
    pub fn new(services: Vec<Service>, pm: f64) -> CA_Mutation {
        CA_Mutation { services, pm }
    }
}

impl Mutation<Service> for CA_Mutation {
    fn apply(&self, solution: &Solution<Service>) -> Solution<Service> {
        let mut new_solution = vec![None; solution.len()];
        let mut rng = rand::thread_rng();

        for i in 0..solution.len() {
            if self.pm < random() {
                new_solution[i] = solution[i].clone();
                continue;
            }

            if random() {
                // Swap
                let swap = thread_rng().gen_range(0, solution.len());

                new_solution[swap] = solution[i].clone();

                // Ensures the operator works the same as an in-place swap
                if swap < i {
                    new_solution[i] = new_solution[swap].clone();
                } else {
                    new_solution[i] = solution[swap].clone();
                }
            } else {
                // Add/Delete
                if solution[i].is_none() {
                    let idx = rng.gen_range(0, self.services.len());
                    let new_service = self.services[idx].clone();
                    new_solution[i] = Some(new_service);
                } else {
                    new_solution[i] = None;
                }
            }
        }

        new_solution
    }
}

// *** Intermediate Solution Expansion + Evaluation *** //
pub struct CA_Evaluate {
    qm: QueueingModel,
    services: Vec<Service>,
    vnfs: Vec<VNF>,
    pub aa_services: Vec<bool>,
    pub vnf_limits: Vec<Option<usize>>,
    pub do_balance: bool,
    pub do_cnstr_handling: bool,
}

impl CA_Evaluate {
    pub fn new(
        qm: QueueingModel,
        services: Vec<Service>,
        vnfs: Vec<VNF>,
        aa_services: Vec<bool>,
        vnf_limits: Vec<Option<usize>>,
        do_balance: bool,
        do_cnstr_handling: bool,
    ) -> CA_Evaluate {
        CA_Evaluate {
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

impl Evaluate<Service> for CA_Evaluate {
    fn get_number_objectives(&self) -> usize {
        3
    }

    fn apply(&mut self, int_solution: &Solution<Service>) -> Constraint<Vec<f64>> {
        let (sequences, solution) = expand_solution(
            &int_solution,
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

        evaluate_solution(
            &solution,
            &routes,
            &self.vnfs,
            &self.services,
            &mut self.qm,
            &self.vnf_limits,
            &self.aa_services,
        )
    }
}

pub fn expand_solution(
    intermediate_solution: &Vec<Option<Service>>,
    fat_tree: &FatTree,
    services: &Vec<Service>,
    aa_services: &Vec<bool>,
    max_instances: &Vec<Option<usize>>,
    do_balance: bool,
    do_cnstr_handling: bool,
) -> (Vec<(usize, Vec<usize>)>, Vec<Option<VNF>>) {
    let mut intermediate_solution = intermediate_solution.clone();

    // Balance
    if do_balance {
        balance(&mut intermediate_solution, fat_tree, services, aa_services);
    }

    let mut max_instances = max_instances.clone();
    let mut solution = vec![None; fat_tree.num_vms()];

    let mut aa_servers = vec![None; fat_tree.num_servers()];

    // First Feasible - Anti Affinity services
    let mut aa_sequences = first_feasible(
        &intermediate_solution,
        &mut solution,
        &mut aa_servers,
        &aa_services,
        &mut max_instances,
        fat_tree,
        true,
        do_cnstr_handling,
    );

    // First Feasible - Other services
    let mut ot_sequences = first_feasible(
        &intermediate_solution,
        &mut solution,
        &mut aa_servers,
        &aa_services,
        &mut max_instances,
        fat_tree,
        false,
        do_cnstr_handling
    );

    aa_sequences.append(&mut ot_sequences);

    (aa_sequences, solution)
}

fn balance(
    intermediate_solution: &mut Vec<Option<Service>>,
    ft: &FatTree,
    services: &Vec<Service>,
    aa_constraints: &Vec<bool>,
) {
    // Find used/free spaces
    let mut free_spaces = Vec::new();
    let mut placed = Vec::new();

    let mut count = vec![0; services.len()];

    for i in 0..intermediate_solution.len() {
        if intermediate_solution[i].is_none() {
            free_spaces.push(i);
        } else {
            let service = intermediate_solution[i].as_ref().unwrap();
            let service_id = service.id;

            count[service_id] = count[service_id] + 1;

            let pre = service.prod_rate / (service.first().service_rate * count[service_id] as f64);
            let post =
                service.prod_rate / (service.first().service_rate * (count[service_id] - 1) as f64);

            let contribution = pre - post;

            placed.push((i, service_id, contribution));
        }
    }

    let mut lengths = vec![0; services.len()];
    let mut missing = Vec::new();

    let mut used = 0;

    for i in 0..services.len() {
        if count[i] == 0 {
            missing.push(i);
        }

        let service_length = services[i].vnfs.len() as f64;

        if aa_constraints[i] {
            lengths[i] =
                (service_length / ft.vms_per_server as f64).ceil() as usize * ft.vms_per_server;
        } else {
            lengths[i] = service_length as usize;
        }

        used = used + lengths[i] * count[i];
    }

    while missing.len() > 0 && free_spaces.len() > 0 {
        let s_id = missing.pop().unwrap();
        let pos = free_spaces.pop().unwrap();

        intermediate_solution[pos] = Some(services[s_id].clone());
        used = used + lengths[s_id];
    }

    placed.sort_unstable_by(|(_, _, a_c), (_, _, b_c)| a_c.partial_cmp(&b_c).unwrap());

    while missing.len() > 0 || used > ft.num_vms() {
        if placed.len() == 0 {
            return; // Unable to balance solution
        }

        let (pos, s_p, contribution) = placed.pop().unwrap();

        if contribution == std::f64::INFINITY {
            return; // Unable to balance solution
        }

        intermediate_solution[pos] = None;
        used = used - lengths[s_p];

        if missing.len() > 0 {
            let s_m = missing.pop().unwrap();
            intermediate_solution[pos] = Some(services[s_m].clone());
            used = used + lengths[s_m];
        }
    }
}

fn first_feasible(
    intm_solution: &Vec<Option<Service>>,
    current_solution: &mut Vec<Option<VNF>>,
    aa_servers: &mut Vec<Option<usize>>,
    aa_services: &Vec<bool>,
    max_instances: &mut Vec<Option<usize>>,
    fat_tree: &FatTree,
    aa_only: bool,
    do_cnstr_handling: bool,
) -> Vec<(usize, Vec<usize>)> {
    let mut sequences = Vec::new();

    // No need for extra balance handling
    if aa_only && !do_cnstr_handling {
        return sequences;
    }

    for i in 0..fat_tree.num_vms() {
        let service = &intm_solution[i];

        if service.is_none() {
            continue;
        }

        let service = service.as_ref().unwrap();

        if (aa_services[service.id] != aa_only) && do_cnstr_handling {
            continue;
        }

        let pos = i / fat_tree.vms_per_server; // Automatically rounds down

        let maybe_sequence = place_service(
            service,
            pos,
            current_solution,
            aa_servers,
            aa_services,
            max_instances,
            &fat_tree,
        );

        if let Some(sequence) = maybe_sequence {
            sequences.push((service.id, sequence));
        }
    }

    sequences
}

fn place_service(
    service: &Service,
    start: usize,
    current_solution: &mut Vec<Option<VNF>>,
    aa_servers: &mut Vec<Option<usize>>,
    aa_services: &Vec<bool>,
    rem_instances: &mut Vec<Option<usize>>,
    fat_tree: &FatTree,
) -> Option<Vec<usize>> {
    let mut vnfs = VecDeque::new();

    for vnf in &service.vnfs {
        vnfs.push_back(vnf.clone());
    }

    let mut dir = 1;
    let mut curr = start;

    let mut sequence = Vec::new();

    loop {
        // Check server constraints
        let server = curr / fat_tree.vms_per_server;

        if current_solution[curr].is_none() // VM is free
               // Server is not AA/Server is AA but for the current service
            && none_or(&aa_servers[server], |s| *s == service.id)
        {
            let vnf = vnfs.pop_front().unwrap();

            if none_or(&rem_instances[vnf.id], |num| *num > 0) {
                // Decrement remaining instances
                rem_instances[vnf.id].map(|num| num - 1);

                current_solution[curr] = Some(vnf);
                sequence.push(curr);

                if aa_services[service.id] {
                    aa_servers[server] = Some(service.id);
                }
            }
        }

        if vnfs.is_empty() {
            return Some(sequence); // Placed all VNFs
        }

        if curr == 0 && dir == -1 {
            return None;
        }

        if curr == fat_tree.num_vms() - 1 {
            dir = -1;
        }

        curr = (curr as i64 + dir) as usize;
    }
}

pub fn none_or<X, F>(a: &Option<X>, f: F) -> bool
where
    F: Fn(&X) -> bool,
{
    a.is_none() || (a.is_some() && f(a.as_ref().unwrap()))
}
