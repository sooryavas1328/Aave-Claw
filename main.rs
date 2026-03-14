mod aave;
mod config;
mod monitor;
mod notifier;
mod openclaw;
mod ui;

use clap::{Parser, Subcommand};
use colored::*;
use tracing::info;

#[derive(Parser)]
#[command(
    name = "aave-claw",
    version = env!("CARGO_PKG_VERSION"),
    about = "⚡ High-performance Aave DeFi bot — liquidation monitor, yield optimizer & position tracker",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor positions for liquidation risk
    Monitor {
        /// Ethereum wallet address to monitor
        #[arg(short, long)]
        address: String,

        /// Alert threshold for health factor (default: 1.2)
        #[arg(short, long, default_value = "1.2")]
        threshold: f64,

        /// RPC endpoint (overrides config)
        #[arg(short, long)]
        rpc: Option<String>,
    },
    /// Show current positions and yields for an address
    Positions {
        /// Ethereum wallet address
        #[arg(short, long)]
        address: String,
    },
    /// Display top yield opportunities on Aave
    Yields {
        /// Filter by network (ethereum, polygon, arbitrum, base)
        #[arg(short, long, default_value = "ethereum")]
        network: String,
    },
    /// Run as an OpenClaw skill server
    Skill {
        /// Port to listen on
        #[arg(short, long, default_value = "7070")]
        port: u16,
    },
    /// Initialize configuration file
    Init,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("aave_claw=info".parse()?),
        )
        .without_time()
        .init();

    print_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { address, threshold, rpc } => {
            let cfg = config::Config::load()?;
            let rpc_url = rpc.unwrap_or(cfg.rpc_url.clone());
            info!("Starting liquidation monitor for {}", address.yellow());
            monitor::run_monitor(&address, threshold, &rpc_url).await?;
        }
        Commands::Positions { address } => {
            let cfg = config::Config::load()?;
            let positions = aave::fetch_positions(&address, &cfg.rpc_url).await?;
            ui::display_positions(&positions);
        }
        Commands::Yields { network } => {
            let yields = aave::fetch_yields(&network).await?;
            ui::display_yields(&yields);
        }
        Commands::Skill { port } => {
            info!("Starting as OpenClaw skill on port {}", port.to_string().yellow());
            openclaw::run_skill_server(port).await?;
        }
        Commands::Init => {
            config::Config::init()?;
            println!(
                "{} Configuration file created: {}",
                "✓".green().bold(),
                ".aave-claw.toml".yellow()
            );
            println!("  Edit it to set your RPC URL and Telegram bot token.");
        }
    }

    Ok(())
}

fn print_banner() {
    println!("{}", r#"
   ___  ___  _   _____ ___     ___ _      _____      __
  / _ |/ _ | | / / __/ __|   / __| |    /   \ \    / /
 / __ / __ | |/ / _|| (__   | (__| |__ | () |\ \/\/ / 
/_/ |_/_/ |_|___/___|\___| __|___|____|\___ | \_/\_/ 
                           |___|                       
"#.bright_purple().bold());
    println!("  {} by {}", "v0.1.0".dimmed(), "susanudgzf".bright_purple());
    println!("  {} Aave DeFi Monitor · OpenClaw Skill\n", "⚡".yellow());
}
