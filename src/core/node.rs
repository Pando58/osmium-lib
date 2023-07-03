use crate::{app::Osmium, utils::AutoInc, OperationError};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Node {
    input_ids: Vec<usize>,
    output_ids: Vec<usize>,
    class_manager: Box<dyn NodeClassManager>,
}

impl Node {
    fn new<T: NodeClassManager + 'static>() -> Self {
        Self {
            input_ids: Vec::new(),
            output_ids: Vec::new(),
            class_manager: Box::new(T::new()),
        }
    }

    pub fn create<T: NodeClassManager + 'static>(
        app: &Osmium,
        graph_id: usize,
    ) -> Result<usize, OperationError> {
        if let None = app.graphs.borrow().get(&graph_id) {
            return Err(OperationError::NonExistentItem);
        }

        Ok(app.nodes.borrow_mut().push(Node::new::<T>()))
    }
}

pub trait NodeClassManager: Debug {
    fn new() -> Self
    where
        Self: Sized;
}
