#!/usr/bin/env bash
# Publish a new version of the library.

set -e

die() { echo "🔥 Error: $*" 1>&2; exit 1; }

if ! command -v cargo; then
    die "Missing cargo";
fi

output=$(cargo doc --quiet 2>&1)
if [[ -n "$output" ]]; then
    die "There were errors or warnings when generating the doc: $output"
fi

echo "👍 Generated the doc."
