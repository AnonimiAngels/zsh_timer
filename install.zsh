#!/usr/bin/env -S zsh -f
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${ZSH_SCRIPT}")" && pwd)"

[ -f "$ROOT_DIR/Cargo.toml" ] || { echo "Cargo.toml not found; run from project root."; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "cargo not found; install Rust toolchain first."; exit 1; }

cargo build --release

BUILD_LIB="$ROOT_DIR/target/release/libtimer.so"
[ -f "$BUILD_LIB" ] || { echo "build failed: $BUILD_LIB missing"; exit 1; }

INSTALL_DIR="$HOME/.zsh-modules/timer"
mkdir -p "$INSTALL_DIR"
cp "$BUILD_LIB" "$INSTALL_DIR/timer.so"

readonly INSTALL_FILE=".execution_timer"

[[ -f "$INSTALL_FILE" ]] || exit 1
if command -v realpath >/dev/null 2>&1; then
  abs=$(realpath "$INSTALL_FILE")
else
  abs="$(cd "$(dirname "$INSTALL_FILE")" 2>/dev/null && pwd)/$(basename $INSTALL_FILE)"
fi
zshrc="${ZDOTDIR:-$HOME}/.zshrc"
[[ -f $zshrc ]] || touch "$zshrc"
grep -Fqx "[[ -f \"$abs\" ]] && source \"$abs\"" "$zshrc" || printf '\n[[ -f "%s" ]] && source "%s"\n' "$abs" "$abs" >> "$zshrc"
