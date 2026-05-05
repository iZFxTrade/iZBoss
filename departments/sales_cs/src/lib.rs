/// Sales & CS Department — Phòng Sales & Chăm Sóc Khách Hàng
/// Manages chatbot interactions, customer funnel, and whitelist access control.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub telegram_id: String,
    pub username: String,
    pub stage: FunnelStage,
    pub joined_at: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunnelStage {
    Lead,       // Tiếp cận lần đầu
    Interested, // Đã hỏi về sản phẩm
    Trial,      // Đang dùng thử
    Customer,   // Khách hàng chính thức
    Churned,    // Đã rời đi
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesReport {
    pub total_leads: usize,
    pub active_customers: usize,
    pub churned: usize,
    pub conversion_rate: f64,
    pub top_funnel_stage: String,
}

pub struct SalesAgent {
    pub customers: Vec<Customer>,
    pub whitelist: Vec<String>, // Telegram usernames approved
}

impl SalesAgent {
    pub fn new() -> Self {
        Self {
            customers: Vec::new(),
            whitelist: vec![
                "iZFxTrade".to_string(),
                "FxBlueNet".to_string(),
            ],
        }
    }

    /// Check if a Telegram user is in the whitelist
    pub fn is_whitelisted(&self, username: &str) -> bool {
        self.whitelist.iter().any(|w| w.eq_ignore_ascii_case(username))
    }

    /// Add customer to funnel
    pub fn add_lead(&mut self, telegram_id: &str, username: &str) {
        if self.customers.iter().any(|c| c.telegram_id == telegram_id) {
            return; // Already exists
        }
        self.customers.push(Customer {
            telegram_id: telegram_id.to_string(),
            username: username.to_string(),
            stage: FunnelStage::Lead,
            joined_at: timestamp_now(),
            notes: String::new(),
        });
        println!("[Sales] 🆕 Lead mới: @{}", username);
    }

    /// Advance customer through the funnel
    pub fn advance_stage(&mut self, telegram_id: &str, new_stage: FunnelStage) {
        if let Some(c) = self.customers.iter_mut().find(|c| c.telegram_id == telegram_id) {
            println!("[Sales] ↑ @{}: {:?} → {:?}", c.username, c.stage, new_stage);
            c.stage = new_stage;
        }
    }

    /// Generate sales & CS report
    pub fn generate_report(&self) -> SalesReport {
        let active = self.customers.iter().filter(|c| c.stage == FunnelStage::Customer).count();
        let churned = self.customers.iter().filter(|c| c.stage == FunnelStage::Churned).count();
        let total = self.customers.len();
        SalesReport {
            total_leads: total,
            active_customers: active,
            churned,
            conversion_rate: if total > 0 { active as f64 / total as f64 * 100.0 } else { 0.0 },
            top_funnel_stage: "Lead".to_string(),
        }
    }
}

fn timestamp_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().to_string()
}

impl Default for SalesAgent {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_whitelist() {
        let agent = SalesAgent::new();
        assert!(agent.is_whitelisted("iZFxTrade"));
        assert!(!agent.is_whitelisted("unknown_user"));
    }
}
