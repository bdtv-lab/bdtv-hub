use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Player {
    pub nickname: String,
    pub uuid: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Server {
    pub nickname: String,
    pub slug: String,
    pub address: String,
    pub port: u16,
}
