use crate::{
    api::query_parse_error::QueryParseError,
    app::Osmium,
    core::{nodes::Nodes, Graph, Node},
    OperationError,
};
use std::str::{FromStr, SplitWhitespace};

pub struct OpCreate<'a> {
    app: &'a mut Osmium,
}

impl<'a> OpCreate<'a> {
    pub fn new(app: &'a mut Osmium) -> Self {
        Self { app }
    }

    pub fn graph(self) -> Result<(), OperationError> {
        Graph::create(self.app)
    }

    pub fn node(self, kind: Nodes, graph_id: usize) -> Result<(), OperationError> {
        Node::create(self.app, kind, graph_id).map(|_| ())
    }

    pub fn query(self, mut args: SplitWhitespace) -> Result<(), QueryParseError> {
        match args.next() {
            Some("graph") => self
                .graph()
                .map_err(|err| QueryParseError::OperationError(err)),
            Some("node") => {
                let kind = match args.next() {
                    Some(s) => Nodes::from_str(s).map_err(|_| QueryParseError::InvalidArgument),
                    None => Err(QueryParseError::MissingArgument),
                }?;

                let graph_id = match args.next() {
                    Some(s) => s
                        .parse::<usize>()
                        .map_err(|_| QueryParseError::InvalidArgument),
                    None => Err(QueryParseError::MissingArgument),
                }?;

                self.node(kind, graph_id)
                    .map_err(|err| QueryParseError::OperationError(err))
            }
            Some(_) => Err(QueryParseError::UnknownOperation),
            None => Err(QueryParseError::MissingArgument),
        }
    }
}
