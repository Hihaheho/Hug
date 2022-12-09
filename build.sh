#! /bin/bash
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --release --features web &&
~/.cargo/registry/bin/wasm-bindgen-0.2.64/bin/wasm-bindgen --out-dir .public --target web target/wasm32-unknown-unknown/release/hug.wasm
