#!/bin/sh

cargo \
    doc \
    --target wasm32-unknown-unknown \
    --all-features
