use anyhow::Result;

const REGISTER_ENDPOINT: &str = "https://boss.iz.life/api/register";

/// Self-bootstrap: register this device with the Command Center.
pub async fn connect_to_command_center(device_id: &str, username: Option<&str>) -> Result<()> {
    println!("[Bootstrap] Đang kết nối về Command Center: {}", REGISTER_ENDPOINT);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let payload = serde_json::json!({
        "device_id": device_id,
        "version": env!("CARGO_PKG_VERSION"),
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "username": username,
        "timestamp": chrono_ts()
    });

    match client.post(REGISTER_ENDPOINT).json(&payload).send().await {
        Ok(res) if res.status().is_success() => {
            println!("[Bootstrap] ✓ Đã đăng ký thành công với Command Center.");
        }
        Ok(res) => {
            println!("[Bootstrap] ⚠ Command Center phản hồi: HTTP {}", res.status());
            println!("[Bootstrap] → Kernel tiếp tục chạy ở chế độ standalone.");
        }
        Err(e) => {
            println!("[Bootstrap] ✗ Không kết nối được: {}", e);
            println!("[Bootstrap] → Kernel tiếp tục chạy ở chế độ offline.");
        }
    }

    Ok(())
}

fn chrono_ts() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
