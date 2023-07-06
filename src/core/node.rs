use crate::{app::Osmium, core::nodes::Nodes, OperationError};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Node {
    kind: Nodes,
    pub input_ids: Vec<usize>,
    pub output_ids: Vec<usize>,
}

impl Node {
    pub fn new(kind: Nodes) -> Self {
        Self {
            kind,
            input_ids: Vec::new(),
            output_ids: Vec::new(),
        }
    }

    pub fn create(app: &mut Osmium, kind: Nodes, graph_id: usize) -> Result<usize, OperationError> {
        if let None = app.graphs.get(&graph_id) {
            return Err(OperationError::NonExistentItem);
        }

        let node_id = kind.create(app)?;

        Ok(node_id)
    }
}
