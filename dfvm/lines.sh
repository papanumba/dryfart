#!/bin/sh
cat src/*.c include/*.h *.sh | grep . | wc -l
