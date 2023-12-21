#!/bin/sh
lines=0
cd infarter
lines=$(($lines+$(./lines.sh)))
cd ..
cd dfvm
lines=$(($lines+$(./lines.sh)))
cd ..
cd dfarted
lines=$(($lines+$(./lines.sh)))
cd ..
echo $lines
exit 0
