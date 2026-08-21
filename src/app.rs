use smaragdine::Printer;
use tokio::sync::Mutex;


#[derive(Debug, Default)]
pub struct AppState {
    pub online_players: Mutex<u32>,
    pub printer: Printer,
}
