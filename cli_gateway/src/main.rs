mod api_client;

use api_client::ApiClient;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "boss",
    about = "iZ.Life BOSS — Supreme CLI Interface",
    version = "0.1.0",
    long_about = "Điều phối toàn bộ hệ thống iZ.Life BOSS từ Terminal.\nĐại K có thể xem status, quản lý Agents/Nodes và giao lệnh cho BA."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Xem tổng quan hệ thống (Nodes, Agents, Core status)
    Status,
    /// Quản lý Agents trong hệ thống
    Agent {
        #[command(subcommand)]
        action: AgentAction,
    },
    /// Xem Node Fleet và heartbeat
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
}

#[derive(Subcommand)]
enum AgentAction {
    /// Liệt kê tất cả Agents
    List,
}

#[derive(Subcommand)]
enum NodeAction {
    /// Xem toàn bộ Node Fleet
    Fleet,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let api = ApiClient::new();

    match cli.command {
        Commands::Status => cmd_status(&api).await,
        Commands::Agent { action } => match action {
            AgentAction::List => cmd_agent_list(&api).await,
        },
        Commands::Node { action } => match action {
            NodeAction::Fleet => cmd_node_fleet(&api).await,
        },
    }
}

async fn cmd_status(api: &ApiClient) {
    println!("{}", "╔══════════════════════════════════╗".cyan());
    println!("{}", "║   iZ.Life BOSS — System Status   ║".cyan());
    println!("{}", "╚══════════════════════════════════╝".cyan());

    match api.fetch_system_data().await {
        Ok(data) => {
            println!("\n{} {}", "◆ Core Version:".bold(), data.core.version.green());
            println!("{} {}", "◆ Status:".bold(), data.core.status.yellow());
            println!("{} {}", "◆ Last Sync:".bold(), data.core.sync);

            let online_nodes = data.nodes.iter().filter(|n| n.status == "online").count();
            let active_agents = data.agents.iter().filter(|a| a.status == "running" || a.status == "online").count();

            println!("\n{}", "── Fleet Overview ──────────────".dimmed());
            println!("  {} {}/{} online", "🖥  Nodes:".bold(), online_nodes.to_string().green(), data.nodes.len());
            println!("  {} {}/{} active", "🤖 Agents:".bold(), active_agents.to_string().green(), data.agents.len());
        }
        Err(e) => {
            println!("{} {}", "✗ Cannot connect to boss.iz.life:".red().bold(), e);
            println!("{}", "  → Set BOSS_API_URL env var or check your connection.".dimmed());
        }
    }
}

async fn cmd_agent_list(api: &ApiClient) {
    println!("{}", "╔══════════════════════════════════╗".cyan());
    println!("{}", "║     iZ.Life BOSS — Agents        ║".cyan());
    println!("{}", "╚══════════════════════════════════╝\n".cyan());

    match api.fetch_system_data().await {
        Ok(data) => {
            for agent in &data.agents {
                let status_colored = match agent.status.as_str() {
                    "running" | "online" => agent.status.green(),
                    "idle"               => agent.status.yellow(),
                    _                    => agent.status.red(),
                };
                let dept = agent.dept_name.as_deref().unwrap_or("—");
                let task = agent.current_task.as_deref().unwrap_or("—");
                let progress = agent.progress.unwrap_or(0);

                println!("  {} [{}]", agent.name.bold(), status_colored);
                println!("     Dept: {} | Progress: {}%", dept, progress);
                println!("     Task: {}", task.dimmed());
                println!();
            }
        }
        Err(e) => println!("{} {}", "✗ Error:".red().bold(), e),
    }
}

async fn cmd_node_fleet(api: &ApiClient) {
    println!("{}", "╔══════════════════════════════════╗".cyan());
    println!("{}", "║   iZ.Life BOSS — Node Fleet      ║".cyan());
    println!("{}", "╚══════════════════════════════════╝\n".cyan());

    match api.fetch_system_data().await {
        Ok(data) => {
            for node in &data.nodes {
                let status_colored = if node.status == "online" {
                    node.status.green()
                } else {
                    node.status.red()
                };
                let cpu = node.cpu_info.as_deref().unwrap_or("Unknown");
                let ram_gb = node.ram_total.unwrap_or(0) / 1024;

                println!("  {} [{}]", node.name.bold(), status_colored);
                println!("     Role: {} | CPU: {} | RAM: {}GB", node.role, cpu, ram_gb);
                println!();
            }
        }
        Err(e) => println!("{} {}", "✗ Error:".red().bold(), e),
    }
}
