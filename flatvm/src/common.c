/* common.c */

#include <stdlib.h>
#include <stdio.h>
#include "common.h"

int bytearr_cmp(cbyte_p s, cbyte_p t, size_t len)
{
    TIL(i, len) {
        if (s[i] != t[i])
            return FALSE;
    }
    return TRUE;
}

void * realloc_or_free(void *ptr, size_t new_size)
{
    void *result = NULL;
    if (new_size == 0) { /* no need to realloc */
        free(ptr);
    } else {
        result = realloc(ptr, new_size);
        if (result == NULL)
            exit(1);
    }
    return result;
}

void eput(const char *msg)
{
    fputs(msg, stderr);
}

void eputln(const char *msg)
{
    fprintf(stderr, "%s\n", msg);
}

_Noreturn
void panic(const char *msg)
{
    eputln(msg);
    exit(EXIT_FAILURE);
}

_Noreturn
void todo(const char *msg)
{
    eput("TODO: ");
    panic(msg);
}

_Noreturn
void unreachable(void)
{
    panic("reached the unreachable");
}
