#!/usr/bin/env bash

#
# Copy the files to the public directory.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

cp target/wasm32-unknown-emscripten/release/pa.js app/assets
cp target/wasm32-unknown-emscripten/release/pa.wasm app/assets