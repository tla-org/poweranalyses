#!/usr/bin/env bash

set -e

find index.html style.css frontend.js | entr -s './public.sh'
