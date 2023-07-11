use crate::{app::Osmium, core::DataType, utils::AutoInc, Event, OperationError};

#[derive(Debug)]
pub struct Input {
    datatype: DataType,
    output_id: Option<usize>,
}

impl Input {
    fn new(datatype: DataType) -> Self {
        Self {
            datatype,
            output_id: None,
        }
    }

    pub fn create(
        app: &mut Osmium,
        datatype: DataType,
        node_id: usize,
    ) -> Result<usize, OperationError> {
        if let None = app.nodes.get(&node_id) {
            return Err(OperationError::NonExistentItem);
        }

        let input_id = app.inputs.push(Input::new(datatype));

        app.emit(Event::InputsUpdated { node_id });

        Ok(input_id)
    }
}
