use crate::types::Player;

pub enum Event {
    PlayerJoined(Player),
    PlayerLeft(Player),
    PlayerCountChanged(usize),
}
