#!/usr/bin/env sh

cargo build --all
cargo test -- --nocapture
