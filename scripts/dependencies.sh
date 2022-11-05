#!/usr/bin/env bash

set -e

cargo install cargo-outdated
cargo outdated --exit-code 1
