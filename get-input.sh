#!/usr/bin/env bash

HEADERS=/dev/stderr
[[ -t 1 ]] && CONTENT=/dev/null || CONTENT=/dev/stdout

URL="https://adventofcode.com/2021/day/$1/input"
SESSION_TOKEN='53616c7465645f5f349cbc7a6dadef2bfa033d19cbd862793812c7daffe59de10825df9f9eec6e61bbd45d7efe32be15'

curl -sSLX GET -D "$HEADERS" -o "$CONTENT" -b "session=$SESSION_TOKEN" "$URL"
