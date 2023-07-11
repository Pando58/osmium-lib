use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Event {
    GraphsUpdated,
    NodesUpdated { graph_id: usize },
    InputsUpdated { node_id: usize },
}
