use std::{
    collections::{HashMap, HashSet},
    sync::atomic::Ordering,
};

use tokio::time::Instant;
use tracing::debug;

use uuid::Uuid;

use crate::{
    app::{Event, State},
    types::{Player, Server},
};

impl State {
    /// 统计所有服务器内不重复的在线玩家数量
    async fn unique_player_count(&self) -> usize {
        self.online_players
            .lock()
            .await
            .values()
            .flat_map(|players| players.keys())
            .collect::<HashSet<_>>()
            .len()
    }

    /// 上报去重后的在线人数
    pub async fn report_player_count(&self) {
        let count = self.unique_player_count().await;

        // 与上次上报的值相同则不重复发送
        if self.last_reported_count.swap(count, Ordering::Relaxed) == count {
            return;
        }

        debug!("Player count changed to {count}");
        let _ = self.event_tx.send(Event::PlayerCountChanged(count)).await;
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
    ///
    /// 玩家可能同时在多个服务器上
    ///
    /// 只有从所有服务器上都消失才算离开
    pub async fn check_player_timeouts(&self, timeout: u64) {
        // 限制锁的作用域
        let left_players = {
            let mut online_players = self.online_players.lock().await;
            let now = Instant::now();

            // 剔除所有服务器上的超时玩家
            // 同一名玩家可能在多个服务器上超时
            let mut timed_out: HashMap<Uuid, Player> = HashMap::new();
            for players in online_players.values_mut() {
                players.retain(|uuid, (player, last_heartbeat)| {
                    let alive = now.duration_since(*last_heartbeat).as_secs() <= timeout;
                    if !alive {
                        timed_out.insert(*uuid, player.clone());
                    }
                    alive
                });
            }

            // 仍然存在的玩家为切换服务器的玩家
            timed_out
                .into_iter()
                .filter(|(uuid, _)| {
                    !online_players
                        .values()
                        .any(|players| players.contains_key(uuid))
                })
                .map(|(_, player)| player)
                .collect::<Vec<_>>()
        };

        // 发送玩家离开的事件
        for player in left_players {
            debug!("Player {} left server", player.nickname);
            let _ = self.event_tx.send(Event::PlayerLeft(player)).await;
        }
    }

    pub async fn mark_server_as_online(&self, server: Server) {
        let mut online_servers = self.online_servers.lock().await;
        let is_new = online_servers
            .insert(server.clone(), Instant::now())
            .is_none();

        if is_new {
            debug!("Server {} is online", server.name);
        }
    }

    pub async fn check_server_timeouts(&self, timeout: u64) {
        let mut timeout_servers = Vec::new();

        let mut online_servers = self.online_servers.lock().await;
        let now = Instant::now();

        for (server, last_heartbeat) in online_servers.iter() {
            if now.duration_since(*last_heartbeat).as_secs() > timeout {
                timeout_servers.push(server.clone());
            }
        }

        for server in timeout_servers {
            online_servers.remove(&server);
            debug!("Server {} is offline", server.name);
        }
    }
}
