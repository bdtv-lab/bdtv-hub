mod app;
mod console;
mod envconf;
mod logging;
mod server;
mod warden;

use std::sync::Arc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{app::AppState, console::console, server::http_server, warden::warden};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用状态
    let state = Arc::new(AppState::default());
    // 初始化日志系统
    logging::init(&state);

    let token = CancellationToken::new();
    let mut set = JoinSet::new();

    // 启动控制台
    set.spawn(console(Arc::clone(&state), token.clone()));
    // 启动 http 服务器
    set.spawn(http_server(Arc::clone(&state), token.clone()));
    // 启动在线状态巡检
    set.spawn(warden(Arc::clone(&state), token.clone()));
    // 启动关闭信号监听
    set.spawn(shutdown_signal(token));

    // 关键：等所有任务真正退出，main 才会返回
    while let Some(res) = set.join_next().await {
        if let Err(e) = res {
            tracing::error!("发生 panic: {e}");
        }
    }
    tracing::info!("服务已退出");

    Ok(())
}

async fn shutdown_signal(token: CancellationToken) {
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("无法监听 SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => tracing::info!("收到 Ctrl+C"),
        _ = terminate => tracing::info!("收到 SIGTERM"),
        _ = token.cancelled() => return,
    }

    token.cancel();
}
