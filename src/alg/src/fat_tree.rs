#[derive(Debug, Clone, Copy)]
pub struct FatTree {
    pub num_ports: usize,
    pub vms_per_server: usize,
}

impl FatTree {
    pub fn new(k: usize, vms_per_server: usize) -> FatTree {
        FatTree { num_ports: k, vms_per_server }
    }

    pub fn components_at_layer(self, layer: usize) -> usize {
        match layer {
            0 => self.num_vms(),
            1 => self.num_servers(),
            2 => self.num_edges(),
            3 => self.num_agg(),
            4 => self.num_core(),
            _ => panic!("There are only 5 layers in a Fat Tree network"),
        }
    }

    pub fn num_vms(&self) -> usize {
        self.num_servers() * self.vms_per_server
    }

    pub fn num_servers(&self) -> usize {
        self.num_ports.pow(3) / 4
    }

    pub fn num_edges(&self) -> usize {
        self.num_agg()
    }

    pub fn num_agg(&self) -> usize {
        self.num_ports * (self.num_ports / 2)
    }

    pub fn num_core(&self) -> usize {
        (self.num_ports / 2).pow(2)
    }

    pub fn num_all(&self) -> usize {
        self.num_vms() + self.num_servers() + self.num_edges() + self.num_agg() + self.num_core()
    }

    pub fn is_vnf(&self, node_id: usize) -> bool {
        node_id < self.num_vms()
    }

    pub fn is_server(&self, node_id: usize) -> bool {
        node_id < (self.num_vms() + self.num_servers()) && !self.is_vnf(node_id)
    }
}
