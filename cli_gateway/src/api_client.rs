use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_API_URL: &str = "https://boss.iz.life";

/// HTTP client for communicating with boss.iz.life API
pub struct ApiClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeData {
    pub id: String,
    pub name: String,
    pub status: String,
    pub role: String,
    pub cpu_info: Option<String>,
    pub ram_total: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AgentData {
    pub id: String,
    pub name: String,
    pub status: String,
    pub skill: Option<String>,
    pub dept_name: Option<String>,
    pub progress: Option<i64>,
    pub current_task: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SystemData {
    pub nodes: Vec<NodeData>,
    pub agents: Vec<AgentData>,
    pub core: CoreInfo,
}

#[derive(Debug, Deserialize)]
pub struct CoreInfo {
    pub version: String,
    pub status: String,
    pub sync: String,
}

impl ApiClient {
    pub fn new() -> Self {
        let base_url = std::env::var("BOSS_API_URL")
            .unwrap_or_else(|_| DEFAULT_API_URL.to_string());
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Fetch all system data from /api/data
    pub async fn fetch_system_data(&self) -> Result<SystemData> {
        let url = format!("{}/api/data", self.base_url);
        let res = self.client.get(&url).send().await?;
        let data: SystemData = res.json().await?;
        Ok(data)
    }
}
