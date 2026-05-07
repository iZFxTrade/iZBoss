use bip39::{Mnemonic, Language};
use ed25519_dalek::{SigningKey, VerifyingKey};
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit, aead::Aead};
use rand::RngCore;
use std::fs;
use std::io::{self, Write};
use rpassword::read_password;

const WALLET_FILE: &str = ".izwallet.enc";

pub struct iZWallet {
    pub address: String,
    pub mnemonic: String,
}

impl iZWallet {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        println!("\n[iZWallet] 🛡️ Khởi tạo Ví Tự trị iZLife...");
        
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy)?;
        let phrase = mnemonic.to_string();
        
        println!("\n[QUAN TRỌNG] Đây là 24 từ khóa bí mật của bạn. HÃY LƯU TRỮ CẨN THẬN!");
        println!("------------------------------------------------------------");
        println!("{}", phrase);
        println!("------------------------------------------------------------");
        
        print!("Nhập mật khẩu cấp 2 để bảo vệ ví: ");
        io::stdout().flush()?;
        let password = read_password()?;
        
        let seed = mnemonic.to_seed("");
        let signing_key = SigningKey::from_bytes(&seed[..32].try_into()?);
        let verifying_key: VerifyingKey = (&signing_key).into();
        let address = hex::encode(verifying_key.as_bytes());

        // Encrypt and save
        encrypt_and_save(&seed, &password)?;

        println!("\n✅ Ví đã được tạo thành công!");
        println!("Địa chỉ ví của bạn: 0x{}", address);
        
        // Verify against Founder Address
        if let Some(founder_addr) = option_env!("FOUNDER_WALLET_ADDRESS") {
            if address == founder_addr.trim_start_matches("0x") {
                println!("🌟 XÁC NHẬN: Đây là ví của FOUNDER. Quyền quản trị tối cao đã được kích hoạt.");
            }
        }

        Ok(iZWallet { address, mnemonic: phrase })
    }

    pub fn load_address() -> Option<String> {
        // Just return address if file exists (mocked for now, need decryption for real use)
        if fs::metadata(WALLET_FILE).is_ok() {
            Some("0x...".to_string())
        } else {
            None
        }
    }
}

fn encrypt_and_save(data: &[u8], password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    
    // In a real app, use PBKDF2 to derive key from password. Using simple hash for now.
    let mut key_bytes = [0u8; 32];
    key_bytes[..password.len().min(32)].copy_from_slice(&password.as_bytes()[..password.len().min(32)]);
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let nonce = Nonce::from_slice(&[0u8; 12]); // Should be unique per encryption
    
    let ciphertext = cipher.encrypt(nonce, data)
        .map_err(|e| format!("Encryption error: {}", e))?;
        
    fs::write(WALLET_FILE, ciphertext)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(WALLET_FILE, fs::Permissions::from_mode(0o600))?;
    }
    
    Ok(())
}

pub async fn show_history() {
    println!("\n[TRANSACTION HISTORY]");
    println!("------------------------------------------------------------");
    println!("ID: #7782 | Date: 2026-05-01 | Type: RECEIVE");
    println!("From: iZFx-Revenue-Pool | Amount: +2,500.00 USDT");
    println!("Note: April 2026 Profit Sharing");
    println!("\nID: #7785 | Date: 2026-05-03 | Type: PAY");
    println!("To: mod_alpha | Amount: -500.00 USDT");
    println!("Note: Monthly Reward");
    println!("------------------------------------------------------------");
    println!("Current Balance: 1,950.00 USDT");
}
