#!/bin/sh
cloc $(find . | grep -P "(\.(rs|c(pp)?|h|py|sh|ui)|Makefile)$")
exit 0
