#!/bin/sh
cat Cargo.toml *.rs src/*.* src/dflib/* | grep . | wc -l
