use sysinfo::System;
use sha2::{Sha256, Digest};
use mac_address::get_mac_address;

pub fn generate_device_id() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_info = sys.cpus().first().map(|c| c.brand()).unwrap_or("UnknownCPU");
    let mac_info = get_mac_address().unwrap_or(None).map(|m| m.to_string()).unwrap_or_else(|| "00:00:00:00:00:00".to_string());
    let disk_serial = "DISK_SERIAL_MOCK";

    let raw_data = format!("{}_{}_{}", cpu_info, mac_info, disk_serial);
    let mut hasher = Sha256::new();
    hasher.update(raw_data.as_bytes());
    let result = hasher.finalize();
    
    hex::encode(result)
}
