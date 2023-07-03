use super::{nodes::NodeExample, Node};
use crate::{app::Osmium, utils::AutoInc, OperationError};

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

    pub fn create(app: &Osmium) -> Result<(), OperationError> {
        let graph_id = app.graphs.borrow_mut().push(Graph::new());
        let my_node_id = Node::create::<NodeExample>(app, graph_id)?;

        let mut graphs = app.graphs.borrow_mut();
        let graph = graphs.get_mut(&graph_id).unwrap();
        graph.node_ids.push(my_node_id);

        Ok(())
    }

    pub fn step() -> i32 {
        return 5;
    }
}
