use std::collections::VecDeque;

use crate::fat_tree::FatTree;
use crate::service::{Service, ServiceID, VNF};

pub type NodeID = usize;

#[derive(Debug, Clone)]
pub struct VnfMetrics {
    pub arrival_rate: f64,
    pub packet_losses: f64,
}

#[derive(Debug)]
pub enum NodeType {
    Component,
    VNF(usize), // VNF(position in service)
}

#[derive(Debug)]
pub struct RouteNode {
    pub node_type: NodeType,
    pub node_id: NodeID,
    pub route_count: u32,
    pub next_nodes: Vec<usize>,
}

impl RouteNode {
    pub fn new_component(node_id: NodeID) -> RouteNode {
        RouteNode {
            node_type: NodeType::Component,
            node_id,
            route_count: 1,
            next_nodes: Vec::new(),
        }
    }

    pub fn new_vnf(node_id: NodeID, stage: usize) -> RouteNode {
        RouteNode {
            node_id,
            node_type: NodeType::VNF(stage),
            route_count: 1,
            next_nodes: Vec::new(),
        }
    }
}

// As the model is run very frequently and the DBs are quite large
// we cache the memory to prevent lots of mallocs
#[derive(Debug, Clone)]
pub struct QueueingModel {
    pub dc: FatTree,
    pub arr_mid: Vec<f64>,
    arr_temp: Vec<f64>,
    pub pl: Vec<f64>,
    pub wt: Vec<f64>,
    sw_sr: f64,
    vnf_sr: Vec<f64>,
    sw_ql: usize,
    pub target_acc: f64,
    pub converged_iterations: usize,
    active_cost: f64,
    idle_cost: f64,
}

impl QueueingModel {
    pub fn new(
        dc: FatTree,
        sw_sr: f64,
        sw_ql: usize,
        accuracy: f64,
        converged_iterations: usize,
        active_cost: f64,
        idle_cost: f64,
    ) -> QueueingModel {
        QueueingModel {
            dc,
            arr_mid: vec![0.0; dc.num_all()],
            arr_temp: vec![0.0; dc.num_all()],
            pl: vec![0.0; dc.num_all()],
            wt: vec![0.0; dc.num_all()],
            sw_sr,
            vnf_sr: vec![0.0; dc.num_vms()],
            sw_ql,
            target_acc: accuracy,
            converged_iterations,
            active_cost,
            idle_cost,
        }
    }

    pub fn with_dc(&self, dc: FatTree, port_service_rate: f64) -> QueueingModel {
        let mut new = self.clone();
        new.dc = dc;
        new.arr_mid = vec![0.0; dc.num_all()];
        new.arr_temp = vec![0.0; dc.num_all()];
        new.pl = vec![0.0; dc.num_all()];
        new.vnf_sr = vec![0.0; dc.num_vms()];
        new.sw_sr = port_service_rate * dc.num_ports as f64;

        new
    }

