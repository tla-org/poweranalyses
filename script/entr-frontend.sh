#!/usr/bin/env bash

set -e

BASEDIR=$(dirname $(dirname $(readlink -f "$0")))

find index.html style.css frontend.js | entr -s "bash $BASEDIR/script/public.sh"
