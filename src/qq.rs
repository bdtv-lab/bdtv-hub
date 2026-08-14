use anyhow::Result;

pub struct ReQuester {
    pub client: reqwest::Client,
    pub base_url: String,
    token: Option<String>,
}

impl ReQuester {
    pub fn new(base_url: &str, token: Option<String>) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            base_url: base_url.to_string(),
            token,
        }
    }

    pub async fn send_group_msg(&self, group_id: u64, message: &str) -> Result<()> {
        let url = format!("{}/send_group_msg", self.base_url);
        let data = serde_json::json!({
            "group_id": group_id,
            "message": message,
        });

        let mut req = self.client.post(&url).json(&data);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to send group message: {status} {}",
                res.text().await?
            ));
        }

        Ok(())
    }
}
