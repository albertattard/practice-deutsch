#!/bin/sh

set -e

cargo fmt
cargo check
cargo test
cargo build --release
