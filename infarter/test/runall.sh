#!/bin/sh

bin="../target/debug/infarter"
echo $bin

if [ "$1" = "-r" ]; # --release
then
    bin="../target/release/infarter"
fi

for f in $(ls *.df)
do
    $bin $f > /dev/null 2> /dev/null
    if [ $? -eq 0 ];
    then
        echo "PASS $f"
    else
        echo "FAIL $f"
    fi
done
