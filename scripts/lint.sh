#!/usr/bin/env sh

cargo clippy --tests --quiet --no-deps -- -D warnings
