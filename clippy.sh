#!/bin/sh

cargo \
    clippy \
    --target wasm32-unknown-unknown \
    --all-features
