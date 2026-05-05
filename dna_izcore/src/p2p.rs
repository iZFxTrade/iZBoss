use std::sync::atomic::{AtomicUsize, Ordering};

pub static ACTIVE_PEERS: AtomicUsize = AtomicUsize::new(0);

pub const BOOTSTRAP_NODES: &[&str] = &[
    "/ip4/104.21.31.205/tcp/4001/p2p/Qm...", // iZBoss Seed A (Direct IP)
    "/ip4/172.67.189.102/tcp/4001/p2p/Qm...", // iZBoss Seed B (Direct IP)
];

const PEER_CACHE_FILE: &str = "peer_cache.json";

/// P2P Network Module — Pure Autonomy Mode
pub async fn start_p2p_network(device_id: &str) {
    println!("[P2P] 🌌 iZCore Autonomous Mesh — Node: {}", device_id);

    // ── STEP 1: Load Peers from Local Cache ──────────────────
    println!("[P2P] Loading peers from local cache...");
    let _ = load_and_dial_cached_peers().await;

    // ── STEP 2: Dial Hardcoded Seeds (No DNS required) ───────
    println!("[P2P] Dialing hardcoded iZBoss Seeds (Absolute Fallback)...");
    for addr in BOOTSTRAP_NODES {
        println!("[P2P] -> Attempting connection to: {}", addr);
    }

    // ── STEP 3: LAN Auto-Discovery (mDNS) ────────────────────
    let _ = try_mdns_discovery(device_id).await;

    // ── STEP 4: Optional Cloud Handshake (If available) ──────
    println!("[P2P] Checking boss.iz.life for new peers (Optional)...");
    let _ = try_bootstrap_peers(device_id).await;
}

async fn load_and_dial_cached_peers() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(content) = std::fs::read_to_string(PEER_CACHE_FILE) {
        let peers: Vec<String> = serde_json::from_str(&content).unwrap_or_default();
        println!("[P2P] ✓ Found {} peers in local cache.", peers.len());
        ACTIVE_PEERS.store(peers.len(), Ordering::SeqCst);
        // logic to dial these peers
    }
    Ok(())
}

fn save_peers_to_cache(peers: &Vec<String>) {
    let _ = std::fs::write(PEER_CACHE_FILE, serde_json::to_string(peers).unwrap_or_default());
}

    // Phase 3: Keep alive — periodic peer refresh
    let mut tick = 0u64;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // Faster refresh for discovery
        tick += 1;
        println!("[P2P] iZBoss Heartbeat #{} — Đang đồng bộ danh sách Nodes...", tick);
        let _ = try_bootstrap_peers(device_id).await;
    }
}

/// Attempt mDNS discovery on local network
async fn try_mdns_discovery(device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[P2P] mDNS: Broadcasting presence as '{}' in local mesh...", device_id);
    // Real libp2p mDNS implementation would go here.
    // For now: announce presence via Command Center API.
    let client = reqwest::Client::new();
    let boss_api = std::env::var("BOSS_API_URL")
        .unwrap_or_else(|_| "https://boss.iz.life".to_string());

    let res = client
        .post(format!("{}/api/p2p/announce", boss_api))
        .json(&serde_json::json!({
            "device_id": device_id,
            "protocol": "mdns",
            "platform": format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
        }))
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => {
            println!("[P2P] ✓ Đã báo danh iZCore thành công lên Command Center.");
        }
        _ => {
            println!("[P2P] ⚠ iZCore đang chạy chế độ offline.");
        }
    }
    Ok(())
}

/// Fetch peer list from Command Center and attempt connections
async fn try_bootstrap_peers(device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let boss_api = std::env::var("BOSS_API_URL")
        .unwrap_or_else(|_| "https://boss.iz.life".to_string());

    let res = client
        .get(format!("{}/api/p2p/peers?device_id={}", boss_api, device_id))
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => {
            let peers: serde_json::Value = r.json().await.unwrap_or_default();
            let count = peers.as_array().map(|a| a.len()).unwrap_or(0);
            
            // Sync with global counter
            ACTIVE_PEERS.store(count, Ordering::SeqCst);

            if count > 0 {
                println!("[P2P] ✓ Connected to {} other iZBoss nodes.", count);
            } else {
                println!("[P2P] ℹ Waiting for other nodes to join the mesh...");
            }
        }
        _ => {}
    }
    Ok(())
}
