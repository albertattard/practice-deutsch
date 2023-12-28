#!/bin/sh

set -e

cargo fmt
cargo check
cargo build --release
