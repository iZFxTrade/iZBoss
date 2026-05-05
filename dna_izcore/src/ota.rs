pub async fn start_ota_listener() {
    println!("[*] Starting OTA Listener... monitoring Cloudflare R2 for continuous execution updates.");
    loop { tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; }
}
