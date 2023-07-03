use crate::core::{Graph, Node};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug)]
pub struct Osmium {
    pub graphs: RefCell<HashMap<usize, Graph>>,
    pub nodes: RefCell<HashMap<usize, Node>>,
}

impl Osmium {
    pub fn new() -> Self {
        Self {
            graphs: RefCell::new(HashMap::new()),
            nodes: RefCell::new(HashMap::new()),
        }
    }
}
