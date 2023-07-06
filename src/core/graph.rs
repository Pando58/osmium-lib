use crate::{
    app::Osmium,
    core::{nodes::Nodes, Node},
    utils::AutoInc,
    OperationError,
};

#[derive(Debug)]
pub struct Graph {
    pub node_ids: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            node_ids: Vec::new(),
        }
    }

    pub fn create(app: &mut Osmium) -> Result<(), OperationError> {
        let graph_id = app.graphs.push(Graph::new());
        let my_node_id = Node::create(app, Nodes::Example, graph_id)?;

        let graph = app.graphs.get_mut(&graph_id).unwrap();
        graph.node_ids.push(my_node_id);

        Ok(())
    }
}
