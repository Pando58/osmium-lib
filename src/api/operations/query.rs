use crate::{
    api::{operations::OpCreate, query_parse_error::QueryParseError},
    app::Osmium,
};
use std::str::SplitWhitespace;

pub struct OpQuery<'a> {
    app: &'a mut Osmium,
}

impl<'a> OpQuery<'a> {
    pub fn new(app: &'a mut Osmium) -> Self {
        Self { app }
    }

    pub fn create(self) -> OpCreate<'a> {
        OpCreate::new(self.app)
    }

    pub fn query(self, mut args: SplitWhitespace) -> Result<(), QueryParseError> {
        match args.next() {
            Some("create") => self.create().query(args),
            Some(_) => Err(QueryParseError::UnknownOperation),
            None => Err(QueryParseError::MissingArgument),
        }
    }
}
