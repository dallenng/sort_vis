#!/usr/bin/env bash

set -e

cargo fmt --check
cargo sort --print

cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
