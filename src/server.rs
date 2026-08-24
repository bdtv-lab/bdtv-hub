use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::AppState;

/// 启动 HTTP 服务器
/// 
/// 这个 HTTP 服务器会接受来自 MCDR 插件发出的请求，并给予相应回复
/// 
/// MCDR 有能力通过这个中心服务器获取整个服务器集群的状态
pub async fn http_server(state: Arc<AppState>, token: CancellationToken) {
    let app = Router::new().route("/hello", get(hello)).with_state(state);

    // 绑定 TCP 监听器
    let listener = match TcpListener::bind("0.0.0.0:7497").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("bind failed: {e}");
            token.cancel();
            return;
        }
    };

    tracing::info!("listening on http://0.0.0.0:7497");

    // 启动 HTTP 服务器
    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(token.clone().cancelled_owned())
        .await
    {
        tracing::error!("server error: {e}");
        token.cancel();
    }
}

async fn hello() -> &'static str {
    "Hello, world!"
}
