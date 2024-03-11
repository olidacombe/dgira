#!/usr/bin/env bash

set -euo pipefail

cargo run -q \
    | jq --unbuffered -r '"\(.key) \(.fields.summary)"' \
    | fzf
