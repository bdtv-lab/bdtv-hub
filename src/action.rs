use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use anyhow::Result;
use serde::Deserialize;

use crate::App;

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

impl App {
    pub async fn handle_action(&self, action: Action, players: &Players) -> Result<()> {
        match action {
            Action::Heartbeat(heartbeat) => {
                println!(
                    "Received heartbeat from player {} at timestamp {}",
                    heartbeat.id, heartbeat.timestamp
                );
                // insert 返回 None 说明之前没有这个 key，即新玩家
                let is_new = {
                    let mut players = players.lock().unwrap();
                    players
                        .insert(heartbeat.id.clone(), Instant::now())
                        .is_none()
                }; // 锁在这里释放，下面才能 await

                if is_new {
                    println!("New player connected: {}", heartbeat.id);
                    self.requester
                        .send_group_msg(
                            self.config.qq_notice_group_id,
                            &format!("Hello {}", heartbeat.id),
                        )
                        .await?;
                }
            }
        }

        Ok(())
    }
}
