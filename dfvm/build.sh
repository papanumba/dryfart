#!/bin/sh

if [ "$1" = "-g" ];
then
    gcc -ansi -Wpedantic -Wall -Wextra -g -DDEBUG *.c
else
    gcc -ansi -Wpedantic -Wall -Wextra -O *.c
fi

exit 0
