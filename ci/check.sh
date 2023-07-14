#!/usr/bin/env sh

set -ex

cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
