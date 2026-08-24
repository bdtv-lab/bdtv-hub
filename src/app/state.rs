use core::time;
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

    /// 检查给定的 UUID 是否在任何服务器中在线
    fn uuid_in_any_server(&self, uuid: &Uuid) -> bool {
        let online_players = self.online_players.try_lock().unwrap();
        online_players
            .values()
            .any(|players| players.contains_key(uuid))
    }

    /// 标记玩家为在线状态
    pub async fn mark_player_as_online(&mut self, server: Server, player: Player) {
        // 检查是否任何服务器内都不包含该玩家
        let is_new = !self.uuid_in_any_server(&player.uuid);

        // 获取在线玩家的可变引用
        let online_players = self.online_players.get_mut();

        // 更新玩家的心跳时间
        online_players
            .entry(server.clone())
            .or_insert_with(HashMap::new)
            .insert(player.uuid, (player.clone(), Instant::now()));

        // 如果是新玩家则发送事件
        if is_new {
            let _ = self.event_tx.send(Event::PlayerJoined(player)).await;
        }
    }

    /// 检查玩家是否超时
    pub async fn check_player_timeouts(&mut self, timeout: u64) {
        let mut online_players = self.online_players.get_mut();
        let now = Instant::now();

        // 用于存储超时的玩家
        let mut timeout_players = Vec::new();

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
            let _ = self.event_tx.send(Event::PlayerLeft(player)).await;
        }
    }
}
