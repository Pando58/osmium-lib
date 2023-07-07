use crate::OperationError;

#[derive(PartialEq, Debug)]
pub enum QueryParseError {
    UnknownOperation,
    MissingArgument,
    InvalidArgument,
    OperationError(OperationError),
}
