#!/usr/bin/env bash

set -e

cargo fmt --check
cargo sort --check --check-format
