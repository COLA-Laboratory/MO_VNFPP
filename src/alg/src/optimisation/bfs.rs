use std::collections::VecDeque;

use rand::{prelude::SliceRandom, thread_rng};

use crate::optimisation::operators::Solution;
use crate::service::*;

pub fn run(
    services: &Vec<Service>,
    num_servers: usize,
    vms_per_server: usize,
    pop_size: usize,
) -> Vec<Solution<VNF>> {
    let min_length = services.iter().map(|s| s.vnfs.len()).sum::<usize>();

    let mut solutions = Vec::new();

    for i in 0..pop_size {
        let prop = i as f64 / pop_size as f64;

        let solution = bfs(services, prop, min_length, vms_per_server, num_servers);

        solutions.push(solution);
    }

    solutions
}

fn bfs(
    services: &Vec<Service>,
    prop: f64,
    min_length: usize,
    vms_per_server: usize,
    num_servers: usize,
) -> Solution<VNF> {
    let num_vms = vms_per_server * num_servers;

    // Get service instances
    let mut service_instances = Vec::new();
    for s_id in 0..services.len() {
        let num_instances = ((prop * num_vms as f64) / min_length as f64).ceil() as usize;

        for _ in 0..num_instances {
            service_instances.push(s_id);
        }
    }

    // Shuffle services so they are distributed about
    let mut rng = thread_rng();
    service_instances.shuffle(&mut rng);

    // Add all VNFs to a queue
    let mut queue = VecDeque::new();

    for s_id in service_instances {
        for vnf in &services[s_id].vnfs {
            queue.push_back(vnf.clone());
        }
    }

    // BFS Placement
    let mut i = 0;
    let mut solution = vec![None; num_vms];

    while !queue.is_empty() && i < num_vms {
        solution[i] = queue.pop_front();
        i = i + 1;
    }

    solution
}
