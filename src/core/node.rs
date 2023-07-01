use std::fmt::Debug;

use crate::core::nodes::NodeExample;

pub trait NodeTrait: Debug {
    fn new() -> Self
    where
        Self: Sized;
}

pub enum Node {
    Example,
}

pub fn new_node(variant: Node) -> impl NodeTrait {
    use Node::*;

    match variant {
        Example => NodeExample::new(),
    }
}
