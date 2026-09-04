use std::sync::Arc;

use axum::{Json, extract::State};
use kyori_component_json::Component;

use crate::{app, richtext};

/// 服务器列表里显示的 MOTD
pub(super) async fn get_motd(State(state): State<Arc<app::State>>) -> Json<Component> {
    Json(Component::Array(richtext::motd(state).await))
}
