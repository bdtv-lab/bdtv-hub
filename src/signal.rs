use tokio_util::sync::CancellationToken;

pub async fn shutdown_signal(token: CancellationToken) {
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
