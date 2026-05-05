use sysinfo::{System, SystemExt, CpuExt};
use colored::*;

pub async fn handle_agent_activation() {
    println!("{}", "\n[Agent] 🧠 Khởi động Hệ thống Hỗ trợ Ngôn ngữ Tự nhiên...".cyan().bold());
    
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_ram = sys.total_memory() / 1024 / 1024 / 1024; // in GB
    let cpu_count = sys.cpus().len();

    println!("[Agent] Chẩn đoán tài nguyên: {} CPU Cores | {} GB RAM", cpu_count, total_ram);

    let suggested_model = if total_ram < 2 {
        "Phi-3-mini (3.8B) - Siêu nhẹ, phù hợp thiết bị nhúng"
    } else if total_ram < 8 {
        "Llama-3-8B-Quantized - Cân bằng giữa tốc độ và trí tuệ"
    } else {
        "Llama-3-70B-GGUF - Sức mạnh tối thượng cho Server"
    };

    println!("[Agent] Đề xuất Model phù hợp: {}", suggested_model.green());
    println!("[Agent] ℹ Khi mạng lưới iZBoss đạt > 1000 Nodes, AO (Autonomous Operator) sẽ tự động kích hoạt chế độ tính toán phân tán.");
    
    println!("\n[Agent] Bạn có muốn tiến hành tải và cài đặt {} không? (y/n)", suggested_model);
    // In real implementation, we would wait for input and use candle/llama.cpp to run it
    println!("[Agent] (Chế độ mô phỏng: Đang chuẩn bị môi trường runtime...)");
}
