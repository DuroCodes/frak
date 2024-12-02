#!/bin/bash

OS=""

# Check the OS
check_os() {
  case "$OSTYPE" in
    linux-gnu*) OS="linux" ;;
    darwin*) OS="macos" ;;
    cygwin*|msys*|win32*) OS="windows" ;;
    *) echo "Unknown OS: $OSTYPE"; exit 1 ;;
  esac
}

# Install Rust if not present
install_rust() {
  if ! command -v rustc &>/dev/null; then
    echo "Rust is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    if [[ "$OS" == "windows" ]]; then
      echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> "$PROFILE_FILE"
    else
      source "$HOME/.cargo/env"
    fi
  else
    echo "Rust is already installed."
  fi
}

# Install wasm32-unknown-unknown target if not present
install_target() {
  if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    rustup target add wasm32-unknown-unknown
  fi
}

# Install wasm-opt if not present
install_wasm_opt() {
  if ! command -v wasm-opt &>/dev/null; then
    cargo install wasm-opt
  fi
}

# Install WABT on Linux
install_wabt_linux() {
  if ! command -v wasm-opt &>/dev/null || ! command -v wasm-strip &>/dev/null; then
    echo "WABT not found, installing via apt..."
    sudo apt update
    sudo apt install -y wabt
  else
    echo "WABT is already installed."
  fi
}

# Install WABT on macOS
install_wabt_macos() {
  # Check if brew is available as a command or located at /opt/homebrew/bin/brew
  if ! command -v brew &>/dev/null; then
    if [[ -f "/opt/homebrew/bin/brew" ]]; then
      echo "/opt/homebrew/bin/brew exists. Adding to PATH..."
      echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> "$HOME/.zshrc"
      eval "$(/opt/homebrew/bin/brew shellenv)"
    else
      echo "Homebrew is not installed. Installing..."
      /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
      echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> "$HOME/.zshrc"
      eval "$(/opt/homebrew/bin/brew shellenv)"
    fi
  else
    echo "Homebrew is already installed."
  fi

  # Ensure /opt/homebrew/bin is in PATH
  if [[ ":$PATH:" != *":/opt/homebrew/bin:"* ]]; then
    echo "Adding /opt/homebrew/bin to PATH..."
    echo 'export PATH="/opt/homebrew/bin:$PATH"' >> "$HOME/.zshrc"
    source "$HOME/.zshrc"
  fi

  # Check and install WABT
  if ! command -v wasm-opt &>/dev/null || ! command -v wasm-strip &>/dev/null; then
    echo "Installing WABT using Homebrew..."
    brew install wabt
  else
    echo "WABT is already installed."
  fi
}


# Install WABT on Windows
install_wabt_windows() {
  WABT_DIR="$HOME/wabt"
  if [[ ! -d "$WABT_DIR" ]]; then
    echo "WABT not found, downloading..."
    mkdir -p "$WABT_DIR"
    WABT_TAR_URL="https://github.com/WebAssembly/wabt/releases/download/1.0.36/wabt-1.0.36-windows.tar.gz"
    curl -L $WABT_TAR_URL -o "$WABT_DIR/wabt.tar.gz"
    tar -xzvf "$WABT_DIR/wabt.tar.gz" -C "$WABT_DIR"
    rm "$WABT_DIR/wabt.tar.gz"
    mv "$WABT_DIR/wabt-1.0.36/"* "$WABT_DIR/"
    rmdir "$WABT_DIR/wabt-1.0.36"
    echo "WABT extracted successfully to $WABT_DIR"
  fi

  if ! command -v wasm-opt &>/dev/null || ! command -v wasm-strip &>/dev/null; then
    echo "Adding WABT to PATH..."
    export PATH="$PATH:$WABT_DIR/bin"
  fi
}

# Main installation logic
install_wabt() {
  case "$OS" in
    linux) install_wabt_linux ;;
    macos) install_wabt_macos ;;
    windows) install_wabt_windows ;;
    *) echo "Unsupported OS for WABT installation"; exit 1 ;;
  esac
}

# Main script logic
check_os
install_rust
install_target
install_wasm_opt
install_wabt

# Build the WebAssembly binary
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/frak.wasm

echo "Building WebAssembly binary..."
RUSTFLAGS='-C target-feature=+bulk-memory,+mutable-globals -A static_mut_refs' \
  INITIAL_MEMORY=168 \
  cargo build --target $TARGET --release

# Optimize and strip the WebAssembly binary
echo "Stripping and optimizing..."
wasm-strip $BINARY
mkdir -p www
wasm-opt -O3 --enable-threads --enable-bulk-memory -o www/frak.wasm $BINARY

echo "Build completed successfully."
