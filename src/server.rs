mod heartbeat;
mod motd;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::{app, envconf::Config};

/// 启动 HTTP 服务器
///
/// 这个 HTTP 服务器会接受来自 MCDR 插件发出的请求，并给予相应回复
///
/// MCDR 有能力通过这个中心服务器获取整个服务器集群的状态
pub async fn http_server(config: Config, state: Arc<app::State>, token: CancellationToken) {
    let app = Router::new()
        .route("/motd", get(motd::get_motd))
        .route("/beat/player", post(heartbeat::beat_for_player))
        .route("/beat/server", post(heartbeat::beat_for_server))
        .with_state(state);

    // 绑定 TCP 监听器
    let listener = match TcpListener::bind(config.http_listen_addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("bind failed: {e}");
            token.cancel();
            return;
        }
    };

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    // 启动 HTTP 服务器
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(token.clone().cancelled_owned())
        .await
    {
        tracing::error!("server error: {e}");
        token.cancel();
    }
}
