mod example;

use crate::{
    app::Osmium,
    core::{nodes::example::NodeExample, Node},
    utils::AutoInc,
    OperationError,
};
use strum_macros::EnumString;

#[derive(Clone, Copy, EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Nodes {
    // #[strum(disabled)]
    Example,
}

impl Nodes {
    pub fn create(&self, app: &mut Osmium) -> Result<usize, OperationError> {
        use Nodes::*;

        let node_id = app.nodes.push(Node::new(*self));

        match self {
            Example => NodeExample::create(app, node_id),
        }?;

        Ok(node_id)
    }
}
