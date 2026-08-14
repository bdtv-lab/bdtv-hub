use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use anyhow::Result;
use serde::Deserialize;

/// ws 连接任务和 30 秒检查器共享的数据：玩家 id -> 最后一次心跳的本地时刻
pub type Players = Mutex<HashMap<String, Instant>>;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "data")]
pub enum Action {
    Heartbeat(PlayerHeartbeat),
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerHeartbeat {
    pub id: String,
    pub timestamp: i64,
}

pub fn handle_action(action: Action, players: &Players) -> Result<()> {
    match action {
        Action::Heartbeat(heartbeat) => {
            println!(
                "Received heartbeat from player {} at timestamp {}",
                heartbeat.id, heartbeat.timestamp
            );
            players.lock().unwrap().insert(heartbeat.id, Instant::now());
        }
    }

    Ok(())
}