    pub fn evaluate(
        &mut self,
        services: &Vec<Service>,
        placements: &Vec<Option<VNF>>,
        routes: &Vec<(ServiceID, Vec<RouteNode>)>,
    ) -> (Vec<f64>, Vec<f64>, f64, usize) {
        let mut num_iterations = 0;
        let mut num_below = 0;
        let mut max_diff: f64;

        // Reset memory
        for i in 0..self.arr_temp.len() {
            self.arr_temp[i] = 0.0;
        }

        for i in 0..self.arr_mid.len() {
            self.arr_mid[i] = 0.0;
        }

        for i in 0..self.pl.len() {
            self.pl[i] = 0.0;
        }

        let mut prev = vec![0.0; self.arr_temp.len()];
        let mut curr = vec![0.0; self.arr_temp.len()];

        set_all_arrival_rates(&routes, &services, &mut curr, &self.pl);

        while num_below < self.converged_iterations {
            // Calculate packet loss for all components (including VNFs)
            set_all_pl(
                placements,
                &mut self.pl,
                &curr,
                self.sw_sr,
                self.sw_ql,
                self.dc.num_vms(),
            );

            for i in 0..curr.len() {
                prev[i] = curr[i];
            }

            // Add arrival rates
            set_all_arrival_rates(&routes, &services, &mut curr, &self.pl);

            // Cumulative moving average of arrival rates
            max_diff = 0.0;
            for i in 0..self.arr_temp.len() {
                let mid = (curr[i] + prev[i]) / 2.0;
                let diff = (self.arr_mid[i] - mid).abs();

                self.arr_mid[i] = mid;

                max_diff = max_diff.max(diff);
            }

            if max_diff < self.target_acc {
                num_below = num_below + 1;
            } else {
                num_below = 0;
            }

            num_iterations = num_iterations + 1;
        }

        // Recalculate PL using average arrival rate
        set_all_pl(
            placements,
            &mut self.pl,
            &self.arr_mid,
            self.sw_sr,
            self.sw_ql,
            self.dc.num_vms(),
        );

        set_all_wt(
            placements,
            &mut self.wt,
            &self.pl,
            &self.arr_mid,
            self.sw_sr,
            self.sw_ql,
            self.dc.num_vms(),
        );

        // Calculate service latency + pl
        let mut service_latency = vec![0.0; services.len()];
        let mut service_pl = vec![0.0; services.len()];

        let mut s_count = vec![0; services.len()];

        for (s_id, route) in routes {
            let mut node_pk = vec![0.0; route.len()]; // Probability a packet survives to this node
            let mut node_pl = vec![0.0; route.len()]; // Packet loss at this node
            let mut node_pv = vec![0.0; route.len()]; // Probability of visiting this node
            node_pv[0] = 1.0;
            node_pk[0] = 1.0;

            iterate_route(route, |curr| {
                let cn = &route[curr];
                node_pl[curr] = self.pl[cn.node_id];
                node_pk[curr] = node_pk[curr] * (1.0 - node_pl[curr]);

                let num_next = route[curr].next_nodes.len();
                if num_next == 0 {
                    service_pl[*s_id] =
                        calc_ma(service_pl[*s_id], 1.0 - node_pk[curr], s_count[*s_id]).0;
                }

                for node in &route[curr].next_nodes {
                    node_pk[*node] += node_pk[curr] / num_next as f64;
                    node_pv[*node] += node_pv[curr] / num_next as f64;
                }
            });

            let mut latency = 0.0;

            for i in 1..route.len() {
                let rn = &route[i];
                latency = latency + (self.wt[rn.node_id]) * node_pv[i];
            }

            service_latency[*s_id] = calc_ma(service_latency[*s_id], latency, s_count[*s_id]).0;
            s_count[*s_id] += 1;
        }

        // Calculate energy consumption
        let mut sum_energy = 0.0;
        for i in 0..self.dc.num_all() {
            let utilisation;

            if self.dc.is_vnf(i) {
                continue;
            }

            if self.dc.is_server(i) {
                let server_busy = calc_busy(self.arr_mid[i], self.sw_sr, self.sw_ql);
                let mut p_none_busy = 1.0;

                let vm_l = (i - self.dc.num_vms()) * self.dc.vms_per_server;
                let vm_u = vm_l + self.dc.vms_per_server;

                for vm_pos in vm_l..vm_u {
                    if let Some(vnf) = placements[vm_pos] {
                        let vm_not_busy = 1.0
                            - calc_busy(self.arr_mid[vm_pos], vnf.service_rate, vnf.queue_length);
                        p_none_busy = p_none_busy * vm_not_busy;
                    }
                }

                utilisation = 1.0 - ((1.0 - server_busy) * p_none_busy)
            } else {
                utilisation = calc_busy(self.arr_mid[i], self.sw_sr, self.sw_ql)
            };

            if utilisation == 0.0 {
                continue;
            }

            sum_energy += (self.active_cost * utilisation) + (self.idle_cost * (1.0 - utilisation));
        }

        (service_latency, service_pl, sum_energy, num_iterations)
    }

