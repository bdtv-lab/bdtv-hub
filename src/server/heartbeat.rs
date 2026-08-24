use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use tracing::trace;

use crate::{app::AppState, types::Player};


#[derive(Debug, Clone, Deserialize)]
pub(super) struct PlayerBeat {
    pub timestamp: u64,
    pub player: Player
}

pub(super) async fn beat_for_player(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlayerBeat>,
) -> String {
    let player = payload.player;
    trace!("Heartbeat received for {}: {}", player.nickname, player.uuid);

    todo!()
}
