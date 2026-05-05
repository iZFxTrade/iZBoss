use sysinfo::System;
use sha2::{Sha256, Digest};
use mac_address::get_mac_address;

/// Generate a stable, unique device fingerprint using hardware identifiers.
/// Hash = SHA256(CPU_Brand + MAC_Address + Disk_Serial)
pub fn generate_device_id() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU brand string
    let cpu_info = sys.cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "UnknownCPU".to_string());

    // Primary MAC address
    let mac_info = get_mac_address()
        .ok()
        .flatten()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "00:00:00:00:00:00".to_string());

    // Disk serial — read from /sys on Linux, use hostname fallback on other OS
    let disk_serial = read_disk_serial();

    let raw_data = format!("{}_{}_{}", cpu_info, mac_info, disk_serial);

    let mut hasher = Sha256::new();
    hasher.update(raw_data.as_bytes());
    let result = hasher.finalize();

    let full_hash = hex::encode(result);

    // Return first 16 chars as short device ID (still unique enough)
    format!("iznode-{}", &full_hash[..16])
}

/// Read disk serial number — platform-specific
fn read_disk_serial() -> String {
    // Linux: try reading from /sys/block/sda/device/serial or similar
    #[cfg(target_os = "linux")]
    {
        let candidates = [
            "/sys/block/sda/device/serial",
            "/sys/block/nvme0n1/device/serial",
            "/sys/block/mmcblk0/device/serial",
        ];
        for path in &candidates {
            if let Ok(serial) = std::fs::read_to_string(path) {
                return serial.trim().to_string();
            }
        }
    }

    // macOS: use system_profiler or IOKit (simplified — use hostname as fallback)
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("system_profiler")
            .args(["SPHardwareDataType"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("Serial Number") {
                    if let Some(serial) = line.split(':').nth(1) {
                        return serial.trim().to_string();
                    }
                }
            }
        }
    }

    // Fallback: hostname
    hostname_fallback()
}

fn hostname_fallback() -> String {
    std::process::Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown-host".to_string())
}
