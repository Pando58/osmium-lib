use crate::{app::Osmium, core::NodeTrait, utils::AutoInc, OperationError};

#[derive(Debug)]
pub struct NodeExample {}

impl NodeExample {
    fn new() -> Self {
        Self {}
    }
}

impl NodeTrait for NodeExample {
    fn create(app: &Osmium, graph_id: usize) -> Result<usize, OperationError> {
        if let None = app.graphs.borrow().get(&graph_id) {
            return Err(OperationError::NonExistentItem);
        }

        Ok(app.nodes.borrow_mut().push(Box::new(NodeExample::new())))
    }
}
