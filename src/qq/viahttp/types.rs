use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}