    pub fn evaluate_once(
        &mut self,
        services: &Vec<Service>,
        placements: &Vec<Option<VNF>>,
        routes: &Vec<(ServiceID, Vec<RouteNode>)>,
    ) -> (Vec<f64>, Vec<f64>, f64, usize) {
        // Reset memory
        for i in 0..self.arr_mid.len() {
            self.arr_mid[i] = 0.0;
        }

        for i in 0..self.pl.len() {
            self.pl[i] = 0.0;
        }

        set_all_arrival_rates(&routes, &services, &mut self.arr_mid, &self.pl);
        
        set_all_pl(
            placements,
            &mut self.pl,
            &self.arr_mid,
            self.sw_sr,
            self.sw_ql,
            self.dc.num_vms(),
        );

        set_all_wt(
            placements,
            &mut self.wt,
            &self.pl,
            &self.arr_mid,
            self.sw_sr,
            self.sw_ql,
            self.dc.num_vms(),
        );

        // Calculate service latency + pl
        let mut service_latency = vec![0.0; services.len()];
        let mut service_pl = vec![0.0; services.len()];

        let mut s_count = vec![0; services.len()];

        for (s_id, route) in routes {
            let mut node_pk = vec![0.0; route.len()]; // Probability a packet survives to this node
            let mut node_pl = vec![0.0; route.len()]; // Packet loss at this node
            let mut node_pv = vec![0.0; route.len()]; // Probability of visiting this node
            node_pv[0] = 1.0;
            node_pk[0] = 1.0;

            iterate_route(route, |curr| {
                let cn = &route[curr];
                node_pl[curr] = self.pl[cn.node_id];
                node_pk[curr] = node_pk[curr] * (1.0 - node_pl[curr]);

                let num_next = route[curr].next_nodes.len();
                if num_next == 0 {
                    service_pl[*s_id] =
                        calc_ma(service_pl[*s_id], 1.0 - node_pk[curr], s_count[*s_id]).0;
                }

                for node in &route[curr].next_nodes {
                    node_pk[*node] += node_pk[curr] / num_next as f64;
                    node_pv[*node] += node_pv[curr] / num_next as f64;
                }
            });

            let mut latency = 0.0;

            for i in 1..route.len() {
                let rn = &route[i];
                latency = latency + (self.wt[rn.node_id]) * node_pv[i];
            }

            service_latency[*s_id] = calc_ma(service_latency[*s_id], latency, s_count[*s_id]).0;
            s_count[*s_id] += 1;
        }

        // Calculate energy consumption
        let mut sum_energy = 0.0;
        for i in 0..self.dc.num_all() {
            let utilisation;

            if self.dc.is_vnf(i) {
                continue;
            }

            if self.dc.is_server(i) {
                let server_busy = calc_busy(self.arr_mid[i], self.sw_sr, self.sw_ql);
                let mut p_none_busy = 1.0;

                let vm_l = (i - self.dc.num_vms()) * self.dc.vms_per_server;
                let vm_u = vm_l + self.dc.vms_per_server;

                for vm_pos in vm_l..vm_u {
                    if let Some(vnf) = placements[vm_pos] {
                        let vm_not_busy = 1.0
                            - calc_busy(self.arr_mid[vm_pos], vnf.service_rate, vnf.queue_length);
                        p_none_busy = p_none_busy * vm_not_busy;
                    }
                }

                utilisation = 1.0 - ((1.0 - server_busy) * p_none_busy)
            } else {
                utilisation = calc_busy(self.arr_mid[i], self.sw_sr, self.sw_ql)
            };

            if utilisation == 0.0 {
                continue;
            }

            sum_energy += (self.active_cost * utilisation) + (self.idle_cost * (1.0 - utilisation));
        }

        (service_latency, service_pl, sum_energy, 1)


    }

