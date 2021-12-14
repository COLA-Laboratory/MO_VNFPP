use rand::prelude::*;
use std::collections::VecDeque;

use crate::optimisation::operators::Solution;
use crate::service::*;

pub fn run(
    nss: &Vec<Vec<usize>>,
    services: &Vec<Service>,
    num_servers: usize,
    vms_per_server: usize,
    pop_size: usize,
) -> Vec<Solution<VNF>> {
    let min_length = services.iter().map(|s| s.vnfs.len()).sum::<usize>();

    let mut solutions = Vec::new();

    for i in 0..pop_size {
        let solution = round_robin(&nss[i], services, vms_per_server, num_servers);
        solutions.push(solution);
    }

    solutions
}

fn round_robin(
    ns: &Vec<usize>,
    services: &Vec<Service>,
    vms_per_server: usize,
    num_servers: usize,
) -> Solution<VNF> {
    let num_vms = vms_per_server * num_servers;

    // Get service instances
    let mut service_instances = Vec::new();
    for s_id in 0..services.len() {
        let num_instances = ns[s_id];

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

    // Round robin placement
    let mut min = 0;
    let mut max = 2;

    let mut num_placed = 0;

    let mut server_ids: Vec<usize> = (0..num_servers).collect();

    let mut solution = vec![None; num_vms];

    while num_placed < num_vms && max <= vms_per_server {
        for i in &server_ids {
            for j in min..max {
                let pos = i * vms_per_server + j;

                solution[pos] = queue.pop_front();

                num_placed = num_placed + 1;
            }
        }

        min = max;
        max = max + 1;

        server_ids.reverse();
    }

    solution
}
