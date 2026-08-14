use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "data")]
pub enum Action {
    Heartbeat(PlayerHeartbeat),
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerHeartbeat {
    pub id: String,
    pub timestamp: i64,
}

pub fn handle_action(action: Action) -> Result<()> {
    match action {
        Action::Heartbeat(heartbeat) => {
            println!(
                "Received heartbeat from player {} at timestamp {}",
                heartbeat.id, heartbeat.timestamp
            );
        }
    }

    Ok(())
}
