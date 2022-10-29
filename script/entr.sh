#!/usr/bin/env bash

set -e

find src nmath index.html frontend.js target/wasm32-unknown-emscripten/release | entr -s './build.sh'
