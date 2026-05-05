mod ao;
mod ba;
mod iacs;
mod llm_client;

use ao::{AdminOfficer, DeptTask};
use ba::BossAssistant;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     iZ.Life BOSS — EXECUTIVE INTELLIGENCE    ║");
    println!("║          BA (Strategic Brain) Active         ║");
    println!("║       AO (Operating Director) Active         ║");
    println!("╚══════════════════════════════════════════════╝");

    let cf_account_id = env::var("CF_ACCOUNT_ID").unwrap_or_else(|_| "MOCK_ACCOUNT".to_string());
    let cf_api_token  = env::var("CF_API_TOKEN").unwrap_or_else(|_| "MOCK_TOKEN".to_string());

    // Initialize BA and AO
    let ba = BossAssistant::new(&cf_account_id, &cf_api_token);
    let mut ao = AdminOfficer::new(&cf_account_id, &cf_api_token);

    println!("\n[SYSTEM] BA và AO đã khởi động. Chờ chỉ thị từ Đại K...\n");

    // Demo: BA nhận chỉ thị → tạo GoalAssign → AO nhận và phân bổ
    let demo_input = "Tăng hiệu suất trading tuần này lên 15%. Ưu tiên XAUUSD grid strategy.";

    println!("═══ [Đại K] Chỉ thị: \"{}\"", demo_input);
    println!();

    // BA phân tích
    let _ba_response = ba.process_input(demo_input).await?;
    println!();

    // BA giao việc cho AO
    let goal_msg = ba.assign_goal_to_ao(
        "Tăng hiệu suất trading XAUUSD 15%",
        "Cuối tuần",
        "Phân bổ từ w-inv",
    );

    // AO nhận và xử lý
    ao.receive_goal(&goal_msg).await?;
    println!();

    // AO phân bổ task cụ thể cho phòng ban
    ao.assign_to_dept(DeptTask {
        dept_id: "dept-trading".to_string(),
        agent_id: "L-iZFx (Quants Master)".to_string(),
        task: "Kích hoạt XAUUSD Grid Strategy — lot size 0.01, spacing 50 pips".to_string(),
        deadline: "Cuối tuần".to_string(),
        kpi: "+15% equity từ trading".to_string(),
    });

    ao.assign_to_dept(DeptTask {
        dept_id: "dept-finance".to_string(),
        agent_id: "CFO Alpha".to_string(),
        task: "Theo dõi P&L XAUUSD và cập nhật w-inv balance".to_string(),
        deadline: "Daily".to_string(),
        kpi: "Báo cáo P&L lúc 08:00 mỗi ngày".to_string(),
    });

    println!();

    // AO báo cáo về BA
    let report = ao.generate_status_report();
    ba.receive_status_report(&report);

    println!("\n[SYSTEM] Executive Intelligence cycle hoàn tất.");
    println!("[SYSTEM] Để chạy liên tục: thiết lập systemd service hoặc dùng `boss` CLI.");

    Ok(())
}
