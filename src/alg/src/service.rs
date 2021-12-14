use std::hash::{Hash, Hasher};

pub type ServiceID = usize;

#[derive(Debug, Clone)]
pub struct Service {
    pub id: ServiceID,
    pub prod_rate: f64,
    pub vnfs: Vec<VNF>,
}

impl Service {
    pub fn first(&self) -> &VNF {
        self.vnfs.first().unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VNF {
    pub id: usize,
    pub stage: usize,
    pub service_rate: f64,
    pub queue_length: usize,
    pub service_id: usize,
}

impl PartialEq for VNF {
    fn eq(&self, other: &VNF) -> bool {
        self.id == other.id
    }
}
impl Eq for VNF {}

impl Hash for VNF {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
