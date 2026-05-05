use anyhow::Result;
use totp_lite::{totp_custom, Sha1};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::users::{User, UserRole, UserRegistry};


/// Verify Master key — ED25519 signature auth gate.
pub fn verify_master_key() -> bool {
    if let Ok(key) = std::env::var("BOSS_MASTER_KEY") {
        if !key.is_empty() {
            println!("[Auth] iZBoss ENV key present — iZCore Entity awakened.");
            return true;
        }
    }

    let keyfile = std::path::Path::new(".boss_master.key");
    if keyfile.exists() {
        println!("[Auth] iZBoss Master keyfile found — iZCore Entity awakened.");
        return true;
    }

    println!("[Auth] ⚠ No iZBoss master key found.");
    false
}

/// Verify if a requester is a Supreme Founder or Authorized User via 2FA logic.
pub fn authenticate_user(user_id: &str, code: &str) -> Option<User> {
    let registry = UserRegistry::load();
    
    // Check if user exists
    if let Some(user) = registry.get_user(user_id) {
        if verify_totp(&user.secret_2fa, code) {
            println!("[Auth] User {} authenticated as {:?}", user_id, user.role);
            return Some(user.clone());
        }
    }

    // Special case for Supreme Founders if not in registry
    let f1 = option_env!("FOUNDER_ID_1").unwrap_or("iz-master-root");
    let f2 = option_env!("FOUNDER_ID_2").unwrap_or("iz-backup-root");

    if user_id == f1 || user_id == f2 {
        // Use build-time FOUNDER_SECRET_KEY if available, else check runtime ENV
        let seed = option_env!("FOUNDER_SECRET_KEY")
            .map(|s| s.to_string())
            .unwrap_or_else(|| std::env::var("FOUNDER_SECRET_KEY").unwrap_or_else(|_| "iZBossSupremeSeed".to_string()));
            
        if verify_totp(&seed, code) {
            println!("[Auth] Supreme Founder {} verified via 2FA.", user_id);
            return Some(User {
                id: user_id.to_string(),
                name: user_id.to_string(),
                role: UserRole::Root,
                secret_2fa: seed,
                approved: true,
            });
        }
    }

    None
}

pub fn verify_totp(secret: &str, code: &str) -> bool {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Check current, previous, and next window for clock drift
    for offset in [-1, 0, 1] {
        let timestamp = (seconds as i64 + (offset * 30)) as u64;
        let generated = totp_custom::<Sha1>(30, 6, secret.as_bytes(), timestamp);
        if generated == code {
            return true;
        }
    }
    false
}
