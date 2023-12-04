#!/bin/sh

CFLAGS="-ansi -Wpedantic -Wall -Wextra -Iinclude"

if [ "$1" = "-g" ];
then
    gcc $CFLAGS -g -DDEBUG src/*.c
else
    gcc $CFLAGS -O3 src/*.c
fi

exit 0
