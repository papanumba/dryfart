#!/bin/sh
gcc -pg -Iinclude -O3 -ansi src/*.c -o gprof-a.out
./gprof-a.out "$@"
gprof gprof-a.out gmon.out > gprof-out.txt
