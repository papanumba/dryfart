#!/bin/sh
cat Cargo.toml src/*.rs src/*/*.rs | grep . | wc -l
