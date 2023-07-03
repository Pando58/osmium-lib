use crate::core::node::NodeClassManager;

#[derive(Debug)]
pub struct NodeExample {}

impl NodeClassManager for NodeExample {
    fn new() -> Self {
        Self {}
    }
}
