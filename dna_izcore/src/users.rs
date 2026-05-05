use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Root,      // Founder: Complete control
    Admin,     // Business management, secondary authorizations
    Moderator, // Read-only access
    User,      // Standard user: owns their own node but needs approval
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub role: UserRole,
    pub secret_2fa: String, // TOTP shared secret
    pub approved: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserRegistry {
    pub users: HashMap<String, User>,
}

impl UserRegistry {
    pub fn load() -> Self {
        let path = Path::new("iZCore_users.json");
        if path.exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("iZCore_users.json", data)
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub async fn sync_from_command_center(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let boss_api = std::env::var("BOSS_API_URL")
            .unwrap_or_else(|_| "https://boss.iz.life".to_string());
        
        let client = reqwest::Client::new();
        let res = client.get(format!("{}/api/users/sync", boss_api)).send().await?;
        
        if res.status().is_success() {
            let data: serde_json::Value = res.json().await?;
            if let Some(users_array) = data["users"].as_array() {
                for u in users_array {
                    let user: User = serde_json::from_value(u.clone())?;
                    self.users.insert(user.id.clone(), user);
                }
                self.save()?;
                println!("[Sync] ✓ Updated user registry ({} users).", self.users.len());
            }
        }
        Ok(())
    }
}
