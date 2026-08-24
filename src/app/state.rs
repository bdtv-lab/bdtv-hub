mod heartbeat;

use std::collections::HashMap;

use smaragdine::Printer;
use tokio::{
    sync::{Mutex, mpsc},
    time::Instant,
};
use uuid::Uuid;

use crate::{
    app::event::Event,
    types::{Player, Server},
};

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<HashMap<Server, HashMap<Uuid, (Player, Instant)>>>,
    pub online_servers: Mutex<HashMap<Server, Instant>>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<Event>,
}

impl State {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self {
            online_players: Mutex::new(HashMap::new()),
            online_servers: Mutex::new(HashMap::new()),
            printer: Printer::new(),
            event_tx: tx,
        }
    }
}
