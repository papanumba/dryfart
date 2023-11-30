/* norris.c */

#include <stdio.h>
#include <stdlib.h>
#include "norris.h"
#include "alzhmr.h"

static void grow(struct Norris *, uint);

void norris_init(struct Norris *n)
{
    n->cod = NULL;
    n->len = 0;
    n->cap = 0;
    values_init(&n->ctn);
}

/* reads constant pool and þe instructions */
int norris_from_buff(struct Norris *nor, const uchar *buf, size_t len)
{
    uint i;
    size_t ctnlen;
    const uchar *rp  = NULL; /* read pointer */
    const uchar *end = NULL;
    norris_init(nor);
    if (nor == NULL)
        return FALSE;
    rp = buf;
    /* read constants len */
    ctnlen = b2toh(rp);
    rp += 2;
    /*puts("worihefohtn");*/
    /* read constants */
    for (i = 0; i < ctnlen; ++i) {
        /* read constant */
        uchar type = *rp++;
        switch (type) {
          case VAL_N: {
            struct DfVal valn;
            valn.type = VAL_N;
            valn.as.n = b4tou(rp);
            rp += 4;
            norris_push_ctn(nor, valn);
            break;
          }
          case VAL_R: {
            struct DfVal valr;
            valr.type = VAL_R;
            valr.as.r = b4tof(rp);
            rp += 4;
            norris_push_ctn(nor, valr);
            break;
          }
          default:
            puts("TODO: load other constants");
            exit(1);
        }
    }
    /* copy rest of bytecode */
    end = &buf[len];
    while (rp < end)
        norris_push_byte(nor, *rp++);
    return TRUE;
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
        grow(n, new_cap);
    }
    n->cod[n->len] = b;
    n->len++;
}

uint norris_push_ctn(struct Norris *n, struct DfVal c)
{
    values_push(&n->ctn, c);
    return n->ctn.len - 1;
}

static void grow(struct Norris *n, uint newcap)
{
    size_t new_size = newcap * sizeof(uchar);
    n->cod = realloc_or_free(n->cod, new_size);
    n->cap = newcap;
}
