use kyori_component_json::{Component, HoverEvent, UuidRepr};

use crate::types::Player;

pub(super) fn player_component(player: Player) -> Component {
    Component::text(&player.nickname).hover_event(Some(HoverEvent::ShowEntity {
        name: Some(Component::text(&player.nickname)),
        id: String::from("minecraft:player"),
        uuid: UuidRepr::String(player.uuid.to_string()),
    }))
}
