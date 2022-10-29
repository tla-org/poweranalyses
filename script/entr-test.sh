#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

find src nmath build.rs | entr -s 'cargo test'
