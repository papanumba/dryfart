/* norris.c */

#include "../include/norris.h"
#include "../include/alzhmr.h"

void norris_init(struct Norris *n)
{
    n->cod = NULL;
    n->len = 0;
    n->cap = 0;
    values_init(&n->ctn);
}

void norris_grow(struct Norris *n, uint newcap)
{
    size_t new_size = newcap * sizeof(uchar);
    n->cod = realloc_or_free(n->cod, new_size);
    n->cap = newcap;
}

void norris_free(struct Norris *n)
{
    realloc_or_free(n->cod, 0);
    values_free(&n->ctn);
    norris_init(n); /* set all to 0 */
}

void norris_push_byte(struct Norris *n, uchar b)
{
    if (n->cap < n->len + 1) {
        uint new_cap = GROW_CAP(n->cap);
        norris_grow(n, new_cap);
    }
    n->cod[n->len] = b;
    n->len++;
}

uint norris_push_ctn(struct Norris *n, struct DfVal c)
{
    values_push(&n->ctn, c);
    return n->ctn.len - 1;
}
