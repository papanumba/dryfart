#!/bin/sh
cloc $(find . | grep -P "\.(rs|c|h|py|md|sh|ui)$") #| grep -o -P "\d+$"
exit 0