    pub fn evaluate_mm1(
        &mut self,
        services: &Vec<Service>,
        placements: &Vec<Option<VNF>>,
        routes: &Vec<(ServiceID, Vec<RouteNode>)>,
    ) -> (Vec<f64>, Vec<f64>, f64, usize) {
        // Reset memory
        for i in 0..self.arr_mid.len() {
            self.arr_mid[i] = 0.0;
        }

        for i in 0..self.pl.len() {
            self.pl[i] = 0.0;
        }

        // Calculate sum arrival rates
        set_all_arrival_rates(&routes, &services, &mut self.arr_mid, &self.pl);

        // Calculate service latency
        let mut service_latency = vec![0.0; services.len()];
        let mut s_count = vec![0; services.len()];

        for (s_id, route) in routes {
            let mut node_pv = vec![0.0; route.len()]; // Probability of visiting this node
            node_pv[0] = 1.0;

            iterate_route(route, |curr| {
                let num_next = route[curr].next_nodes.len();
                for node in &route[curr].next_nodes {
                    node_pv[*node] += node_pv[curr] / num_next as f64;
                }
            });

            let mut latency = 0.0;
            for i in 1..route.len() {
                let rn = &route[i];
                let arr = self.arr_mid[rn.node_id];

                let (srv, _) = match rn.node_type {
                    NodeType::Component => (self.sw_sr, self.sw_ql),
                    NodeType::VNF(stage) => {
                        let vnf = &services[*s_id].vnfs[stage];
                        (vnf.service_rate, vnf.queue_length)
                    }
                };

                let wt = if arr > srv {
                    std::f64::INFINITY
                } else if arr == 0.0 {
                    0.0
                } else {
                    1.0 / (srv - arr)
                };

                latency = latency + (wt * node_pv[i]);
            }

            service_latency[*s_id] = calc_ma(service_latency[*s_id], latency, s_count[*s_id]).0;
            s_count[*s_id] += 1;
        }

        // Calculate energy consumption
        let mut sum_energy = 0.0;
        for i in 0..self.dc.num_all() {
            let mut utilisation = 0.0;

            if self.dc.is_vnf(i) {
                continue;
            }

            if self.dc.is_server(i) {
                let server_busy = self.arr_mid[i] / self.sw_sr;

                if server_busy > 1.0 {
                    utilisation = 1.0;
                }

                let mut p_none_busy = 1.0;

                let vm_l = (i - self.dc.num_vms()) * self.dc.vms_per_server;
                let vm_u = vm_l + self.dc.vms_per_server;

                for vm_pos in vm_l..vm_u {
                    if let Some(vnf) = placements[vm_pos] {
                        if self.arr_mid[vm_pos] > vnf.service_rate
                        {
                            utilisation = 1.0;
                            break;
                        }

                        let vm_not_busy = 1.0 - self.arr_mid[vm_pos] / vnf.service_rate;
                        p_none_busy = p_none_busy * vm_not_busy;
                    }
                }

                if utilisation != 1.0 {
                    utilisation = 1.0 - ((1.0 - server_busy) * p_none_busy)
                }
            } else {
                let server_busy = self.arr_mid[i] / self.sw_sr;
                if server_busy > 1.0 {
                    utilisation = 1.0;
                } else {
                    utilisation = server_busy;
                }
            };

            if utilisation == 0.0 {
                continue;
            }

            sum_energy += (self.active_cost * utilisation) + (self.idle_cost * (1.0 - utilisation));
        }

        (service_latency, vec![0.0; services.len()], sum_energy, 0)
    }
}

fn calc_ma(current_mean: f64, new_value: f64, num_points: usize) -> (f64, f64) {
    let new = current_mean + (new_value - current_mean) / (num_points + 1) as f64;
    (new, (new - current_mean).abs())
}

fn set_all_arrival_rates(
    solution: &Vec<(ServiceID, Vec<RouteNode>)>,
    services: &Vec<Service>,
    sw_arr: &mut Vec<f64>,
    sw_pl: &Vec<f64>,
) {
    // Reset memory
    for i in 0..sw_arr.len() {
        sw_arr[i] = 0.0;
    }

    let mut num_instances = vec![0; services.len()];
    for (s_id, _) in solution {
        num_instances[*s_id] += 1;
    }

    for (s_id, route) in solution {
        let mut arrs = vec![0.0; route.len()];
        arrs[0] = services[*s_id].prod_rate / num_instances[*s_id] as f64;

        iterate_route(route, |curr| {
            let cn = &route[curr];

            // First VNF doesn't have any incoming packets
            if let NodeType::VNF(pos) = cn.node_type {
                if pos != 0 {
                    sw_arr[cn.node_id] += arrs[curr];
                }
            } else {
                sw_arr[cn.node_id] += arrs[curr];
            }

            let pl = sw_pl[cn.node_id];
            let eff_out = arrs[curr] * (1.0 - pl);
            let distr_out = eff_out / cn.next_nodes.len() as f64;

            for n_id in &cn.next_nodes {
                arrs[*n_id] = arrs[*n_id] + distr_out;
            }
        });
    }

    for i in 0..sw_arr.len() {
        sw_arr[i] = round_to(sw_arr[i], 3);
    }
}

