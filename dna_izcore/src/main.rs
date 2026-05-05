mod auth;
mod bootstrap;
mod fingerprint;
mod ota;
mod p2p;

use std::time::Duration;

const HEARTBEAT_INTERVAL_SECS: u64 = 30;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════╗");
    println!("║     iZ.Life BOSS — iZCore DNA Kernel         ║");
    println!("║          Autonomous Entity Awakening         ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // ── 1. Hardware Fingerprinting ───────────────────────────
    let device_id = fingerprint::generate_device_id();
    println!("[DNA] Device Fingerprint: {}", device_id);

    // ── 2. Auth Gate ─────────────────────────────────────────
    if !auth::verify_master_key() {
        println!("[Auth] ✗ UNAUTHORIZED — Entity stays dormant.");
        return Ok(());
    }
    println!("[Auth] ✓ Entity Awakened.\n");

    // ── 3. Bootstrap: Register with Command Center ───────────
    bootstrap::connect_to_command_center(&device_id).await?;

    // ── 4. Spawn P2P Network (background) ───────────────────
    let p2p_id = device_id.clone();
    let p2p_handle = tokio::spawn(async move {
        p2p::start_p2p_network(&p2p_id).await;
    });

    // ── 5. Spawn OTA Listener (background) ──────────────────
    let ota_handle = tokio::spawn(async {
        ota::start_ota_listener().await;
    });

    // ── 6. Heartbeat Loop — báo danh mỗi 30 giây ────────────
    let hb_id = device_id.clone();
    let heartbeat_handle = tokio::spawn(async move {
        run_heartbeat_loop(&hb_id).await;
    });

    println!("[DNA] All systems active. Heartbeat: {}s | OTA: 60s", HEARTBEAT_INTERVAL_SECS);
    println!("[DNA] Press Ctrl+C to shutdown.\n");

    let _ = tokio::join!(p2p_handle, ota_handle, heartbeat_handle);
    Ok(())
}

/// Heartbeat loop — ping Command Center every 30s to maintain node status
async fn run_heartbeat_loop(device_id: &str) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .unwrap_or_default();

    let boss_api = std::env::var("BOSS_API_URL")
        .unwrap_or_else(|_| "https://boss.iz.life".to_string());

    let mut tick = 0u64;
    loop {
        tokio::time::sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECS)).await;
        tick += 1;

        let payload = serde_json::json!({
            "device_id": device_id,
            "status": "online",
            "platform": format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            "version": env!("CARGO_PKG_VERSION"),
            "tick": tick,
            "timestamp": timestamp_now()
        });

        match client
            .post(format!("{}/api/heartbeat", boss_api))
            .json(&payload)
            .send()
            .await
        {
            Ok(res) if res.status().is_success() => {
                println!("[Heartbeat #{}] ✓ {} — online", tick, device_id);
            }
            Ok(res) => {
                println!("[Heartbeat #{}] ⚠ HTTP {}", tick, res.status());
            }
            Err(e) => {
                println!("[Heartbeat #{}] ✗ Offline — {}", tick, e);
            }
        }
    }
}

fn timestamp_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
