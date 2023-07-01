use crate::{
    app::Osmium,
    core::{new_node, Graph, Node},
    utils::AutoInc,
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

    pub fn graph(self) {
        Graph::create(self.app);
    }

    pub fn node(self, graph_id: usize, node: Node) -> Result<(), OperationError> {
        let mut graphs = self.app.graphs.borrow_mut();

        let Some(graph) = graphs.get_mut(&graph_id) else {
            return Err(OperationError::NonExistentItem);
        };

        let id = self.app.nodes.borrow_mut().push(Box::new(new_node(node)));

        graph.node_ids.push(id);

        Ok(())
    }

    // pub fn add(self, n: i32) -> OpAdd<'a> {
    //     OpAdd::new(self.app, n)
    // }
}
