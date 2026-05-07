use serde::{Serialize, Deserialize};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub download_url: String,
    pub sha256: String,
    pub signature: String, // Hex encoded signature from Founder/Root
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteInstallCommand {
    pub manifest: SkillManifest,
    pub timestamp: u64,
    pub target_nodes: Vec<String>, // List of Node IDs or "*" for all
}

pub struct SkillOrchestrator;

impl SkillOrchestrator {
    /// Verify and install a remote skill
    pub async fn remote_install(command_json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let cmd: RemoteInstallCommand = serde_json::from_str(command_json)?;
        
        println!("[Skills] 📡 Nhận lệnh cài đặt từ xa cho Skill: {} (v{})", cmd.manifest.name, cmd.manifest.version);

        // 1. Verify Signature
        if !verify_founder_signature(&cmd)? {
            println!("[Skills] ✗ Chữ ký không hợp lệ! Lệnh cài đặt bị từ chối.");
            return Err("Invalid Founder Signature".into());
        }

        println!("[Skills] ✓ Chữ ký Founder hợp lệ. Đang tiến hành tải xuống...");

        // 2. Download and Install (Mocked for now)
        download_and_extract_skill(&cmd.manifest).await?;

        println!("[Skills] ✅ Đã cài đặt thành công Skill: {}", cmd.manifest.name);
        Ok(())
    }
}

fn verify_founder_signature(cmd: &RemoteInstallCommand) -> Result<bool, Box<dyn std::error::Error>> {
    let founder_addr = option_env!("FOUNDER_WALLET_ADDRESS").unwrap_or("");
    if founder_addr.is_empty() {
        return Ok(false); // No founder set, can't verify
    }

    let pub_key_bytes = hex::decode(founder_addr.trim_start_matches("0x"))?;
    let verifying_key = VerifyingKey::from_bytes(&pub_key_bytes[..32].try_into()?)?;
    
    let sig_bytes = hex::decode(&cmd.manifest.signature)?;
    let signature = Signature::from_bytes(&sig_bytes[..64].try_into()?)?;

    // Message is the concatenation of ID, Version and SHA256
    let message = format!("{}{}{}", cmd.manifest.id, cmd.manifest.version, cmd.manifest.sha256);
    
    match verifying_key.verify(message.as_bytes(), &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

async fn download_and_extract_skill(manifest: &SkillManifest) -> Result<(), Box<dyn std::error::Error>> {
    let skills_dir = Path::new("skills");
    if !skills_dir.exists() {
        fs::create_dir_all(skills_dir)?;
    }

    let dest_path = skills_dir.join(format!("{}_{}.skill", manifest.id, manifest.version));
    
    // In real implementation, use reqwest to download and verify SHA256
    println!("[Skills] 📥 Đang tải gói từ: {}", manifest.download_url);
    fs::write(dest_path, "MOCK_SKILL_CONTENT")?;
    
    Ok(())
}
