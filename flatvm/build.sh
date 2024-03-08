#!/bin/sh

CC="clang"
CFLAGS="-std=c99 -Wpedantic -Wall -Wextra -Iinclude -o flatvm"
if [ "$CC" = "clang" ]; then
    CFLAGS="$CFLAGS -flto"
fi
FILES=$(ls src/*.c -I vm-ops.c | grep -v vm-ops.c)

if [ "$1" = "-g" ];
then
    echo "Building debug"
    $CC $CFLAGS -g -DSAFE -DDEBUG $FILES
elif [ "$1" = "-unsafe" ];
then
    echo "Building unsafe"
    $CC $CFLAGS -O3 $FILES
else
    echo "Building normal"
    $CC $CFLAGS -O3 -DSAFE $FILES && strip flatvm
fi

exit 0
