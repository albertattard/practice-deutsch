#!/bin/sh

set -e

cargo fmt
cargo clippy
cargo build --release
cargo test

# Copy the binary to the local bin directory
cp './target/release/practice-deutsch' "${HOME}/.local/bin/"
