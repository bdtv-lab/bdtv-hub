mod heartbeat;

use std::{collections::HashMap, sync::atomic::AtomicUsize};

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

type OnlineServers = HashMap<Server, Instant>;
type OnlinePlayers = HashMap<Uuid, (Player, Instant)>;

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<HashMap<Server, OnlinePlayers>>,
    pub online_servers: Mutex<OnlineServers>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<Event>,
    /// 上次上报的去重在线人数
    last_reported_count: AtomicUsize,
}

impl State {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self {
            online_players: Mutex::new(HashMap::new()),
            online_servers: Mutex::new(HashMap::new()),
            printer: Printer::new(),
            event_tx: tx,
            last_reported_count: AtomicUsize::new(0),
        }
    }
}
