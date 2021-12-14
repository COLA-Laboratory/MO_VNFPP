use std::collections::VecDeque;

use rand::{prelude::SliceRandom, thread_rng, Rng};

use crate::optimisation::operators::Solution;
use crate::service::*;

// Joint Optimization of Chain Placement and Request Scheduling for Network Function Virtualization
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
        let solution = bfdsu(&nss[i], services, vms_per_server, num_servers);
        solutions.push(solution);
    }

    solutions
}

fn bfdsu(
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

    // BFDSU Placement
    let mut solution = vec![None; num_vms];

    let mut spare: Vec<usize> = (0..num_servers).collect();
    let mut used: Vec<usize> = Vec::new();
    let mut capacities = vec![vms_per_server; num_servers];

    while !queue.is_empty() {
        let vm = queue.pop_front();

        let mut available = Vec::new();

        // -- Identify candidate servers
        // First check if a used server can be used
        for server in &used {
            if capacities[*server] >= 1 {
                available.push(*server);
            }
        }

        // Otherwise, check the unused servers
        if available.is_empty() {
            for server in &spare {
                if capacities[*server] >= 1 {
                    available.push(*server);
                }
            }
        }

        // No spaces remaining
        if available.is_empty() {
            break;
        }

        // -- Choose a server with some probability
        // Sort ascending by capacity
        available.sort_by(|&x, &y| capacities[x].cmp(&capacities[y]));

        // Calculate the bounds
        let mut bounds = Vec::new();
        let sum: f64 = available.iter().map(|&s| 1.0 / capacities[s] as f64).sum();

        for server in &available {
            bounds.push((1.0 / capacities[*server] as f64) / sum);
        }

        // Choose a server based on the probabilities
        let mut rng = thread_rng();
        let rn: f64 = rng.gen(); // Gen. number between 0 and 1

        let mut i = 0;
        let mut sum = 0.0;
        loop {
            sum = sum + bounds[i];

            if rn < sum {
                break;
            }

            i = i + 1;
        }

        let s = available[i];

        // -- Update solution and tables
        // Place VM
        let vm_id = s * vms_per_server + (vms_per_server - capacities[s]);
        solution[vm_id] = vm;

        // Update tables
        capacities[s] = capacities[s] - 1;

        if let Some(i) = spare.iter().position(|&x| x == s) {
            spare.swap_remove(i);
            used.push(i);
        }
    }

    solution
}
