#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

cargo build --target wasm32-unknown-emscripten --release

./script/public.sh
