#!/usr/bin/env bash

set -e

cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --all-features -- -W clippy::pedantic
cargo test --all-targets --all-features
