#!/bin/sh
set -e
clear
./format_measures.sh
cargo clippy --all-targets -- -D warnings
cargo test --quiet
echo OK
