use crate::app::Osmium;

use super::OpCreate;

pub struct OpQuery<'a> {
    app: &'a Osmium,
}

impl<'a> OpQuery<'a> {
    pub fn new(app: &'a Osmium) -> Self {
        Self { app }
    }

    pub fn create(self) -> OpCreate<'a> {
        OpCreate::new(self.app)
    }
}
