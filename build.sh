#!/usr/bin/env bash

set -e

cargo build --target wasm32-unknown-emscripten --release

./public.sh
