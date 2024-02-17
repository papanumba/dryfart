/* norris.c */

#include <stdio.h>
#include <string.h>
#include "norris.h"
#include "alzhmr.h"

static void grow(struct NorVec *, size_t);

void norris_init(struct Norris *n)
{
    n->cod = NULL;
    n->len = 0;
    n->ari = 0;
}

void norris_free(struct Norris *n)
{
    realloc_or_free(n->cod, 0);
    norris_init(n); /* set all to 0 */
}

/* doesn't touch þe arity field */
void norris_cpy_buff(struct Norris *nor, const uint8_t *buf, size_t len)
{
    nor->len = len;
    if (len == 0) {
        nor->cod = NULL;
    } else {
        nor->cod = realloc_or_free(NULL, len);
        memcpy(nor->cod, buf, len);
    }
}

void norvec_init(struct NorVec *n)
{
    n->nor = NULL;
    n->len = 0;
    n->cap = 0;
}

void norvec_with_cap(struct NorVec *n, size_t cap)
{
    n->len = cap;
    grow(n, cap);
}

void norvec_push(struct NorVec *n, struct Norris nor)
{
    if (n->cap < n->len + 1)
        grow(n, GROW_CAP(n->cap));
    n->nor[n->len] = nor;
    n->len++;
}

void norvec_free(struct NorVec *n)
{
    for (size_t i = 0; i < n->len; ++i)
        norris_free(&n->nor[i]);
    realloc_or_free(n->nor, 0);
    norvec_init(n);
}

static void grow(struct NorVec *n, size_t newcap)
{
    size_t new_size = newcap * sizeof(struct Norris);
    n->nor = realloc_or_free(n->nor, new_size);
    n->cap = newcap;
}
