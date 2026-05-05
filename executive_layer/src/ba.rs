use anyhow::Result;
use crate::iacs::{AgentRole, IacsMessage, IacsMessageType};
use crate::llm_client::LlmClient;

const BA_SYSTEM_PROMPT: &str = r#"
Bạn là BA (Boss Assistant) — Bộ não chiến lược của hệ thống iZ.Life BOSS.
Nhiệm vụ của bạn:
1. Lắng nghe và phân tích ý tưởng từ Đại K (BOSS HƯNG).
2. Thiết lập OKRs và mục tiêu rõ ràng.
3. Tổng hợp báo cáo từ AO và các phòng ban.
4. Đưa ra quyết định chiến lược và giao nhiệm vụ cho AO.

Luôn trả lời ngắn gọn, súc tích, chuyên nghiệp bằng tiếng Việt.
Mọi quyết định phải có: MỤC TIÊU - THỜI HẠN - THƯỚC ĐO THÀNH CÔNG.
"#;

pub struct BossAssistant {
    llm: LlmClient,
    message_queue: Vec<IacsMessage>,
}

impl BossAssistant {
    pub fn new(cf_account_id: &str, cf_api_token: &str) -> Self {
        Self {
            llm: LlmClient::new(cf_account_id, cf_api_token),
            message_queue: Vec::new(),
        }
    }

    /// Nhận input từ Đại K và phân tích chiến lược
    pub async fn process_input(&self, input: &str) -> Result<String> {
        println!("[BA] Đang phân tích chỉ thị từ Đại K...");
        let response = self.llm.call_cf_ai(BA_SYSTEM_PROMPT, input).await
            .unwrap_or_else(|_| self.fallback_response(input));
        println!("[BA] → {}", response);
        Ok(response)
    }

    /// Tạo IACS GoalAssign message gửi xuống AO
    pub fn assign_goal_to_ao(&self, goal: &str, deadline: &str, budget: &str) -> IacsMessage {
        let payload = serde_json::json!({
            "goal": goal,
            "deadline": deadline,
            "budget": budget,
            "priority": "high"
        });
        let msg = IacsMessage::new(
            AgentRole::BA,
            AgentRole::AO,
            IacsMessageType::GoalAssign,
            payload,
        );
        println!("[BA → AO] GOAL_ASSIGN: {}", goal);
        msg
    }

    /// Xử lý báo cáo từ AO
    pub fn receive_status_report(&self, report: &IacsMessage) {
        if report.message_type == IacsMessageType::StatusReport {
            println!("[BA] Nhận STATUS_REPORT từ AO: {:?}", report.payload);
        }
    }

    fn fallback_response(&self, input: &str) -> String {
        format!(
            "[BA - Offline Mode] Chỉ thị đã ghi nhận: '{}'. Hệ thống LLM chưa kết nối — đang chờ Cloudflare API token.",
            input
        )
    }
}
