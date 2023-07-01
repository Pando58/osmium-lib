use crate::core::NodeTrait;

#[derive(Debug)]
pub struct NodeExample;

impl NodeTrait for NodeExample {
    fn new() -> Self {
        Self
    }
}
