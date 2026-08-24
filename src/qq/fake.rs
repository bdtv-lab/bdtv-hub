use tracing::debug;

use crate::{app, qq::ReQuester};

#[derive(Debug, Default)]
pub struct DummyReq;

impl ReQuester for DummyReq {
    async fn handle_event(&self, event: &app::Event) -> anyhow::Result<()> {
        match event {
            app::Event::PlayerJoined(player) => {
                debug!(
                    "Sent message to QQ: Player {} joined server",
                    player.nickname
                );
                Ok(())
            }
            app::Event::PlayerLeft(player) => {
                debug!("Sent message to QQ: Player {} left server", player.nickname);
                Ok(())
            }
        }
    }
}
