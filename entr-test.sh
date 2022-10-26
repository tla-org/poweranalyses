#!/usr/bin/env bash

set -e

find src nmath build.rs | entr -s 'cargo test'
