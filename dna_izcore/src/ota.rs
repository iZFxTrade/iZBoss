use serde::Deserialize;

const OTA_CHECK_INTERVAL_SECS: u64 = 60;
const OTA_MANIFEST_URL: &str = "https://boss.iz.life/api/ota/latest";

#[derive(Debug, Deserialize)]
struct OtaManifest {
    version: String,
    platform: String,
    download_url: String,
    sha256: String,
}

/// OTA Listener — polls boss.iz.life every 60s for new binary versions.
pub async fn start_ota_listener() {
    println!("[OTA] Listener đã khởi động — kiểm tra cập nhật mỗi {}s.", OTA_CHECK_INTERVAL_SECS);

    let current_version = env!("CARGO_PKG_VERSION");
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    let client = reqwest::Client::new();

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(OTA_CHECK_INTERVAL_SECS)).await;

        match check_for_update(&client, current_version, &platform).await {
            Ok(Some(manifest)) => {
                println!("[OTA] 🔄 Phát hiện phiên bản mới: {} → {}", current_version, manifest.version);
                println!("[OTA] 📥 Download URL: {}", manifest.download_url);
                // TODO: Download, verify SHA256, replace binary, restart
                println!("[OTA] ⚠ Auto-update chưa được kích hoạt — cần cấu hình thêm.");
            }
            Ok(None) => {
                // Already on latest, silent
            }
            Err(e) => {
                println!("[OTA] ✗ Không kiểm tra được bản cập nhật: {}", e);
            }
        }
    }
}

async fn check_for_update(
    client: &reqwest::Client,
    current_version: &str,
    platform: &str,
) -> Result<Option<OtaManifest>, reqwest::Error> {
    let url = format!("{}?platform={}", OTA_MANIFEST_URL, platform);
    let res = client.get(&url).send().await?;

    if !res.status().is_success() {
        return Ok(None);
    }

    let manifest: OtaManifest = match res.json().await {
        Ok(m) => m,
        Err(_) => return Ok(None),
    };

    if manifest.version != current_version {
        Ok(Some(manifest))
    } else {
        Ok(None)
    }
}
