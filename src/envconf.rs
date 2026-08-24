use std::env;

use dotenvy::dotenv;

const DEFAULT_HTTP_LISTEN_ADDR: &str = "127.0.0.1:3000";
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

pub fn load_env() -> Config {
    dotenv().ok();

    Config {
        qq_http_api_base_url: env::var("QQ_HTTP_API_BASE_URL").ok(),
        qq_http_api_token: env::var("QQ_HTTP_API_TOKEN").ok(),
        qq_notice_group_id: env::var("QQ_NOTICE_GROUP_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok()),
        http_listen_addr: env::var("HTTP_LISTEN_ADDR")
            .unwrap_or_else(|_| DEFAULT_HTTP_LISTEN_ADDR.to_string()),
        check_interval: env::var("CHECK_INTERVAL")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_CHECK_INTERVAL),
        timeout: env::var("TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_TIMEOUT),
    }
}