fn set_all_pl(
    placements: &Vec<Option<VNF>>,
    pl: &mut Vec<f64>,
    arr: &Vec<f64>,
    sw_srv_rate: f64,
    sw_queue_length: usize,
    num_vms: usize,
) {
    for i in 0..pl.len() {
        let mut srv = sw_srv_rate;
        let mut ql = sw_queue_length;

        // VNFs have different service rates/queue lengths
        if i < num_vms {
            let vm = &placements[i];
            if let Some(vnf) = vm {
                if vnf.stage == 0 {
                    pl[i] = 0.0; // Initial VNFs can't drop packets
                }

                srv = vnf.service_rate;
                ql = vnf.queue_length;
            }
        }

        pl[i] = calc_pl(arr[i], srv, ql);
    }
}

fn set_all_wt(
    placements: &Vec<Option<VNF>>,
    wt: &mut Vec<f64>,
    pl: &Vec<f64>,
    arr: &Vec<f64>,
    sw_srv_rate: f64,
    sw_queue_length: usize,
    num_vms: usize,
) {
    for i in 0..wt.len() {
        let mut srv = sw_srv_rate;
        let mut ql = sw_queue_length;

        // VNFs have different service rates/queue lengths
        if i < num_vms {
            let vm = &placements[i];
            if let Some(vnf) = vm {
                srv = vnf.service_rate;
                ql = vnf.queue_length;
            }
        }

        wt[i] = calc_wt(arr[i], srv, ql, pl[i]);
    }
}

fn calc_pl(arrival_rate: f64, service_rate: f64, queue_length: usize) -> f64 {
    let queue_length = queue_length as f64;
    let rho = arrival_rate / service_rate;

    if rho == 1.0 {
        1.0 / (queue_length + 1.0)
    } else {
        ((1.0 - rho) * rho.powf(queue_length)) / (1.0 - rho.powf(queue_length + 1.0))
    }
}

pub fn calc_wt(arrival_rate: f64, service_rate: f64, queue_length: usize, packet_loss: f64) -> f64 {
    let queue_length = queue_length as f64;

    let rho = arrival_rate / service_rate;

    if arrival_rate == 0. {
        return 0.;
    }

    let num_in_system = if rho != 1.0 {
        let a = rho
            * (1.0 - (queue_length + 1.0) * rho.powf(queue_length)
                + queue_length * rho.powf(queue_length + 1.0));
        let b = (1.0 - rho) * (1.0 - rho.powf(queue_length + 1.0));

        a / b
    } else {
        queue_length / 2.0
    };

    let ar = arrival_rate * (1.0 - packet_loss);

    num_in_system / ar
}

fn calc_busy(arrival_rate: f64, service_rate: f64, queue_length: usize) -> f64 {
    if arrival_rate > 0.0 && service_rate == 0.0 {
        return std::f64::INFINITY;
    }

    let rho = arrival_rate / service_rate;
    let k = queue_length as f64;

    let p_empty = if arrival_rate != service_rate {
        (1.0 - rho) / (1.0 - rho.powf(k + 1.0))
    } else {
        1.0 / (k + 1.0)
    };

    1.0 - p_empty
}

pub fn iterate_route(route: &Vec<RouteNode>, mut apply: impl FnMut(usize)) {
    let mut num_routes: Vec<u32> = route.iter().map(|x| x.route_count).collect();
    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(curr) = queue.pop_front() {
        num_routes[curr] = num_routes[curr] - 1;

        if num_routes[curr] == 0 {
            apply(curr);

            for n in &route[curr].next_nodes {
                queue.push_back(*n);
            }
        }
    }
}

fn round_to(number: f64, dp: usize) -> f64 {
    let ten: f64 = 10.0;
    let shift: f64 = ten.powf(dp as f64);
    (number * shift).round() / shift
}
