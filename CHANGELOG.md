# Changelog

All notable changes to Aave-Claw will be documented in this file.

## [0.1.0] - 2025-03-14

### Added
- Real-time health factor monitor with configurable alert threshold
- Telegram notifications for liquidation risk
- Yield opportunity display across 6 networks (Ethereum, Polygon, Arbitrum, Base, Optimism, Avalanche)
- Full position breakdown (supplied/borrowed assets, APYs, collateral flags)
- OpenClaw skill server (`aave-claw skill`) with 3 tools: `get_positions`, `get_yields`, `check_health`
- One-line installer for Linux/macOS (`install.sh`) and Windows (`install.ps1`)
- Cross-platform release builds via GitHub Actions (Windows, Linux musl/glibc, macOS x86/arm64)
- Config file (`.aave-claw.toml`) and env var support
- Colored terminal output with health factor status indicators
