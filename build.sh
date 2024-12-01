#!/bin/bash

set -e  # Exit on any error

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/frak.wasm

# Function to install Rust if it's not already installed
install_rust() {
  if ! command -v cargo &>/dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
  fi
}

# Function to install wabt package (wabt-tools) on Linux, macOS, or Windows
install_wabt() {
  if ! command -v wasm-strip &>/dev/null; then
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
      echo "Installing wabt (wabt-tools) for Linux..."
      sudo apt-get update && sudo apt-get install -y wabt
    elif [[ "$OSTYPE" == "darwin"* ]]; then
      echo "Installing wabt (wabt-tools) for macOS..."
      brew install wabt
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
      echo "Installing wabt (wabt-tools) for Windows..."
      # Windows: Install using winget (Windows Package Manager)
      if command -v winget &>/dev/null; then
        winget install wabt
      else
        echo "winget is not available, please install wabt manually from https://github.com/WebAssembly/wabt/releases"
        exit 1
      fi
    fi
  fi
}

# Ensure the wasm32-unknown-unknown target is installed
install_target() {
  if ! rustup target list --installed | grep -q $TARGET; then
    echo "Installing target $TARGET..."
    rustup target add $TARGET
  fi
}

# Build the WebAssembly binary, ignoring static_mut_refs warning
build_wasm() {
  echo "Building WebAssembly binary..."
  RUSTFLAGS='-C target-feature=+bulk-memory,+mutable-globals -A static_mut_refs' \
    INITIAL_MEMORY=168 \
    cargo build --target $TARGET --release
}

# Optimize the WebAssembly binary
optimize_wasm() {
  echo "Stripping debug symbols..."
  wasm-strip $BINARY

  echo "Optimizing WebAssembly binary..."
  mkdir -p www
  wasm-opt -O3 --enable-threads --enable-bulk-memory -o www/frak.wasm $BINARY
}

# Main script execution
install_rust
install_target
install_wabt
build_wasm
optimize_wasm

echo "Build completed successfully."
