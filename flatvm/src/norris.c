/* norris.c */

#include <stdio.h>
#include <string.h>
#include "norris.h"
#include "alzhmr.h"

void norris_init(struct Norris *n)
{
    n->cod = NULL;
    n->len = 0;
    n->ari = 0;
}

void norris_free(struct Norris *n)
{
    free(n->cod);
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
    DYNARR_INIT(*n);
}

void norvec_w_cap(struct NorVec *n, size_t cap)
{
    DYNARR_W_CAP(*n, cap);
}

void norvec_push(struct NorVec *n, struct Norris nor)
{
    DYNARR_PUSH(*n, nor);
}

void norvec_free(struct NorVec *n)
{
    DYNARR_FREE(*n, norris_free);
}
