use axum::Json;
use kyori_component_json::{Color, Component, NamedColor, TextDecoration};

/// 服务器列表里显示的 MOTD
pub(super) async fn get_motd() -> Json<Component> {
    let motd = Component::Array(vec![
        Component::text("欢迎来到 ").color(Some(Color::Named(NamedColor::Gray))),
        Component::text("BDTV")
            .color(Some(Color::Named(NamedColor::Gold)))
            .decoration(TextDecoration::Bold, Some(true)),
        Component::from("\n"),
        Component::text("一个服务器").color(Some(Color::Named(NamedColor::DarkGray))),
    ]);

    Json(motd)
}
