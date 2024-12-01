#!/bin/bash

set -e  # Exit on any error

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/frak.wasm

# Check and install dependencies
if ! command -v cargo &>/dev/null; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
fi

# Ensure the wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q $TARGET; then
  echo "Installing target $TARGET..."
  rustup target add $TARGET
fi

if ! command -v wasm-strip &>/dev/null; then
  echo "Installing wasm-strip from wabt..."
  sudo apt-get update && sudo apt-get install -y wabt
fi

if ! command -v wasm-opt &>/dev/null; then
  echo "Installing wasm-opt from wabt..."
  sudo apt-get update && sudo apt-get install -y wabt
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
