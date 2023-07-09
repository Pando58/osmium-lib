use strum_macros::Display;

#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum OperationResponse {
    Empty,
    Id(usize),
}
