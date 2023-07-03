use crate::{
    app::Osmium,
    core::{create_node, Graph, Node},
    OperationError,
};

// use crate::operations::OpAdd;

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

    pub fn node(self, graph_id: usize, class: Node) -> Result<(), OperationError> {
        create_node(self.app, graph_id, class).map(|_| ())
    }

    // pub fn add(self, n: i32) -> OpAdd<'a> {
    //     OpAdd::new(self.app, n)
    // }
}
