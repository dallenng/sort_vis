#!/usr/bin/env sh

set -ex

cargo fmt --check
cargo sort --check --check-format
