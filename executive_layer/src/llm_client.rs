use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Cloudflare AI / Gemini API client for LLM inference
pub struct LlmClient {
    client: reqwest::Client,
    cf_account_id: String,
    cf_api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmRequest {
    pub messages: Vec<LlmMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub result: LlmResult,
}

#[derive(Debug, Deserialize)]
pub struct LlmResult {
    pub response: String,
}

impl LlmClient {
    pub fn new(cf_account_id: &str, cf_api_token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            cf_account_id: cf_account_id.to_string(),
            cf_api_token: cf_api_token.to_string(),
        }
    }

    /// Call Cloudflare Workers AI — Llama 3.1 8B Instruct
    pub async fn call_cf_ai(&self, system_prompt: &str, user_input: &str) -> Result<String> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/@cf/meta/llama-3.1-8b-instruct",
            self.cf_account_id
        );

        let body = serde_json::json!({
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_input }
            ]
        });

        let res = self.client
            .post(&url)
            .bearer_auth(&self.cf_api_token)
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;
        let response = json["result"]["response"]
            .as_str()
            .unwrap_or("No response")
            .to_string();

        Ok(response)
    }

    /// Simple mock for development/testing without API key
    pub async fn call_mock(&self, _system_prompt: &str, user_input: &str) -> Result<String> {
        Ok(format!(
            "[MOCK LLM] Input received: '{}'. Awaiting real API integration.",
            user_input
        ))
    }
}
