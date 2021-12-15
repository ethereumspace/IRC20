#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package irc20 && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/irc20.wasm -o ./target/wasm32-unknown-unknown/release/irc20.wasm

#  cargo build --target wasm32-unknown-unknown --package chain_cloud --release