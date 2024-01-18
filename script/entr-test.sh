#!/usr/bin/env bash

#
# Run the tests when a file changes.
#

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

find src nmath build.rs | entr -s 'cargo test'
