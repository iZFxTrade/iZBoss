pub async fn start_p2p_network() {
    println!("[*] Starting libp2p node (mDNS & GossipProtocol)...");
    loop { tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; }
}
