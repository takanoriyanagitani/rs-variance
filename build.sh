#!/bin/sh

cargo \
    build \
    --target wasm32-unknown-unknown \
    --features mean,simple,two-pass,ext_wasm,shift \
    --profile release-wasm
