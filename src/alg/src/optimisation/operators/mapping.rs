use std::collections::{HashMap, VecDeque};

use crate::fat_tree::FatTree;
use crate::model::{NodeID, RouteNode};
use crate::service::{Service, VNF};

pub fn find_sequences(
    solution: &Vec<Option<VNF>>,
    services: &Vec<Service>,
) -> Vec<(usize, Vec<usize>)> {
    let mut routes = Vec::new();

    for i in 0..solution.len() {
        if solution[i].is_none() {
            continue;
        }

        let vnf = solution[i].unwrap();
        if vnf.stage != 0 {
            continue;
        }

        let service_id = vnf.service_id;
        let service_len = services[service_id].vnfs.len();

        let mut route = Vec::with_capacity(service_len);

        for j in 0..service_len {
            let possible_node = find_next_nearest(solution, i, service_id, j);

            if let Some(node) = possible_node {
                route.push(node);
            }
        }

        // If all succesfully placed
        if route.len() == service_len {
            routes.push((service_id, route));
        }
    }

    routes
}

fn find_next_nearest(
    solution: &Vec<Option<VNF>>,
    start: usize,
    service_id: usize,
    stage: usize,
) -> Option<usize> {
    for i in 0..solution.len() {
        for j in [-1, 1].iter() {
            let offset = i as i64 * j;
            let pos = start as i64 + offset;

            if pos < 0 || pos >= solution.len() as i64 {
                continue;
            }

            let pos = pos as usize;

            if solution[pos].is_none() {
                continue;
            }
            let vnf = solution[pos].unwrap();

            if vnf.service_id == service_id && vnf.stage == stage {
                return Some(pos);
            }
        }
    }

    None
}

pub fn find_routes(sequence: Vec<NodeID>, dc: &FatTree) -> Vec<RouteNode> {
    // Route graph
    let init_server_id = sequence[0];
    let init_node = RouteNode::new_vnf(init_server_id, 0);

    // Multistage graph
    let mut graph: Vec<RouteNode> = vec![init_node];

    // Lookup from DC ID and sequence stage to position in route graph
    // Lookup has to be indexed by stage as the same node can be visited
    // multiple times
    let mut lookup: HashMap<(NodeID, usize), usize> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((1, graph.len() - 1));

    // Find all the routes between the current node and the target
    while let Some((stage, curr)) = queue.pop_front() {
        let target = sequence[stage];

        // Gives the next set of nodes to visit
        let curr_dc_node = graph[curr].node_id;
        let next_dc_nodes = next_step(curr_dc_node, target, dc);

        for next_dc_node in next_dc_nodes {
            let lk_next = (next_dc_node, stage);

            if let Some(node_id) = lookup.get(&lk_next) {
                // We have already added this node to the graph
                // increment the route counter on it but don't
                // expand it again
                graph[*node_id].route_count += 1;

                // Point the parent to the right node
                graph[curr].next_nodes.push(*node_id);
            } else {
                // This is the first time we've seen this node
                let node_id = graph.len();

                let component = if next_dc_node == target {
                    if stage < sequence.len() - 1 {
                        queue.push_back((stage + 1, node_id));
                    }
                    RouteNode::new_vnf(next_dc_node, stage)
                } else {
                    queue.push_back((stage, node_id));
                    RouteNode::new_component(next_dc_node)
                };

                graph.push(component);

                lookup.insert(lk_next, node_id);
                graph[curr].next_nodes.push(node_id);
            }
        }
    }

    graph
}

pub fn next_step(node: usize, destination: usize, fat_tree: &FatTree) -> Vec<usize> {
    // Find the layer of the current node
    let mut layer = 0;

    let mut sum = 0;
    loop {
        let num_components = fat_tree.components_at_layer(layer);

        if sum + num_components > node {
            break;
        }

        layer = layer + 1;
        sum = sum + num_components;
    }

    // Find the next step using the layer

    match layer {
        0 => next_step_vm(node, destination, fat_tree),
        1 => next_step_server(node, destination, fat_tree),
        2 => next_step_edge(node, destination, fat_tree),
        3 => next_step_agg(node, destination, fat_tree),
        4 => next_step_core(node, destination, fat_tree),
        _ => panic!("This layer should not be reachable"),
    }
}

fn next_step_vm(node: usize, destination: usize, ft: &FatTree) -> Vec<usize> {
    if node == destination {
        return vec![];
    }
    // Go to parent
    let server_pos = node / ft.vms_per_server; // Division of two integers gets rounded down
    let offset = ft.num_vms();

    vec![server_pos + offset]
}

fn next_step_server(node: usize, destination: usize, ft: &FatTree) -> Vec<usize> {
    let server_id = node - ft.num_vms();

    let max_vm_id = (server_id + 1) * ft.vms_per_server;
    let min_vm_id = server_id * ft.vms_per_server;

    // If vm is on server
    if destination >= min_vm_id && destination < max_vm_id {
        vec![destination]
    } else {
        let offset = ft.num_vms() + ft.num_servers();
        let parent_edge = server_id / (ft.num_ports / 2);
        vec![parent_edge + offset]
    }
}

fn next_step_edge(node: usize, destination: usize, ft: &FatTree) -> Vec<usize> {
    let edge_id = node - ft.num_servers() - ft.num_vms();

    let half_k = ft.num_ports / 2;

    // An edge has k/2 servers under it
    let vms_under_edge = half_k * ft.vms_per_server;

    let max_vm_id = (edge_id + 1) * vms_under_edge;
    let min_vm_id = edge_id * vms_under_edge;

    if destination >= min_vm_id && destination < max_vm_id {
        let offset = ft.num_vms();
        vec![(destination / ft.vms_per_server) + offset]
    } else {
        let offset = ft.num_edges() + ft.num_servers() + ft.num_vms();
        let mut parents = Vec::with_capacity(half_k);

        let base_id = (edge_id / half_k) * half_k;

        for i in 0..half_k {
            parents.push(base_id + i + offset);
        }

        parents
    }
}

fn next_step_agg(node: usize, destination: usize, ft: &FatTree) -> Vec<usize> {
    let agg_id = node - ft.num_edges() - ft.num_servers() - ft.num_vms();

    let half_k = ft.num_ports / 2;

    // An agg has k/2 edge switches under it
    let pod_id = agg_id / half_k; // Implicit rounding in division
    let vms_under_agg = half_k.pow(2) * ft.vms_per_server;

    let max_vm_id = (pod_id + 1) * vms_under_agg;
    let min_vm_id = pod_id * vms_under_agg;

    if destination >= min_vm_id && destination < max_vm_id {
        let offset = ft.num_servers() + ft.num_vms();
        vec![(destination / (half_k * ft.vms_per_server)) + offset]
    } else {
        let offset = ft.num_agg() + ft.num_edges() + ft.num_servers() + ft.num_vms();

        let mut parents = Vec::with_capacity(half_k);
        let first_core = agg_id % half_k;

        for i in 0..half_k {
            parents.push(first_core + i * half_k + offset);
        }

        parents
    }
}

fn next_step_core(node: usize, destination: usize, ft: &FatTree) -> Vec<usize> {
    let core_id = node - ft.num_agg() - ft.num_edges() - ft.num_servers() - ft.num_vms();

    let half_k = ft.num_ports / 2;

    let pod_id = destination / (half_k.pow(2) * ft.vms_per_server);
    let offset = ft.num_edges() + ft.num_servers() + ft.num_vms();

    vec![(pod_id * half_k) + (core_id % half_k) + offset]
}
