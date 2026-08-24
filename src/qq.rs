mod fake;

use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::app;
pub use fake::DummyReq;

pub trait ReQuester {
    async fn handle_event(&self, event: &app::Event) -> Result<()>;
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

            Some(event) = rx.recv() => {
                let _ = requester.handle_event(&event).await;

            }
        }
    }
}
