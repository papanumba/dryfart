#!/bin/sh

CFLAGS="-std=c99 -Wpedantic -Wall -Wextra -Iinclude"

if [ "$1" = "-g" ];
then
    echo "Building debug"
    gcc $CFLAGS -g -DSAFE -DDEBUG src/*.c
elif [ "$1" = "-unsafe" ];
then
    echo "Building unsafe"
    gcc $CFLAGS -O3 src/*.c
else
    echo "Building normal"
    gcc $CFLAGS -O3 -DSAFE src/*.c
fi

exit 0
