#!/usr/bin/env bash

set -e

cargo --version
cargo fmt --version

cargo fmt --check
cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
