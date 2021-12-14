pub mod binary;
pub mod constant_model;
pub mod constraint_aware;
pub mod length_used_model;
pub mod mapping;
pub mod mm1_model;
pub mod mm1k_model;
pub mod previous;
pub mod resources_model;
pub mod standard;

use rand::prelude::*;

use crate::fat_tree::FatTree;
use crate::model::{QueueingModel, RouteNode};
use crate::service::{Service, ServiceID, VNF};

pub type Solution<X> = Vec<Option<X>>;

pub trait InitPop<X> {
    fn apply(&self, pop_size: usize) -> Vec<Solution<X>>;
}

pub trait Evaluate<X> {
    fn get_number_objectives(&self) -> usize;
    fn apply(&mut self, solution: &Solution<X>) -> Constraint<Vec<f64>>;
}

pub trait Crossover<X> {
    fn apply(&self, parent_one: &Solution<X>, parent_two: &Solution<X>) -> Vec<Solution<X>>;
}

pub trait Mutation<X> {
    fn apply(&self, solution: &Solution<X>) -> Solution<X>;
}
pub struct TournamentSelection<T>
where
    T: Fn(usize, usize) -> bool,
{
    beats: T,
    pop_size: usize,
}

impl<T> TournamentSelection<T>
where
    T: Fn(usize, usize) -> bool,
{
    pub fn new(pop_size: usize, beats: T) -> Self {
        TournamentSelection { beats, pop_size }
    }

    pub fn tournament(&self, tournament_size: usize) -> usize {
        if tournament_size == 0 {
            panic!("Tournament size must be 1 or greater");
        }

        let mut rng = thread_rng();
        let mut used = Vec::new();

        let mut curr_best = rng.gen_range(0, self.pop_size);

        used.push(curr_best);

        for _ in 0..tournament_size - 1 {
            let mut contender;
            loop {
                contender = rng.gen_range(0, self.pop_size);

                if !used.contains(&contender) {
                    used.push(contender);
                    break;
                }
            }

            if (self.beats)(contender, curr_best) {
                curr_best = contender;
            }
        }

        curr_best
    }
}

#[derive(Clone)]
pub enum Constraint<X> {
    Feasible(X),
    Infeasible,
}

impl<X: Clone> Constraint<X> {
    pub fn is_feasible(&self) -> bool {
        match self {
            Constraint::Feasible(_) => true,
            Constraint::Infeasible => false,
        }
    }

    pub fn unwrap(&self) -> X {
        match self {
            Constraint::Feasible(x) => x.clone(),
            Constraint::Infeasible => panic!("Unwrap called on infeasible value"),
        }
    }
}

pub fn evaluate_solution(
    placements: &Solution<VNF>,
    routes: &Vec<(ServiceID, Vec<RouteNode>)>,
    vnfs: &Vec<VNF>,
    services: &Vec<Service>,
    queueing_model: &mut QueueingModel,
    lim_vnfs: &Vec<Option<usize>>,
    aa_services: &Vec<bool>,
) -> Constraint<Vec<f64>> {
    // Check if solution is feasible
    let dc = &queueing_model.dc;

    if !is_feasible(&placements, dc, &vnfs, aa_services, lim_vnfs) {
        return Constraint::Infeasible;
    }

    let (latencies, packet_losses, energy, _) =
        queueing_model.evaluate(services, placements, routes);

    let avg_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
    let avg_packet_loss = packet_losses.iter().sum::<f64>() / packet_losses.len() as f64;

    Constraint::Feasible(vec![avg_latency, avg_packet_loss, energy])
}

pub fn is_feasible(
    solution: &Solution<VNF>,
    dc: &FatTree,
    vnfs: &Vec<VNF>,
    aa_services: &Vec<bool>,
    lim_vnfs: &Vec<Option<usize>>,
) -> bool {
    if solution.len() > dc.num_vms() {
        return false;
    }

    let mut count_vnfs = vec![0; vnfs.len()];

    for s in 0..dc.num_servers() {
        // Check for anti-affinity VNFs
        let mut aa_service = None;

        for v in 0..dc.vms_per_server {
            let vm = solution[s * dc.vms_per_server + v];
            if vm.is_none() {
                continue;
            }

            let vnf = vm.unwrap();

            if aa_services[vnf.service_id] {
                aa_service = Some(vnf.service_id);
                break;
            }
        }

        for v in 0..dc.vms_per_server {
            let vm = solution[s * dc.vms_per_server + v];
            if vm.is_none() {
                continue;
            }

            let vnf = vm.unwrap();
            count_vnfs[vnf.id] += 1;

            if (lim_vnfs[vnf.id].is_some() && count_vnfs[vnf.id] > lim_vnfs[vnf.id].unwrap())
                || (aa_service.is_some() && vnf.service_id != aa_service.unwrap())
            {
                return false;
            }
        }
    }

    !count_vnfs.into_iter().any(|c| c == 0)
}
