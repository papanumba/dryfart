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

DYNARR_API_C(NorVec, struct Norris, norvec, norris_free)
