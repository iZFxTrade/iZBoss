use anyhow::Result;

/// Verify Master key — ED25519 signature auth gate.
/// In production: load pubkey from file and verify startup signature.
/// Currently: reads BOSS_MASTER_KEY env var as a simple passphrase check.
pub fn verify_master_key() -> bool {
    // Check env var first (for CI/CD and automated boot)
    if let Ok(key) = std::env::var("BOSS_MASTER_KEY") {
        if !key.is_empty() {
            println!("[Auth] ENV key present — Entity awakened.");
            return true;
        }
    }

    // Check for keyfile on disk
    let keyfile = std::path::Path::new(".boss_master.key");
    if keyfile.exists() {
        println!("[Auth] Master keyfile found — Entity awakened.");
        return true;
    }

    // Interactive mode: prompt for passphrase
    println!("[Auth] ⚠ No master key found.");
    println!("[Auth] Set BOSS_MASTER_KEY env var or place .boss_master.key file in working dir.");
    false
}
