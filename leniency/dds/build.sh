#!/usr/bin/env bash
# Builds the DDS runner under /tmp/claude-0/target-dds (release, overflow checks off, like a distro build).
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
export CARGO_TARGET_DIR=/tmp/claude-0/target-dds
export PATH=/opt/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:$PATH
cargo build --release --manifest-path "$here/Cargo.toml"
