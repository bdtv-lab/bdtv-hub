use smaragdine::Printer;
use tokio::sync::{Mutex, mpsc};

use crate::app::event::Event;

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<u32>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<Event>,
}

impl State {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self {
            online_players: Mutex::new(0),
            printer: Printer::new(),
            event_tx: tx,
        }
    }
}
