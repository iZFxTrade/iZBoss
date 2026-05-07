mod auth;
mod bootstrap;
mod fingerprint;
mod ota;
mod p2p;
mod dashboard;
mod users;
mod agent;
mod evolution;
mod wallet;

use clap::{Parser, Subcommand};
use std::time::Duration;
use crate::users::{User, UserRole, UserRegistry};

#[derive(Parser)]
#[command(name = "iZCore")]
#[command(about = "iZ.Life BOSS — The DNA Kernel", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Initialize the first Root/Founder user on this node
    #[arg(long)]
    init: bool,

    /// Set username for this node installation
    #[arg(long)]
    username: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Access system controls (requires 2FA)
    Control {
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        code: String,
    },
    /// Manage users (Admin/Root only)
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
    /// Manage AI Agents & LLM
    Agent {
        #[command(subcommand)]
        action: AgentCommands,
    },
    /// Manage iZWallet (Autonomous Finance)
    Wallet {
        #[command(subcommand)]
        action: WalletCommands,
    },
    /// Trigger system self-evolution cycle
    Evolve,
    /// Show node status
    Status,
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Turn on the local AI Agent assistant
    On,
}

#[derive(Subcommand)]
enum UserCommands {
    /// Add a new user to the network
    Add {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        role: String, // root, admin, mod, user
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Initialize a new native wallet
    Init,
    /// View transaction history
    History,
    /// Pay to another User ID
    Pay { to: String, amount: f64, note: Option<String> },
    /// Send to external address
    Send { to: String, amount: f64, memo: Option<String> },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let device_id = fingerprint::generate_device_id();

    // ── 0. CLI Mode Handling ──────────────────────────────────
    if let Some(command) = cli.command {
        match command {
            Commands::Control { user, code } => {
                handle_control_mode(&user, &code).await?;
                return Ok(());
            }
            Commands::User { action } => {
                handle_user_command(action).await?;
                return Ok(());
            }
            Commands::Agent { action } => {
                match action {
                    AgentCommands::On => agent::handle_agent_activation().await,
                }
                return Ok(());
            }
            Commands::Wallet { action } => {
                match action {
                    WalletCommands::Init => { let _ = wallet::iZWallet::init(); }
                    WalletCommands::History => wallet::show_history().await,
                    WalletCommands::Pay { to, amount, note } => println!("P2P Pay: {} to {} | Note: {:?}", amount, to, note),
                    WalletCommands::Send { to, amount, memo } => println!("External Send: {} to {} | Memo: {:?}", amount, to, memo),
                }
                return Ok(());
            }
            Commands::Evolve => {
                evolution::handle_evolution_cycle().await;
                return Ok(());
            }
            Commands::Status => {
                println!("[DNA] Node Status: Active | ID: {}", device_id);
                return Ok(());
            }
        }
    }

    if cli.init {
        handle_init_mode().await?;
        return Ok(());
    }

    // ── 1. Interactive Entry (Default) ───────────────────────
    let registry = UserRegistry::load();
    if registry.users.is_empty() {
        // No users in local registry? This node might be unclaimed or just wiped.
        // But we check if there's a "local owner" set.
        handle_interactive_onboarding(&device_id).await?;
        return Ok(());
    }

    // ── 2. Authenticated Dashboard Entry ─────────────────────
    // If we reach here, the node has users. We assume the primary user is 'owner' or 'root'.
    println!("iZCore Terminal Entry. Please identify yourself.");
    print!("User ID: ");
    use std::io::{self, Write};
    io::stdout().flush()?;
    let mut user_id = String::new();
    io::stdin().read_line(&mut user_id)?;
    let user_id = user_id.trim();

    print!("Enter 2FA Code: ");
    io::stdout().flush()?;
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    if let Some(user) = auth::authenticate_user(user_id, code) {
        display_ecosystem_dashboard(&user, &device_id).await?;
        
        println!("[DNA] Transitioning to background services...");
        run_background_services(device_id).await?;
    } else {
        println!("[Auth] ✗ Access Denied.");
        return Ok(());
    }

    Ok(())
}

async fn handle_interactive_onboarding(device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[iZCore] ⚠ No owner detected for this node.");
    println!("[iZCore] Starting Auto-Onboarding...");
    
    print!("Please enter your desired Username: ");
    use std::io::{self, Write};
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    // Generate 2FA Secret
    let secret = hex::encode(device_id.as_bytes()).get(0..16).unwrap().to_string();
    
    println!("\n[iZCore] 🔐 Setup 2FA: Scan this QR or enter code in your app.");
    println!("[iZCore] SECRET: {}", secret);
    println!("[iZCore] QR: https://www.google.com/chart?chs=200x200&chld=M|0&cht=qr&chl=otpauth://totp/iZCore:{}?secret={}&issuer=iZBoss", username, secret);
    
    print!("\nPlease enter 2FA Code to verify and link node: ");
    io::stdout().flush()?;
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    if auth::verify_totp(&secret, code) {
        let mut registry = UserRegistry::load();
        let new_owner = User {
            id: username.clone(),
            name: username.clone(),
            role: crate::users::UserRole::Contributor, // Node Contributor (Owner)
            secret_2fa: secret,
            approved: true, // Auto-approved locally as owner
        };
        registry.add_user(new_owner.clone());
        registry.save()?;

        // Register with Command Center to claim ownership globally
        bootstrap::connect_to_command_center(device_id, Some(&username)).await?;
        
        println!("\n[✓] Node successfully linked to '{}'.", username);
        display_ecosystem_dashboard(&new_owner, device_id).await?;
    } else {
        println!("[✗] Verification failed. Onboarding aborted.");
    }
    Ok(())
}

async fn display_ecosystem_dashboard(user: &User, _device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[iZCore] ✓ Access Granted. Awakening TPU Dashboard...");
    tokio::time::sleep(Duration::from_millis(800)).await;
    
    // Launch TUI Dashboard
    if let Err(e) = dashboard::run_dashboard(user).await {
        println!("[iZCore] ✗ Dashboard Error: {}", e);
    }
    
    Ok(())
}
async fn run_background_services(device_id: String) -> Result<(), Box<dyn std::error::Error>> {
    // ── 4. Spawn P2P Network (background) ───────────────────
    let p2p_id = device_id.clone();
    let p2p_handle = tokio::spawn(async move {
        p2p::start_p2p_network(&p2p_id).await;
    });

    // ── 5. Spawn OTA Listener (background) ──────────────────
    let ota_handle = tokio::spawn(async {
        ota::start_ota_listener().await;
    });

    // ── 6. Heartbeat Loop — báo danh mỗi 30 giây ────────────
    let hb_id = device_id.clone();
    let heartbeat_handle = tokio::spawn(async move {
        run_heartbeat_loop(&hb_id).await;
    });

    println!("[DNA] All systems active. Heartbeat: {}s | OTA: 60s", HEARTBEAT_INTERVAL_SECS);
    println!("[DNA] Press Ctrl+C to shutdown.\n");

    let _ = tokio::join!(p2p_handle, ota_handle, heartbeat_handle);
    Ok(())
}

async fn handle_init_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Init] 🧬 iZCore Initialization — Setting up first Root user...");
    
    let mut registry = UserRegistry::load();
    if !registry.users.is_empty() {
        println!("[Init] ⚠ System already initialized. Use 'izcore control' to manage users.");
        return Ok(());
    }

    // Generate random secret for 2FA
    let secret = hex::encode(fingerprint::generate_device_id().as_bytes()).get(0..16).unwrap().to_string();
    let root = User {
        id: "root".to_string(),
        name: "Founder Node".to_string(),
        role: UserRole::Root,
        secret_2fa: secret.clone(),
        approved: true,
    };

    registry.add_user(root);
    registry.save()?;

    println!("\n[Init] ✓ First Root User Created: 'root'");
    println!("[Init] 🔐 YOUR 2FA SECRET: {}", secret);
    println!("[Init] scan this in Google Authenticator or use the code above.");
    println!("[Init] (Simulated QR: https://www.google.com/chart?chs=200x200&chld=M|0&cht=qr&chl=otpauth://totp/iZCore:root?secret={}&issuer=iZBoss)", secret);
    println!("\n[Init] ✓ Initialization complete. Run 'izcore' to start the kernel.\n");
    Ok(())
}

