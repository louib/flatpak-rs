#!/usr/bin/env bash
# Publish a new version of the library.

SCRIPT_DIR=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_DIR")

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if ! command -v cargo; then
    die "Missing cargo";
fi

cargo build --release
cargo publish
echo "📦 Published the flatpak-rs package on crates.io."

# FIXME this is hackish and a better way would be to properly
# parse the TOML file. But for now this does the trick.
sed -i 's/name = "flatpak-rs"/name = "flatpak"/g' "$SCRIPT_DIR/../Cargo.toml"

cargo build --release
# This call to publish requires the dirty option because we've just renamed the crate.
cargo publish --allow-dirty
echo "📦 Published the flatpak package on crates.io."

# Revert the hack from above.
sed -i 's/name = "flatpak"/name = "flatpak-rs"/g' "$SCRIPT_DIR/../Cargo.toml"
