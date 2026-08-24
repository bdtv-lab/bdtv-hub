use serde::Deserialize;
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Player {
    pub nickname: String,
    pub uuid: Uuid
}
