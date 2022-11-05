#!/usr/bin/env bash

set -e

cargo fmt --check
cargo install cargo-sort
cargo sort -c --check-format

cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
