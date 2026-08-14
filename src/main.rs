mod action;
mod config;
mod qq;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{MissedTickBehavior, interval};
use tokio_tungstenite::tungstenite::Message;

use crate::action::{Action, Players, handle_action};
use crate::config::{Config, load_config};

struct App {
    players: Players,
    config: Config,
    requester: qq::ReQuester,
}

impl App {
    fn new(config: Config) -> Self {
        Self {
            players: Mutex::new(HashMap::new()),
            config: config.clone(),
            requester: qq::ReQuester::new(&config.qq_http_api_base_url, config.qq_http_api_token),
        }
    }

    async fn run(self: Arc<Self>) -> Result<()> {
        tokio::spawn(self.clone().check_loop());

        let listener = TcpListener::bind(&self.config.ws_listen_addr).await?;
        println!("listening on ws://{}", self.config.ws_listen_addr);

        while let Ok((stream, peer)) = listener.accept().await {
            let app = self.clone();
            tokio::spawn(async move {
                if let Err(e) = app.handle(stream).await {
                    eprintln!("{peer} error: {e}");
                }
                println!("{peer} disconnected");
            });
        }

        Ok(())
    }

    /// 每 30 秒扫一遍，剔除心跳超时的玩家
    async fn check_loop(self: Arc<Self>) {
        let mut ticker = interval(Duration::from_secs(self.config.check_interval));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        ticker.tick().await; // 第一拍立即返回，吃掉它

        loop {
            ticker.tick().await;

            let timeout = Duration::from_secs(self.config.timeout);
            let dead: Vec<String> = {
                let mut players = self.players.lock().unwrap();
                let mut dead = Vec::new();
                players.retain(|id, last| {
                    let alive = last.elapsed() <= timeout;
                    if !alive {
                        dead.push(id.clone());
                    }
                    alive
                });
                println!("check: {} online", players.len());
                dead
            }; // 锁在这里释放，下面才能 await

            for id in dead {
                println!("player {id} timed out");
                if let Err(e) = self
                    .requester
                    .send_group_msg(self.config.qq_notice_group_id, "你已经超时了！")
                    .await
                {
                    eprintln!("notify failed for {id}: {e}");
                }
            }
        }
    }

    async fn handle(&self, stream: TcpStream) -> Result<()> {
        let peer = stream.peer_addr()?;
        let mut ws = tokio_tungstenite::accept_async(stream).await?;
        println!("{peer} connected");

        while let Some(msg) = ws.next().await {
            match msg? {
                Message::Text(text) => {
                    let action: Action = serde_json::from_str(&text)?;
                    handle_action(action, &self.players)?;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Arc::new(App::new(load_config()?)).run().await
}
