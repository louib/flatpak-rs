#!/usr/bin/env bash
# Publish a new version the library.

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

SCRIPT_DIR=$(dirname "$0")

if ! command -v cargo; then
    die "Missing cargo";
fi

cargo build --release
cargo publish
echo "📦 Published the package on crates.io."
