use std::{sync::Arc, time::Duration};

use tokio::time;
use tokio_util::sync::CancellationToken;
use tracing::trace;

use crate::app;

pub async fn warden(state: Arc<app::State>, token: CancellationToken) {
    // 创建定时器
    let mut ticker = time::interval(Duration::from_secs(1));
    // 第一次 tick 立即触发
    // 也就是跳过第一次的触发逻辑
    ticker.tick().await;

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                check(&state).await;
            }

            _ = token.cancelled() => {
                break;
            }
        }
    }
}

async fn check(state: &app::State) {
    trace!("Checking player timeouts");
    state.check_player_timeouts(10).await;
    state.check_server_timeouts(10).await;
}
