set -e

if [ -v CI ]; then
  cargo install cargo-sort
fi

cargo fmt --check
cargo sort --check

cargo build --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
