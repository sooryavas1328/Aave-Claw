#!/usr/bin/env bash
set -e

REPO="susanudgzf/Aave-Claw"
BIN="aave-claw"
INSTALL_DIR="/usr/local/bin"

RED='\033[0;31m'
GREEN='\033[0;32m'
PURPLE='\033[0;35m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo -e "${PURPLE}  ⚡ Aave-Claw Installer${NC}"
echo -e "${PURPLE}  ━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  x86_64)  ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
esac

case "$OS" in
  linux)  TARGET="${ARCH}-unknown-linux-musl" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  *) echo -e "${RED}Unsupported OS: $OS${NC}"; exit 1 ;;
esac

# Check if cargo is available — if so, build from source
if command -v cargo &>/dev/null; then
  echo -e "${GREEN}✓ Rust toolchain found — building from source${NC}"
  echo -e "  ${YELLOW}cargo install --git https://github.com/${REPO}${NC}"
  cargo install --git "https://github.com/${REPO}" --bin "$BIN"
  echo ""
  echo -e "${GREEN}✓ Installed! Run: ${YELLOW}aave-claw --help${NC}"
  exit 0
fi

# Otherwise download prebuilt binary
echo -e "  Detected: ${YELLOW}${OS} / ${ARCH}${NC}"
echo -e "  Target  : ${YELLOW}${TARGET}${NC}"

# Get latest release tag
LATEST=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
  echo -e "${RED}Could not find a release. Install Rust and run:${NC}"
  echo -e "  ${YELLOW}cargo install --git https://github.com/${REPO}${NC}"
  exit 1
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST}/${BIN}-${TARGET}.tar.gz"

echo -e "  Downloading ${YELLOW}${LATEST}${NC}..."
curl -sL "$DOWNLOAD_URL" -o /tmp/aave-claw.tar.gz

echo -e "  Extracting..."
tar -xzf /tmp/aave-claw.tar.gz -C /tmp

# Install
if [ -w "$INSTALL_DIR" ]; then
  mv /tmp/$BIN "$INSTALL_DIR/$BIN"
else
  echo -e "  (requires sudo to install to ${INSTALL_DIR})"
  sudo mv /tmp/$BIN "$INSTALL_DIR/$BIN"
fi

chmod +x "$INSTALL_DIR/$BIN"

echo ""
echo -e "${GREEN}✓ aave-claw ${LATEST} installed to ${INSTALL_DIR}/${BIN}${NC}"
echo ""
echo -e "  Get started:"
echo -e "    ${YELLOW}aave-claw init${NC}               # create config file"
echo -e "    ${YELLOW}aave-claw yields${NC}             # show yield opportunities"
echo -e "    ${YELLOW}aave-claw positions -a 0x...${NC} # show wallet positions"
echo ""
