use crate::{api::operations::OpCreate, app::Osmium};

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
}
