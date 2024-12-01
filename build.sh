#!/bin/bash

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/frak.wasm

RUSTFLAGS='-C target-feature=+bulk-memory,+mutable-globals' \
  INITIAL_MEMORY=168 \
  cargo build --target wasm32-unknown-unknown --release

wasm-strip $BINARY
mkdir -p www
wasm-opt -O3 --enable-threads --enable-bulk-memory -o www/frak.wasm $BINARY
