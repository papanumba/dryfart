/* common.c */

#include <stdlib.h>
#include <stdio.h>
#include "common.h"

void eput(const char *msg)
{
    fputs(msg, stderr);
}

void eputln(const char *msg)
{
    fprintf(stderr, "%s\n", msg);
}

void panic(const char *msg)
{
    eputln(msg);
    exit(EXIT_FAILURE);
}

void todo(const char *msg)
{
    eput("TODO: ");
    panic(msg);
}

void unreachable(void)
{
    panic("reached the unreachable");
}
