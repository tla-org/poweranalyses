#!/usr/bin/env bash

set -e

cargo build --target wasm32-unknown-emscripten --release

mkdir -p public
cp index.html public
cp frontend.js public
cp target/wasm32-unknown-emscripten/release/pa.js public
cp target/wasm32-unknown-emscripten/release/pa.wasm public
