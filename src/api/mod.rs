mod operation_error;
mod operation_response;
mod operations;
mod query_parse_error;

pub use operation_error::OperationError;
pub use operation_response::OperationResponse;
pub use query_parse_error::QueryParseError;

use self::operations::OpQuery;
use crate::app::Osmium;

pub fn query(app: &mut Osmium) -> OpQuery {
    OpQuery::new(app)
}

pub fn query_str(app: &mut Osmium, query: &str) -> Result<OperationResponse, QueryParseError> {
    let args = query.split_whitespace();

    OpQuery::new(app).query(args)
}
