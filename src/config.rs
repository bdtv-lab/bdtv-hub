use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "config.yaml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub qq_http_api_base_url: String,
    pub qq_http_api_token: Option<String>,
    pub qq_notice_group_id: u64,

    pub ws_listen_addr: String,
    pub check_interval: u64,
    pub timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            qq_http_api_base_url: "http://127.0.0.1:3000".to_string(),
            qq_http_api_token: None,
            qq_notice_group_id: 12345678,
            ws_listen_addr: "0.0.0.0:7497".to_string(),
            check_interval: 15,
            timeout: 30,
        }
    }
}

/// 读取 config.yaml；不存在则写一份默认配置再返回
pub fn load_config() -> Result<Config> {
    let path = Path::new(CONFIG_PATH);

    if !path.exists() {
        let config = Config::default();
        let text = serde_yaml::to_string(&config)?;
        fs::write(path, text).with_context(|| format!("写入默认配置 {CONFIG_PATH} 失败"))?;
        println!("已生成默认配置 {CONFIG_PATH}，请按需修改");
        return Ok(config);
    }

    let text = fs::read_to_string(path).with_context(|| format!("读取 {CONFIG_PATH} 失败"))?;
    let config: Config =
        serde_yaml::from_str(&text).with_context(|| format!("解析 {CONFIG_PATH} 失败"))?;

    Ok(config)
}
