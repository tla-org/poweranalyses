#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))

cd "$BASEDIR/public"

live-server --no-browser
