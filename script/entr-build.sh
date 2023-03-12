#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))
cd "$BASEDIR"

find dist power | entr -s ./script/build.sh
