#!/bin/sh
cloc $(find . | grep -P "\.(rs|c(pp)?|h|py|sh|ui)$") #| grep -o -P "\d+$"
exit 0
