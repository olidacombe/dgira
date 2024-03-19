#!/usr/bin/env bash

set -euo pipefail

cargo run -q -- -l 100 -c \
    | jq --unbuffered -r '"\(.key) \(.fields.summary)"' \
    | fzf
