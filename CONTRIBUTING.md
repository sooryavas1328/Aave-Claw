# Contributing to Aave-Claw

Thank you for your interest in contributing! 🦀

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/Aave-Claw`
3. Install Rust: https://rustup.rs
4. Build: `cargo build`
5. Run tests: `cargo test`

## Development Setup

```bash
# Copy env example
cp .env.example .env
# Edit .env with your RPC URL

# Run in dev mode
cargo run -- yields
cargo run -- monitor --address 0xYourAddress
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Add tests for new functionality

## Pull Request Process

1. Create a feature branch: `git checkout -b feat/my-feature`
2. Make your changes
3. Run `cargo test && cargo clippy && cargo fmt`
4. Push and open a PR against `main`

## Reporting Bugs

Open an issue with:
- Your OS and version
- `aave-claw --version` output
- Steps to reproduce
- Expected vs actual behavior

## Feature Requests

Open an issue with the `enhancement` label and describe your use case.
