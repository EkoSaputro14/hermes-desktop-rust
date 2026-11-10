#!/usr/bin/env bash
# Hermes Desktop — Native Rust & Portable Launcher (macOS & Linux)
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "============================================================================"
echo "  HERMES DESKTOP (RUST ICED EDITION) — DESKTOP LAUNCHER"
echo "============================================================================"

# 1. If native binary is already compiled, run it immediately
if [ -x "$DIR/target/release/hermes-desktop" ]; then
  echo "[OK] Launching compiled native Rust binary: target/release/hermes-desktop"
  exec "$DIR/target/release/hermes-desktop"
fi

# 2. If Cargo is installed locally, compile and run native release binary
if command -v cargo >/dev/null 2>&1; then
  echo "[INFO] Rust Cargo detected ($(cargo --version)). Compiling optimized release binary..."
  cargo build --release
  if [ -x "$DIR/target/release/hermes-desktop" ]; then
    echo "[OK] Build succeeded! Launching native binary..."
    exec "$DIR/target/release/hermes-desktop"
  fi
fi

# 3. Instant Zero-Install Mode: Open Standalone Desktop Window
echo "[INFO] Launching Hermes Desktop Standalone Application..."
if [[ "$OSTYPE" == "darwin"* ]]; then
  if [ -d "/Applications/Google Chrome.app" ]; then
    open -n -a "Google Chrome" --args --app="file://$DIR/Hermes-Desktop-Standalone.html" --window-size=1440,900
    exit 0
  fi
  open "$DIR/Hermes-Desktop-Standalone.html"
else
  if command -v google-chrome >/dev/null 2>&1; then
    google-chrome --app="file://$DIR/Hermes-Desktop-Standalone.html" --window-size=1440,900 &
    exit 0
  elif command -v chromium >/dev/null 2>&1; then
    chromium --app="file://$DIR/Hermes-Desktop-Standalone.html" --window-size=1440,900 &
    exit 0
  else
    xdg-open "$DIR/Hermes-Desktop-Standalone.html"
  fi
fi
