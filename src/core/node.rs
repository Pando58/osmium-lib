use crate::{app::Osmium, core::nodes::NodeExample, OperationError};
use std::fmt::Debug;

pub trait NodeTrait: Debug {
    fn create(app: &Osmium, graph_id: usize) -> Result<usize, OperationError>
    where
        Self: Sized;
}

pub enum Node {
    Example,
}

pub fn create_node(app: &Osmium, graph_id: usize, class: Node) -> Result<usize, OperationError> {
    use Node::*;

    match class {
        Example => NodeExample::create(app, graph_id),
    }
}
