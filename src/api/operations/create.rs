use crate::{
    app::Osmium,
    core::{Graph, Node, NodeClassManager},
    OperationError,
};

pub struct OpCreate<'a> {
    app: &'a Osmium,
}

impl<'a> OpCreate<'a> {
    pub fn new(app: &'a Osmium) -> Self {
        Self { app }
    }

    pub fn graph(self) -> Result<(), OperationError> {
        Graph::create(self.app)
    }

    pub fn node<T: NodeClassManager + 'static>(
        self,
        graph_id: usize,
    ) -> Result<(), OperationError> {
        Node::create::<T>(self.app, graph_id).map(|_| ())
    }
}
