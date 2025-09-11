#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[ -f "$ROOT_DIR/Cargo.toml" ] || { echo "Cargo.toml not found; run from project root."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "cargo not found; install Rust toolchain first."; exit 1; }

cargo build --release

BUILD_LIB="$ROOT_DIR/target/release/libtimer.so"
[ -f "$BUILD_LIB" ] || { echo "build failed: $BUILD_LIB missing"; exit 1; }

INSTALL_DIR="$HOME/.zsh-modules/timer"
mkdir -p "$INSTALL_DIR"
cp "$BUILD_LIB" "$INSTALL_DIR/timer.so"

echo "Installed: $INSTALL_DIR/timer.so"
echo "Load with: zmodload $INSTALL_DIR/timer"
