#!/bin/bash

if [[ $1 = "wasm" ]]; then
  if cargo build --target wasm32-unknown-unknown; then
    wasm-server-runner target/wasm32-unknown-unknown/debug/bevy_test.wasm
  fi
else
  cargo run
fi
