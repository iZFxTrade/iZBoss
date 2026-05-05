use serde::{Deserialize, Serialize};

/// IACS Message Types (Inter-Assistant Communication Schema)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IacsMessageType {
    /// BA -> AO: Giao mục tiêu, ngân sách, thời hạn
    GoalAssign,
    /// BA -> AO: Hỏi về mạng lưới, tài nguyên
    ResourceQuery,
    /// AO -> BA: Báo cáo tiến độ và rủi ro từ phòng ban
    StatusReport,
    /// AO -> BA: Xác nhận hoạt động đã được khởi tạo
    ActionConfirm,
}

/// IACS Message Envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IacsMessage {
    pub id: String,
    pub from: AgentRole,
    pub to: AgentRole,
    pub message_type: IacsMessageType,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

/// Agent Roles in the Executive Layer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentRole {
    BA,  // Boss Assistant — Strategic Brain
    AO,  // Administrative Officer — Operating Director
}

impl IacsMessage {
    pub fn new(
        from: AgentRole,
        to: AgentRole,
        message_type: IacsMessageType,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: format!("iacs-{}", uuid_simple()),
            from,
            to,
            message_type,
            payload,
            timestamp: chrono_now(),
        }
    }
}

/// Generate a simple pseudo-UUID using timestamp
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    format!("{:x}", t)
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", t)
}
