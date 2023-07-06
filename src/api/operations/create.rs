use crate::{
    app::Osmium,
    core::{nodes::Nodes, Graph, Node},
    OperationError,
};

pub struct OpCreate<'a> {
    app: &'a mut Osmium,
}

impl<'a> OpCreate<'a> {
    pub fn new(app: &'a mut Osmium) -> Self {
        Self { app }
    }

    pub fn graph(self) -> Result<(), OperationError> {
        Graph::create(self.app)
    }

    pub fn node(self, kind: Nodes, graph_id: usize) -> Result<(), OperationError> {
        Node::create(self.app, kind, graph_id).map(|_| ())
    }
}
