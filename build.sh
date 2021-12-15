#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package template && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/template.wasm -o ./target/wasm32-unknown-unknown/release/template.wasm

#  cargo build --target wasm32-unknown-unknown --package chain_cloud --release