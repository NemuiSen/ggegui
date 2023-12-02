#!/usr/bin/env sh

set -ex

cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
