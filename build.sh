#! /bin/bash

cargo build --target wasm32-unknown-unknown --release --features web &&
wasm-bindgen --out-dir .public --target web target/wasm32-unknown-unknown/release/hug.wasm
