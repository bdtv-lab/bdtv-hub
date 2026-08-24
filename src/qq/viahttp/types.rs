use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

/// OneBot HTTP API 响应格式
#[derive(Debug, Deserialize)]
pub(super) struct ApiResponse {
    pub retcode: i64,
    pub data: Value,
    pub message: String,
}
