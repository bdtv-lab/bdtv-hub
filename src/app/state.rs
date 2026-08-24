use std::collections::HashMap;

use smaragdine::Printer;
use tokio::{
    sync::{Mutex, mpsc},
    time::Instant,
};
use tracing::debug;
use uuid::Uuid;

use crate::{
    app::event::Event,
    types::{Player, Server},
};

#[derive(Debug)]
pub struct State {
    pub online_players: Mutex<HashMap<Server, HashMap<Uuid, (Player, Instant)>>>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<Event>,
}

impl State {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self {
            online_players: Mutex::new(HashMap::new()),
            printer: Printer::new(),
            event_tx: tx,
        }
    }

    /// 标记玩家为在线状态
    pub async fn mark_player_as_online(&self, server: Server, player: Player) {
        let is_new = {
            // 获取在线玩家的可变引用
            let mut online_players = self.online_players.lock().await;

            // 检查是否任何服务器内都不包含该玩家
            let is_new = !online_players
                .values()
                .any(|players| players.contains_key(&player.uuid));

            // 更新玩家的心跳时间
            online_players
                .entry(server.clone())
                .or_insert_with(HashMap::new)
                .insert(player.uuid, (player.clone(), Instant::now()));

            is_new
        };

        // 如果是新玩家则发送事件
        if is_new {
            debug!("Player {} joined server", player.nickname);
            let _ = self.event_tx.send(Event::PlayerJoined(player)).await;
        }
    }

    /// 检查玩家是否超时
    pub async fn check_player_timeouts(&self, timeout: u64) {
        // 用于存储超时的玩家
        let mut timeout_players = Vec::new();

        let mut online_players = self.online_players.lock().await;
        let now = Instant::now();

        // 遍历每个服务器的在线玩家
        for (_, players) in online_players.iter_mut() {
            // 临时存储超时的 UUID
            // 避免迭代时修改
            let mut timed_out_uuids = Vec::new();

            // 检查每个玩家的心跳时间
            for (uuid, (player, last_heartbeat)) in players.iter() {
                if now.duration_since(*last_heartbeat).as_secs() > timeout {
                    timed_out_uuids.push(*uuid);
                    timeout_players.push(player.clone());
                }
            }

            for uuid in timed_out_uuids {
                players.remove(&uuid);
            }
        }

        // 发送超时玩家离开的事件
        for player in timeout_players {
            debug!("Player {} left server", player.nickname);
            let _ = self.event_tx.send(Event::PlayerLeft(player)).await;
        }
    }
}
