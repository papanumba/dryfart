#!/bin/sh

for f in $(ls *.df)
do
    ../target/debug/infarter $f > /dev/null 2> /dev/null
    if [ $? -eq 0 ];
     then
        echo "PASS $f"
    else
        echo "PASS $f"
    fi
done
