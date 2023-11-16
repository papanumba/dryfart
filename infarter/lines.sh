#!/bin/sh
cat Cargo.toml src/*.rs src/dflib/* | grep . | wc -l
