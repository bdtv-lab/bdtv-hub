use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{app, types::Server};

pub(super) async fn get_servers(State(state): State<Arc<app::State>>) -> Json<Vec<Server>> {
    let mut servers: Vec<_> = state.online_servers.lock().await.keys().cloned().collect();
    servers.sort_by(|a, b| a.slug.cmp(&b.slug));
    Json(servers)
}
