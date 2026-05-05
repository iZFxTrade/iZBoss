use anyhow::Result;
use crate::iacs::{AgentRole, IacsMessage, IacsMessageType};
use crate::llm_client::LlmClient;

const AO_SYSTEM_PROMPT: &str = r#"
Bạn là AO (Administrative Officer) — Giám đốc Vận hành của hệ thống iZ.Life BOSS.
Nhiệm vụ của bạn:
1. Nhận mục tiêu từ BA và phân bổ xuống các phòng ban.
2. Theo dõi tiến độ thực thi của từng phòng ban.
3. Quản lý tài nguyên Node và Agent.
4. Báo cáo kết quả và rủi ro trở lại BA.

Luôn trả lời ngắn gọn, súc tích, tập trung vào HÀNH ĐỘNG cụ thể.
Mỗi phân công phải có: PHÒNG BAN - AGENT - DEADLINE - KPI.
"#;

/// Task assignment to a specific department
#[derive(Debug, Clone)]
pub struct DeptTask {
    pub dept_id: String,
    pub agent_id: String,
    pub task: String,
    pub deadline: String,
    pub kpi: String,
}

pub struct AdminOfficer {
    llm: LlmClient,
    active_tasks: Vec<DeptTask>,
}

impl AdminOfficer {
    pub fn new(cf_account_id: &str, cf_api_token: &str) -> Self {
        Self {
            llm: LlmClient::new(cf_account_id, cf_api_token),
            active_tasks: Vec::new(),
        }
    }

    /// Nhận GoalAssign từ BA và xử lý
    pub async fn receive_goal(&mut self, msg: &IacsMessage) -> Result<()> {
        if msg.message_type != IacsMessageType::GoalAssign {
            return Ok(());
        }

        let goal = msg.payload["goal"].as_str().unwrap_or("Unknown goal");
        let deadline = msg.payload["deadline"].as_str().unwrap_or("TBD");

        println!("[AO] Nhận GOAL_ASSIGN từ BA: '{}'", goal);
        println!("[AO] Đang phân tích và phân bổ nhiệm vụ xuống phòng ban...");

        let prompt = format!(
            "Goal: {}\nDeadline: {}\n\nPhân bổ nhiệm vụ cụ thể cho từng phòng ban liên quan.",
            goal, deadline
        );

        let plan = self.llm.call_cf_ai(AO_SYSTEM_PROMPT, &prompt).await
            .unwrap_or_else(|_| format!(
                "[AO - Offline] Goal '{}' đã ghi nhận. Đang chờ kết nối LLM để phân bổ.", goal
            ));

        println!("[AO] Kế hoạch phân bổ:\n{}", plan);
        Ok(())
    }

    /// Phân bổ task trực tiếp cho một phòng ban
    pub fn assign_to_dept(&mut self, task: DeptTask) {
        println!(
            "[AO → {}] Giao task cho {}: '{}' | Deadline: {} | KPI: {}",
            task.dept_id, task.agent_id, task.task, task.deadline, task.kpi
        );
        self.active_tasks.push(task);
    }

    /// Tổng hợp báo cáo và gửi về BA
    pub fn generate_status_report(&self) -> IacsMessage {
        let tasks_summary: Vec<serde_json::Value> = self.active_tasks.iter().map(|t| {
            serde_json::json!({
                "dept": t.dept_id,
                "agent": t.agent_id,
                "task": t.task,
                "deadline": t.deadline,
                "status": "in_progress"
            })
        }).collect();

        let payload = serde_json::json!({
            "active_tasks": tasks_summary,
            "total_active": self.active_tasks.len(),
            "summary": format!("{} tasks đang được thực thi trên toàn hệ thống.", self.active_tasks.len())
        });

        let msg = IacsMessage::new(
            AgentRole::AO,
            AgentRole::BA,
            IacsMessageType::StatusReport,
            payload,
        );
        println!("[AO → BA] STATUS_REPORT: {} active tasks", self.active_tasks.len());
        msg
    }
}
