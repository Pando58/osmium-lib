use std::{cell::RefCell, collections::HashMap};

use crate::core::{Graph, NodeTrait};

#[derive(Debug)]
pub struct Osmium {
    pub graphs: RefCell<HashMap<usize, Graph>>,
    pub nodes: RefCell<HashMap<usize, Box<dyn NodeTrait>>>,
}

impl Osmium {
    pub fn new() -> Self {
        Self {
            graphs: RefCell::new(HashMap::new()),
            nodes: RefCell::new(HashMap::new()),
        }
    }
}
