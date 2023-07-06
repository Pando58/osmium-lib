mod operations;

use crate::app::Osmium;

use self::operations::OpQuery;

pub fn query(app: &mut Osmium) -> OpQuery {
    OpQuery::new(app)
}
