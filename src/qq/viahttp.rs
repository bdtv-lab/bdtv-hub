use tracing::{debug, info};

use anyhow::Result;
use reqwest::{RequestBuilder, Response};
use serde_json::Value;

use crate::{app, qq::ReQuester};

#[derive(Debug)]
pub struct HttpReq {
    base_url: String,
    token: Option<String>,
    group_id: i64,
    client: reqwest::Client,
}

impl HttpReq {
    pub fn new(base_url: String, token: Option<String>, group_id: i64) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("BDTV/1.0")
            .build()
            .unwrap();

        debug!(
            "HttpReq initialized with base_url: {}, send to group: {}",
            base_url, group_id
        );

        Self {
            base_url,
            token,
            group_id,
            client,
        }
    }

    /// 以 GET 请求访问指定接口
    ///
    /// 参数作为 query 附加
    async fn get(&self, endpoint: &str, params: &Value) -> Result<Response> {
        let url = format!("{}/{endpoint}", self.base_url);
        self.send(self.client.get(&url).query(params), endpoint)
            .await
    }

    /// 以 POST 请求访问指定接口
    ///
    /// 参数作为 JSON body 发送
    async fn post(&self, endpoint: &str, data: &Value) -> Result<Response> {
        let url = format!("{}/{endpoint}", self.base_url);
        self.send(self.client.post(&url).json(data), endpoint).await
    }

    /// 附加鉴权信息后发送请求
    async fn send(&self, req: RequestBuilder, endpoint: &str) -> Result<Response> {
        let req = match &self.token {
            Some(token) => req.bearer_auth(token),
            None => req,
        };

        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to request {endpoint}: {status} {}",
                res.text().await?
            ));
        }

        Ok(res)
    }

    /// 获取登录账号的信息
    pub async fn get_login_info(&self) -> Result<()> {
        self.get("get_login_info", &serde_json::json!({})).await?;

        Ok(())
    }

    /// 发送群聊消息
    pub async fn send_group_msg(&self, group_id: i64, message: &str) -> Result<()> {
        self.post(
            "send_group_msg",
            &serde_json::json!({
                "group_id": group_id,
                "message": message,
            }),
        )
        .await?;

        Ok(())
    }
}

impl ReQuester for HttpReq {
    async fn handle_event(&self, event: &app::Event) -> Result<()> {
        match event {
            app::Event::PlayerJoined(player) => {
                info!(
                    "Sending message to QQ: Player {} joined server",
                    player.nickname
                );
                self.send_group_msg(self.group_id, &format!("{} 加入了服务器", player.nickname))
                    .await?;
                Ok(())
            }
            app::Event::PlayerLeft(player) => {
                info!(
                    "Sending message to QQ: Player {} left server",
                    player.nickname
                );
                self.send_group_msg(self.group_id, &format!("{} 离开了服务器", player.nickname))
                    .await?;
                Ok(())
            }
        }
    }
}
