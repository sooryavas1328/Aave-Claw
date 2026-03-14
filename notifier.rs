use anyhow::Result;
use reqwest::Client;

use crate::aave::UserPosition;
use crate::config::Config;

/// Send a Telegram alert when health factor drops below threshold.
pub async fn send_alert(cfg: &Config, pos: &UserPosition, threshold: f64) -> Result<()> {
    let token = match &cfg.telegram_bot_token {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return Ok(()), // No Telegram configured — skip silently
    };
    let chat_id = match &cfg.telegram_chat_id {
        Some(c) if !c.is_empty() => c.clone(),
        _ => return Ok(()),
    };

    let emoji = if pos.health_factor < 1.1 { "🚨🚨🚨" } else { "⚠️" };

    let text = format!(
        "{emoji} *Aave-Claw Alert*\n\
        \n\
        Address: `{}`\n\
        Health Factor: *{:.3}* (threshold: {:.2})\n\
        \n\
        💰 Collateral: ${:.2}\n\
        📉 Total Debt: ${:.2}\n\
        💎 Net Worth:  ${:.2}\n\
        \n\
        _{}_",
        shorten_address(&pos.address),
        pos.health_factor,
        threshold,
        pos.total_collateral_usd,
        pos.total_debt_usd,
        pos.net_worth_usd,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S UTC"),
        emoji = emoji,
    );

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        }))
        .send()
        .await?;

    Ok(())
}

fn shorten_address(addr: &str) -> String {
    if addr.len() > 12 {
        format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
    } else {
        addr.to_string()
    }
}
