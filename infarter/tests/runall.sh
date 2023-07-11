#!/bin/sh

for f in $(ls .)
do
    cargo run $f
done
