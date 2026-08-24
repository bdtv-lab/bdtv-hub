use std::{sync::Arc, time::Duration};

use tokio::time;
use tokio_util::sync::CancellationToken;

use crate::app;

pub async fn warden(_state: Arc<app::State>, token: CancellationToken) {
    let client = reqwest::Client::new();
    let mut ticker = time::interval(Duration::from_secs(30));
    ticker.tick().await;

    loop {
        let response = tokio::select! {
            _ = token.cancelled() => break,
            response = async {
                ticker.tick().await;
                client.get("http://127.0.0.1:7497/hello").send().await
            } => response,
        };

        match response {
            Ok(response) => tracing::info!("self request: {}", response.status()),
            Err(error) => tracing::error!("self request failed: {error}"),
        }
    }
}
