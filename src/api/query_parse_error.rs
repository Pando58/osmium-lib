use crate::OperationError;
use strum_macros::Display;

#[derive(Clone, Copy, PartialEq, Display, Debug)]
pub enum QueryParseError {
    UnknownOperation,
    MissingArgument,
    InvalidArgument,
    OperationError(OperationError),
}
