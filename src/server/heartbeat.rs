use std::sync::Arc;

use axum::{Json, extract::State};
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::trace;

use crate::{
    app,
    types::{Player, Server},
};

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PlayerBeat {
    pub server: Server,
    pub player: Player,
}

pub(super) async fn beat_for_player(
    State(state): State<Arc<app::State>>,
    Json(payload): Json<PlayerBeat>,
) -> StatusCode {
    let player = payload.player;
    let server = payload.server;
    trace!(
        "Heartbeat received for {}: {} in server {}",
        player.nickname, player.uuid, server.name
    );

    state.mark_player_as_online(server, player).await;

    StatusCode::OK
}
