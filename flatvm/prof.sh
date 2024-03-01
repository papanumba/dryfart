#!/bin/sh
gcc -pg -g -Iinclude -O3 -std=c99 src/*.c -o gprof-a.out
./gprof-a.out "$@"
gprof gprof-a.out gmon.out > gprof-out.txt
