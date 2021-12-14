use std::collections::{HashSet, VecDeque};

use rand::{prelude::SliceRandom, thread_rng};
use std::hash::Hash;

use crate::service::*;
use crate::{fat_tree::FatTree, optimisation::operators::Solution};

// ESP-VDCE: Energy, SLA and Price-Driven Virtual Data Center Embedding
pub fn run(
    nss: &Vec<Vec<usize>>,
    services: &Vec<Service>,
    ft: &FatTree,
    vms_per_server: usize,
    pop_size: usize,
) -> Vec<Solution<VNF>> {
    let min_length = services.iter().map(|s| s.vnfs.len()).sum::<usize>();

    let mut solutions = Vec::new();

    for i in 0..pop_size {
        let solution = esp_vdce(&nss[i], services, min_length, ft, vms_per_server);

        solutions.push(solution);
    }

    solutions
}

fn esp_vdce(
    ns: &Vec<usize>,
    services: &Vec<Service>,
    min_length: usize,
    ft: &FatTree,
    vms_per_server: usize,
) -> Solution<VNF> {
    let num_servers = ft.num_servers();
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

    // ESP-VDCE Placement
    let mut servers = vec![Vec::new(); num_servers];
    let mut all_used = vec![HashSet::new(); services.len()];

    while !queue.is_empty() {
        let vnf = queue.pop_front().unwrap();

        let used_srvs = &all_used[vnf.service_id].clone();
        if !used_srvs.is_empty() {
            if try_place_vnf(vnf, used_srvs, &mut all_used, &mut servers, vms_per_server) {
                continue;
            }
        }

        // If none of the already used servers are available, pick a new one
        let used_srvs = &all_used[vnf.service_id].clone();

        // 1. First consider the servers in the rack above each server
        let mut rack_srvs = HashSet::new();
        for server in used_srvs {
            let servers_per_rack = ft.num_ports / 2;

            let rack_min =
                (*server as f64 / servers_per_rack as f64).floor() as usize * servers_per_rack;
            let rack_max = rack_min + servers_per_rack;

            for i in rack_min..rack_max {
                rack_srvs.insert(i);
            }
        }

        if try_place_vnf(vnf, &rack_srvs, &mut all_used, &mut servers, vms_per_server) {
            continue;
        }

        // 2. Otherwise consider the servers in the pod of each server
        let mut pod_srvs = HashSet::new();
        for server in used_srvs {
            let servers_per_pod = (ft.num_ports / 2).pow(2);

            let pod_min =
                (*server as f64 / servers_per_pod as f64).floor() as usize * servers_per_pod;
            let pod_max = pod_min + servers_per_pod;

            for i in pod_min..pod_max {
                pod_srvs.insert(i);
            }
        }

        if try_place_vnf(vnf, &pod_srvs, &mut all_used, &mut servers, vms_per_server) {
            continue;
        }

        // 3. Otherwise choose from the remaining servers
        let mut all_srvs = (0..num_servers).collect();
        set_subtract(&mut all_srvs, &used_srvs);
        set_subtract(&mut all_srvs, &rack_srvs);
        set_subtract(&mut all_srvs, &pod_srvs);

        if try_place_vnf(vnf, &all_srvs, &mut all_used, &mut servers, vms_per_server) {
            continue;
        }
    }

    let mut solution = Vec::new();

    for server in servers {
        for i in 0..ft.vms_per_server {
            if i < server.len() {
                solution.push(Some(server[i]));
            } else {
                solution.push(None)
            }
        }
    }

    solution
}

fn set_subtract<X: Hash + Eq>(original_set: &mut HashSet<X>, subtraction_set: &HashSet<X>) {
    for item in subtraction_set {
        original_set.remove(&item);
    }
}

fn try_place_vnf(
    vnf: VNF,
    other_servers: &HashSet<usize>,
    used_servers: &mut Vec<HashSet<usize>>,
    servers: &mut Vec<Vec<VNF>>,
    vms_per_server: usize,
) -> bool {
    // Get servers and their utilizations
    let mut su_pair: Vec<(usize, usize)> = other_servers
        .iter()
        .map(|s: &usize| (*s, servers[*s].len()))
        .collect();

    // Sort descending in order of utilization
    su_pair.sort_unstable_by(|(_, u_a), (_, u_b)| u_a.cmp(u_b));

    for (server, utilization) in su_pair {
        if utilization < vms_per_server {
            // Place VNF on server
            used_servers[vnf.service_id].insert(server);
            servers[server].push(vnf);

            return true;
        }
    }

    false
}
