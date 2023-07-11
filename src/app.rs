use crate::{
    core::{Graph, Input, Node},
    Event,
};
use std::collections::HashMap;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug)]
pub struct Osmium {
    pub graphs: HashMap<usize, Graph>,
    pub nodes: HashMap<usize, Node>,
    pub inputs: HashMap<usize, Input>,
    event_tx: Sender<Event>,
}

impl Osmium {
    pub fn new() -> Self {
        Self {
            graphs: HashMap::new(),
            nodes: HashMap::new(),
            inputs: HashMap::new(),
            event_tx: broadcast::channel::<Event>(100).0,
        }
    }

    pub fn emit(&self, event: Event) {
        let _ = self.event_tx.send(event);
    }

    pub fn create_event_receiver(&self) -> Receiver<Event> {
        self.event_tx.subscribe()
    }
}
