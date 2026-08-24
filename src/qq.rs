mod fake;
mod viahttp;

use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

use crate::{app, envconf::Config};
pub use {fake::DummyReq, viahttp::HttpReq};

pub trait ReQuester {
    async fn handle_event(&self, event: &app::Event) -> Result<()>;
}

/// 所有消息发送器实现的集合
pub enum AnyReq {
    Http(HttpReq),
    Dummy(DummyReq),
}

impl ReQuester for AnyReq {
    async fn handle_event(&self, event: &app::Event) -> Result<()> {
        match self {
            Self::Http(req) => req.handle_event(event).await,
            Self::Dummy(req) => req.handle_event(event).await,
        }
    }
}

pub fn get_requester(config: Config) -> AnyReq {
    if let Some(base_url) = config.qq_http_api_base_url
        && let Some(group_id) = config.qq_notice_group_id
    {
        AnyReq::Http(HttpReq::new(
            base_url.clone(),
            config.qq_http_api_token.clone(),
            group_id as i64,
        ))
    } else {
        warn!("未配置 QQ HTTP API");
        AnyReq::Dummy(DummyReq)
    }
}

pub async fn qq_requester(
    mut rx: Receiver<app::Event>,
    requester: impl ReQuester,
    token: CancellationToken,
) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                break;
            }

            event = rx.recv() => {
                // 通道已关闭，不会再有事件
                let Some(event) = event else {
                    break;
                };

                if let Err(e) = requester.handle_event(&event).await {
                    error!("处理事件失败: {e}");
                }
            }
        }
    }
}
