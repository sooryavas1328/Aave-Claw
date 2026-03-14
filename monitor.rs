use anyhow::Result;
use colored::*;
use tracing::{info, warn};

use crate::aave::{self, UserPosition};
use crate::config::Config;
use crate::notifier;

/// Run the liquidation monitor loop for a given address.
pub async fn run_monitor(address: &str, threshold: f64, rpc_url: &str) -> Result<()> {
    let cfg = Config::load().unwrap_or_default();
    let interval = std::time::Duration::from_secs(cfg.poll_interval_secs);

    println!(
        "{} Monitoring {} every {}s",
        "👁".cyan(),
        address.yellow(),
        cfg.poll_interval_secs
    );
    println!(
        "{} Alert threshold: health factor < {}\n",
        "⚠".yellow(),
        threshold.to_string().red()
    );

    loop {
        match aave::fetch_positions(address, rpc_url).await {
            Ok(pos) => {
                print_health_status(&pos, threshold);

                if pos.health_factor < threshold {
                    warn!(
                        "ALERT: Health factor {:.3} below threshold {:.3}!",
                        pos.health_factor, threshold
                    );
                    if let Err(e) = notifier::send_alert(&cfg, &pos, threshold).await {
                        warn!("Failed to send notification: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to fetch positions: {}", e);
            }
        }

        info!(
            "Next check in {}s...",
            cfg.poll_interval_secs
        );
        tokio::time::sleep(interval).await;
    }
}

fn print_health_status(pos: &UserPosition, threshold: f64) {
    let hf = pos.health_factor;

    let (icon, hf_colored) = if hf >= 2.0 {
        ("✅", hf.to_string().green().bold())
    } else if hf >= threshold + 0.2 {
        ("🟡", hf.to_string().yellow().bold())
    } else {
        ("🔴", hf.to_string().red().bold())
    };

    let now = chrono::Local::now().format("%H:%M:%S");

    println!(
        "[{}] {} Health Factor: {}  |  Collateral: ${:.0}  |  Debt: ${:.0}",
        now.to_string().dimmed(),
        icon,
        hf_colored,
        pos.total_collateral_usd,
        pos.total_debt_usd,
    );
}
