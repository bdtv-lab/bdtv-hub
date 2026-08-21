use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::AppState;

pub async fn http_server(state: Arc<AppState>, token: CancellationToken) {
    let app = Router::new().route("/hello", get(hello)).with_state(state);

    let listener = match TcpListener::bind("0.0.0.0:7497").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("bind failed: {e}");
            token.cancel();
            return;
        }
    };

    tracing::info!("listening on http://0.0.0.0:7497");

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
