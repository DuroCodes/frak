#!/bin/bash

set -e  # Exit on any error

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/frak.wasm
WABT_VERSION=1.0.36
WABT_DIR="$HOME/wabt-$WABT_VERSION"

# Function to install Rust if not already installed
install_rust() {
  if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    # Check if .cargo exists in the user's profile for Windows
    if [ ! -d "$HOME/.cargo" ]; then
      echo "Rust is not installed. Installing Rust..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      # Add Cargo to PATH (for Windows)
      echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
      echo "Please restart your shell to ensure Cargo is in your PATH."
    else
      echo "Rust is already installed on Windows."
    fi
  else
    # Linux/macOS installation
    if ! command -v cargo &>/dev/null; then
      echo "Rust is not installed. Installing Rust..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source $HOME/.cargo/env
    else
      echo "Rust is already installed."
    fi
  fi
}

# Function to install wabt tools if not already installed
install_wabt() {
  if ! command -v wasm-strip &>/dev/null; then
    echo "Installing wabt (wasm-tools)..."
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
      echo "Windows detected. Installing wabt using manual download..."
      mkdir -p $WABT_DIR
      curl -L https://github.com/WebAssembly/wabt/releases/download/$WABT_VERSION/wabt-$WABT_VERSION-macos-12.tar.gz -o $WABT_DIR/wabt.tar.gz
      tar -xzvf $WABT_DIR/wabt.tar.gz -C $WABT_DIR
      export PATH="$PATH:$WABT_DIR/wabt-$WABT_VERSION/bin"
    else
      echo "Installing wabt-tools using package manager..."
      if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update && sudo apt-get install -y wabt
      elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install wabt
      fi
    fi
  else
    echo "wabt (wasm-tools) is already installed."
  fi
}

# Function to install wasm-opt using cargo
install_wasm_opt() {
  if ! command -v wasm-opt &>/dev/null; then
    echo "Installing wasm-opt via cargo..."
    cargo install wasm-opt
  fi
}

# Check and install dependencies
install_rust
install_wabt
install_wasm_opt

# Ensure the wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q $TARGET; then
  echo "Installing target $TARGET..."
  rustup target add $TARGET
fi

# Build the WebAssembly binary, ignoring static_mut_refs warning
echo "Building WebAssembly binary..."
RUSTFLAGS='-C target-feature=+bulk-memory,+mutable-globals -A static_mut_refs' \
  INITIAL_MEMORY=168 \
  cargo build --target $TARGET --release

# Optimize the WebAssembly binary
echo "Stripping debug symbols..."
wasm-strip $BINARY

echo "Optimizing WebAssembly binary..."
mkdir -p www
wasm-opt -O3 --enable-threads --enable-bulk-memory -o www/frak.wasm $BINARY

echo "Build completed successfully."
