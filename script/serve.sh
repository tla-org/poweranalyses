#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))

cd "$BASEDIR/public"

PORT="$1"
if [ "$PORT" = "" ]; then
    PORT="8080"
fi

live-server --no-browser --port="$PORT"
