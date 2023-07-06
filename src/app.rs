use crate::core::{Graph, Input, Node};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Osmium {
    pub graphs: HashMap<usize, Graph>,
    pub nodes: HashMap<usize, Node>,
    pub inputs: HashMap<usize, Input>,
}

impl Osmium {
    pub fn new() -> Self {
        Self {
            graphs: HashMap::new(),
            nodes: HashMap::new(),
            inputs: HashMap::new(),
        }
    }
}
