mod auth;
mod bootstrap;
mod fingerprint;
mod ota;
mod p2p;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("======================================");
    println!("      iZ.Life BOSS - TẦNG DNA         ");
    println!("         (iZcore Kernel)              ");
    println!("======================================");

    // 1. Hardware Fingerprinting
    let device_id = fingerprint::generate_device_id();
    println!("[*] Device Fingerprint: {}", device_id);

    // 2. Auth Gate check (ED25519)
    if !auth::verify_master_key() {
        println!("[-] UNAUTHORIZED: Master Key signature required to awaken this Entity.");
        return Ok(());
    }
    println!("[+] Auth Gate: Master Key verified. Entity awakened.");

    // 3. Self-Bootstrap
    bootstrap::connect_to_command_center(&device_id).await?;

    // 4. P2P Handshake (mDNS & Gossip)
    let p2p_handle = tokio::spawn(async {
        p2p::start_p2p_network().await;
    });

    // 5. OTA Listener
    let ota_handle = tokio::spawn(async {
        ota::start_ota_listener().await;
    });

    let _ = tokio::join!(p2p_handle, ota_handle);

    Ok(())
}
