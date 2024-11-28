#! /bin/bash
set -e
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --release --features web &&
wasm-bindgen --out-dir .public --target web target/wasm32-unknown-unknown/release/hug.wasm
