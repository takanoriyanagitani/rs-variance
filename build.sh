#!/bin/sh

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
    build \
    --target wasm32-unknown-unknown \
    --features mean,simple,two-pass,ext_wasm,shift,wasm_simd \
    --profile release-wasm
