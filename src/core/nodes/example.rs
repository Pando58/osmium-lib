use crate::{
    app::Osmium,
    core::{DataType, Input},
    OperationError,
};

pub struct NodeExample;

impl NodeExample {
    pub fn create(app: &mut Osmium, node_id: usize) -> Result<(), OperationError> {
        let my_input_id = Input::create(app, DataType::Integer, node_id)?;

        let node = app.nodes.get_mut(&node_id).unwrap();
        node.input_ids.push(my_input_id);

        Ok(())
    }
}
