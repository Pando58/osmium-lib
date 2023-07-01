mod operations;

use crate::app::Osmium;

use self::operations::OpQuery;

pub fn query(app: &Osmium) -> OpQuery {
    OpQuery::new(app)
}
