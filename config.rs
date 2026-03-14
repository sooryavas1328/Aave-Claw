use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"# Aave-Claw Configuration
# See: https://github.com/susanudgzf/Aave-Claw

# Ethereum RPC endpoint (get free one at https://alchemy.com or https://infura.io)
rpc_url = "https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"

# Optional: Telegram bot token for alerts
# Get one from @BotFather on Telegram
telegram_bot_token = ""

# Optional: Your Telegram chat ID for receiving alerts
telegram_chat_id = ""

# Default health factor alert threshold
health_factor_threshold = 1.2

# Polling interval in seconds
poll_interval_secs = 30

# Supported networks: ethereum, polygon, arbitrum, base, optimism, avalanche
default_network = "ethereum"
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub health_factor_threshold: f64,
    pub poll_interval_secs: u64,
    pub default_network: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: "https://eth-mainnet.g.alchemy.com/v2/demo".to_string(),
            telegram_bot_token: None,
            telegram_chat_id: None,
            health_factor_threshold: 1.2,
            poll_interval_secs: 30,
            default_network: "ethereum".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Load .env if present
        let _ = dotenv::dotenv();

        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)
                .context("Failed to read config file")?;
            let cfg: Config = toml::from_str(&content)
                .context("Failed to parse config file")?;
            return Ok(cfg);
        }

        // Try env vars as fallback
        let mut cfg = Config::default();
        if let Ok(rpc) = std::env::var("AAVE_CLAW_RPC_URL") {
            cfg.rpc_url = rpc;
        }
        if let Ok(token) = std::env::var("TELEGRAM_BOT_TOKEN") {
            cfg.telegram_bot_token = Some(token);
        }
        if let Ok(chat) = std::env::var("TELEGRAM_CHAT_ID") {
            cfg.telegram_chat_id = Some(chat);
        }

        Ok(cfg)
    }

    pub fn init() -> Result<()> {
        let path = Self::config_path();
        if path.exists() {
            println!("Config already exists at {:?}", path);
            return Ok(());
        }
        std::fs::write(&path, DEFAULT_CONFIG)
            .context("Failed to write config file")?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        // Check current directory first, then home directory
        let local = PathBuf::from(".aave-claw.toml");
        if local.exists() {
            return local;
        }
        if let Some(home) = dirs::home_dir() {
            return home.join(".aave-claw.toml");
        }
        local
    }
}