async fn handle_control_mode(user_id: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(user) = auth::authenticate_user(user_id, code) {
        println!("\n╔══════════════════════════════════════════════╗");
        println!("║       iZCore INTERACTIVE CONTROL MODE        ║");
        println!("║   Authenticated: {} (Role: {:?})   ║", user.id, user.role);
        println!("╚══════════════════════════════════════════════╝\n");
        
        // Interactive CLI loop would go here
        println!("[Control] Ready for commands. Type 'help' for options.");
        println!("[Control] (Current simulation: No commands implemented yet. Exiting.)");
    } else {
        println!("[Auth] ✗ Authentication Failed.");
    }
    Ok(())
}

async fn handle_user_command(action: UserCommands) -> Result<(), Box<dyn std::error::Error>> {
    // Note: In real usage, this should check for a valid session or require a master key
    match action {
        UserCommands::Add { id, role } => {
            let mut registry = UserRegistry::load();
            let user_role = match role.to_lowercase().as_str() {
                "root" => UserRole::Root,
                "admin" => UserRole::Admin,
                "mod" | "moderator" => UserRole::Mod,
                _ => UserRole::Contributor,
            };

            let secret = hex::encode(id.as_bytes()).get(0..16).unwrap().to_string();
            let new_user = User {
                id: id.clone(),
                name: id.clone(),
                role: user_role,
                secret_2fa: secret.clone(),
                approved: false, // Needs approval from Admin/Root
            };

            registry.add_user(new_user);
            registry.save()?;
            println!("[User] ✓ Added user '{}' with role {:?}. Approved: false.", id, role);
            println!("[User] 🔐 USER 2FA SECRET: {}", secret);
        }
    }
    Ok(())
}

const HEARTBEAT_INTERVAL_SECS: u64 = 30;

/// Heartbeat loop — ping Command Center every 30s to maintain node status
async fn run_heartbeat_loop(device_id: &str) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(8))
        .build()
        .unwrap_or_default();

    let boss_api = std::env::var("BOSS_API_URL")
        .unwrap_or_else(|_| "https://boss.iz.life".to_string());

    let mut tick = 0u64;
    loop {
        tokio::time::sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECS)).await;
        tick += 1;

        // Periodic User Sync
        if tick % 10 == 0 {
            let mut registry = UserRegistry::load();
            let _ = registry.sync_from_command_center().await;
        }

        let payload = serde_json::json!({
            "device_id": device_id,
            "status": "online",
            "platform": format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            "version": env!("CARGO_PKG_VERSION"),
            "tick": tick,
            "timestamp": timestamp_now()
        });

        let _ = client.post(format!("{}/api/heartbeat", boss_api)).json(&payload).send().await;
    }
}

fn timestamp_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}
