#!/bin/bash
set -e
REPO_URL="${REPO_URL:-https://github.com/xscriptor/xclock.git}"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="$(mktemp -d)"
OS="$(uname -s)"
detect_id() {
  if [ "$OS" = "Darwin" ]; then
    echo "macos"
    return
  fi
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    echo "${ID}"
    return
  fi
  echo "unknown"
}
ID="$(detect_id)"
ensure_tools() {
  case "$ID" in
    ubuntu|debian)
      sudo apt-get update -y
      sudo apt-get install -y build-essential curl git
      ;;
    fedora)
      sudo dnf install -y make gcc curl git
      ;;
    arch|manjaro)
      sudo pacman -Sy --noconfirm base-devel curl git
      ;;
    macos)
      xcode-select -p >/dev/null 2>&1 || xcode-select --install || true
      ;;
    *)
      :
      ;;
  esac
}
ensure_rust() {
  if ! command -v cargo >/dev/null 2>&1; then
    curl -fsSL https://sh.rustup.rs | sh -s -- -y
    if [ -f "$HOME/.cargo/env" ]; then
      . "$HOME/.cargo/env"
    fi
  fi
}
ensure_tools
ensure_rust
if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust installation failed"
  exit 1
fi
if [ -f "Cargo.toml" ] && grep -q "name = \"xclock\"" "Cargo.toml"; then
  cargo build --release
  sudo install -m 0755 target/release/xclock "$INSTALL_DIR/xclock"
else
  git clone "$REPO_URL" "$TEMP_DIR/xclock"
  cd "$TEMP_DIR/xclock"
  cargo build --release
  sudo install -m 0755 target/release/xclock "$INSTALL_DIR/xclock"
  cd -
  rm -rf "$TEMP_DIR"
fi
echo "Installed $INSTALL_DIR/xclock"
