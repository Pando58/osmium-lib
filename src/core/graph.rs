use super::{nodes::NodeExample, NodeTrait};
use crate::{app::Osmium, utils::AutoInc};

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

    pub fn create(app: &Osmium) {
        app.nodes.borrow_mut().push(Box::new(NodeExample::new()));

        let mut graphs = app.graphs.borrow_mut();

        let id = graphs.push(Graph::new());
        let graph = graphs.get_mut(&id).unwrap();

        graph.node_ids.push(id);
    }

    pub fn step() -> i32 {
        return 5;
    }
}
