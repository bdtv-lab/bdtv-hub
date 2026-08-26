mod types;

use tracing::debug;

use anyhow::{Context, Result};
use reqwest::RequestBuilder;
use serde::de::{DeserializeOwned, IgnoredAny};
use serde_json::Value;

use crate::{
    app,
    qq::{
        ReQuester,
        viahttp::types::{ApiResponse, LoginInfo},
    },
};

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
    async fn get<T: DeserializeOwned>(&self, endpoint: &str, params: &Value) -> Result<T> {
        let url = format!("{}/{endpoint}", self.base_url);
        self.send(self.client.get(&url).query(params), endpoint)
            .await
    }

    /// 以 POST 请求访问指定接口
    ///
    /// 参数作为 JSON body 发送
    async fn post<T: DeserializeOwned>(&self, endpoint: &str, data: &Value) -> Result<T> {
        let url = format!("{}/{endpoint}", self.base_url);
        self.send(self.client.post(&url).json(data), endpoint).await
    }

    /// 附加鉴权信息后发送请求
    async fn send<T: DeserializeOwned>(&self, req: RequestBuilder, endpoint: &str) -> Result<T> {
        let req = match &self.token {
            Some(token) => req.bearer_auth(token),
            None => req,
        };

        let res = req.send().await?;
        let status = res.status();
        let body = res.text().await?;

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to request {endpoint}: {status} {body}"
            ));
        }

        let res: ApiResponse = serde_json::from_str(&body)
            .with_context(|| format!("Failed to parse response of {endpoint}: {body}"))?;

        // 根据 retcode 判断请求是否成功
        if res.retcode != 0 {
            return Err(anyhow::anyhow!(
                "Failed to request {endpoint}: retcode {} {}",
                res.retcode,
                res.message
            ));
        }

        serde_json::from_value(res.data)
            .with_context(|| format!("Failed to parse data of {endpoint}: {body}"))
    }

    /// 获取登录账号的信息
    async fn get_login_info(&self) -> Result<LoginInfo> {
        self.get("get_login_info", &serde_json::json!({})).await
    }

    async fn set_group_card(&self, group_id: i64, user_id: i64, card: &str) -> Result<()> {
        self.post::<IgnoredAny>(
            "set_group_card",
            &serde_json::json!({
                "group_id": group_id,
                "user_id": user_id,
                "card": card,
            }),
        )
        .await?;

        Ok(())
    }

    /// 发送群聊消息
    async fn send_group_msg(&self, group_id: i64, message: &str) -> Result<()> {
        self.post::<IgnoredAny>(
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
                debug!(
                    "Sending message to QQ: Player {} joined server",
                    player.nickname
                );
                self.send_group_msg(self.group_id, &format!("{} 加入了服务器", player.nickname))
                    .await?;
                Ok(())
            }
            app::Event::PlayerLeft(player) => {
                debug!(
                    "Sending message to QQ: Player {} left server",
                    player.nickname
                );
                self.send_group_msg(self.group_id, &format!("{} 离开了服务器", player.nickname))
                    .await?;
                Ok(())
            }
            app::Event::PlayerCountChanged(count) => {
                let login_info = self.get_login_info().await?;

                let card = if *count == 0 {
                    format!("{}", login_info.nickname)
                } else {
                    format!("{} 人在线", count)
                };

                self.set_group_card(self.group_id, login_info.user_id, &card)
                    .await?;
                Ok(())
            }
        }
    }
}
