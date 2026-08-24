use smaragdine::Printer;
use tokio::sync::{Mutex, mpsc};

pub enum AppEvent {
    PlayerJoined,
    PlayerLeft,
}

#[derive(Debug)]
pub struct AppState {
    pub online_players: Mutex<u32>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<AppEvent>,
}

impl AppState {
    pub fn new(tx: mpsc::Sender<AppEvent>) -> Self {
        Self {
            online_players: Mutex::new(0),
            printer: Printer::new(),
            event_tx: tx,
        }
    }
}
