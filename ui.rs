use colored::*;

use crate::aave::{UserPosition, YieldOpportunity};

pub fn display_positions(pos: &UserPosition) {
    println!("{}", "═".repeat(65).bright_purple());
    println!(
        " {} Position Summary for {}",
        "📊".normal(),
        shorten(&pos.address).yellow().bold()
    );
    println!("{}", "═".repeat(65).bright_purple());

    // Health factor
    let (hf_icon, hf_color) = health_factor_style(pos.health_factor);
    println!(
        " {hf_icon} Health Factor   : {}",
        format!("{:.3}", pos.health_factor).color(hf_color).bold()
    );
    println!(
        " 💰 Collateral     : {}",
        format!("${:.2}", pos.total_collateral_usd).green()
    );
    println!(
        " 📉 Total Debt     : {}",
        format!("${:.2}", pos.total_debt_usd).red()
    );
    println!(
        " 💎 Net Worth      : {}",
        format!("${:.2}", pos.net_worth_usd).cyan().bold()
    );

    if !pos.supplied.is_empty() {
        println!("\n {}", "SUPPLIED".bright_green().bold());
        println!(
            " {:<8} {:>12} {:>12} {:>8} {}",
            "Asset".dimmed(),
            "Amount".dimmed(),
            "USD Value".dimmed(),
            "APY".dimmed(),
            "Collateral".dimmed()
        );
        println!(" {}", "─".repeat(55).dimmed());
        for s in &pos.supplied {
            println!(
                " {:<8} {:>12.4} {:>12} {:>7.2}% {}",
                s.symbol.bright_white().bold(),
                s.amount,
                format!("${:.2}", s.amount_usd).green(),
                s.apy,
                if s.is_collateral { "✓".green().to_string() } else { "✗".red().to_string() }
            );
        }
    }

    if !pos.borrowed.is_empty() {
        println!("\n {}", "BORROWED".bright_red().bold());
        println!(
            " {:<8} {:>12} {:>12} {:>8} {}",
            "Asset".dimmed(),
            "Amount".dimmed(),
            "USD Value".dimmed(),
            "APY".dimmed(),
            "Rate".dimmed()
        );
        println!(" {}", "─".repeat(55).dimmed());
        for b in &pos.borrowed {
            println!(
                " {:<8} {:>12.4} {:>12} {:>7.2}% {}",
                b.symbol.bright_white().bold(),
                b.amount,
                format!("${:.2}", b.amount_usd).red(),
                b.apy,
                b.rate_mode.dimmed()
            );
        }
    }

    println!("{}\n", "═".repeat(65).bright_purple());
}

pub fn display_yields(yields: &[YieldOpportunity]) {
    if yields.is_empty() {
        println!("{}", "No yield data available.".yellow());
        return;
    }

    let network = &yields[0].network;
    println!("{}", "═".repeat(72).bright_purple());
    println!(
        " {} Aave Yield Opportunities — {}",
        "📈".normal(),
        network.to_uppercase().bright_purple().bold()
    );
    println!("{}", "═".repeat(72).bright_purple());
    println!(
        " {:<8} {:>10} {:>12} {:>16} {:>12}",
        "Asset".dimmed(),
        "Supply APY".dimmed(),
        "Borrow APY".dimmed(),
        "Liquidity (USD)".dimmed(),
        "Utilization".dimmed()
    );
    println!(" {}", "─".repeat(63).dimmed());

    for y in yields {
        let util_color = if y.utilization_rate > 80.0 {
            "red"
        } else if y.utilization_rate > 60.0 {
            "yellow"
        } else {
            "green"
        };

        println!(
            " {:<8} {:>9.2}%  {:>10.2}%  {:>15}  {:>10}",
            y.symbol.bright_white().bold(),
            y.supply_apy,
            y.borrow_apy_variable,
            format_usd_large(y.total_liquidity_usd).green(),
            format!("{:.1}%", y.utilization_rate).color(util_color),
        );
    }

    println!("{}\n", "═".repeat(72).bright_purple());
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn shorten(addr: &str) -> String {
    if addr.len() > 12 {
        format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
    } else {
        addr.to_string()
    }
}

fn health_factor_style(hf: f64) -> (&'static str, &'static str) {
    if hf >= 2.0 {
        ("✅", "green")
    } else if hf >= 1.5 {
        ("🟡", "yellow")
    } else if hf >= 1.1 {
        ("🟠", "red")
    } else {
        ("🔴", "bright_red")
    }
}

fn format_usd_large(val: f64) -> String {
    if val >= 1_000_000_000.0 {
        format!("${:.2}B", val / 1_000_000_000.0)
    } else if val >= 1_000_000.0 {
        format!("${:.2}M", val / 1_000_000.0)
    } else {
        format!("${:.0}", val)
    }
}
