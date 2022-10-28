#!/usr/bin/env bash

set -e

mkdir -p public
cp index.html public
cp style.css public
cp frontend.js public
cp target/wasm32-unknown-emscripten/release/pa.js public
cp target/wasm32-unknown-emscripten/release/pa.wasm public
