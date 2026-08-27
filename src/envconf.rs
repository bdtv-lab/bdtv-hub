use std::env;

const DEFAULT_HTTP_LISTEN_ADDR: &str = "127.0.0.1:7497";
const DEFAULT_CHECK_INTERVAL: u64 = 5;
const DEFAULT_TIMEOUT: u64 = 30;

#[derive(Debug, Clone)]
pub struct Config {
    pub qq_http_api_base_url: Option<String>,
    pub qq_http_api_token: Option<String>,
    pub qq_notice_group_id: Option<u64>,
    pub http_listen_addr: String,
    pub check_interval: u64,
    pub timeout: u64,
}

/// 读取环境变量
///
/// 未设置或值为空时返回 `None`
fn env_var(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn load_env() -> Config {
    Config {
        qq_http_api_base_url: env_var("QQ_HTTP_API_BASE_URL"),
        qq_http_api_token: env_var("QQ_HTTP_API_TOKEN"),
        qq_notice_group_id: env_var("QQ_NOTICE_GROUP_ID").and_then(|s| s.parse::<u64>().ok()),
        http_listen_addr: env_var("HTTP_LISTEN_ADDR")
            .unwrap_or_else(|| DEFAULT_HTTP_LISTEN_ADDR.to_string()),
        check_interval: env_var("CHECK_INTERVAL")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_CHECK_INTERVAL),
        timeout: env_var("TIMEOUT")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_TIMEOUT),
    }
}
