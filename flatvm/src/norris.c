/* norris.c */

#include <stdio.h>
#include <stdlib.h>
#include "norris.h"
#include "object.h"
#include "alzhmr.h"

static void grow(struct Norris *, uint);
static int check_magic_df(const uchar *);
static const uchar * push_val_n(struct Norris *, const uchar *);
static const uchar * push_val_z(struct Norris *, const uchar *);
static const uchar * push_val_r(struct Norris *, const uchar *);
static const uchar * push_idf  (struct Norris *, const uchar *);

void norris_init(struct Norris *n)
{
    n->cod = NULL;
    n->len = 0;
    n->cap = 0;
    idents_init(&n->idf);
    values_init(&n->ctn);
}

/* reads constant pool and þe instructions */
int norris_from_buff(struct Norris *nor, const uchar *buf, size_t len)
{
    size_t i, ctnlen, idflen, explen;
    const uchar *rp  = NULL; /* read pointer */
    const uchar *end = NULL;
    norris_init(nor);
    if (nor == NULL || buf == NULL || len == 0)
        return FALSE;
    rp = buf;
    if (!check_magic_df(rp)) {
        fputs("magic number err\n", stderr);
        return FALSE;
    }
    rp += 8;
#ifdef DEBUG
    puts("read goodly magic numbr");
#endif
    idflen = b2tohi(rp);
    rp += 2;
#ifdef DEBUG
    printf("gonna read %hu identifier(s)\n", (ushort)idflen);
#endif
    for (i = 0; i < idflen; ++i)
        rp = push_idf(nor, rp);
#ifdef DEBUG
    puts("read goodly idents");
#endif
    /* read constants */
    ctnlen = b2tohi(rp);
    rp += 2;
#ifdef DEBUG
    printf("gonna read %hu constant(s)\n", (ushort)ctnlen);
#endif
    for (i = 0; i < ctnlen; ++i) {
        uchar type = *rp++;
        switch (type) {
          case VAL_N: rp = push_val_n(nor, rp); break;
          case VAL_Z: rp = push_val_z(nor, rp); break;
          case VAL_R: rp = push_val_r(nor, rp); break;
          default:
            fprintf(stderr, "found constant of type %c\n",
                values_type_to_char(type));
            exit(1);
        }
    }
    /* copy rest of bytecode */
    explen = b4tou(rp);
    rp += 4;
    end = &buf[len];
    if (end != rp + explen) {
        fputs("ERROR: file isn't the expected size\n", stderr);
        norris_free(nor);
        return FALSE;
    }
    while (rp < end)
        norris_push_byte(nor, *rp++);
    return TRUE;
}

void norris_free(struct Norris *n)
{
    realloc_or_free(n->cod, 0);
    idents_free(&n->idf);
    values_free(&n->ctn);
    norris_init(n); /* set all to 0 */
}

void norris_push_byte(struct Norris *n, uchar b)
{
    if (n->cap < n->len + 1)
        grow(n, GROW_CAP(n->cap));
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

static int check_magic_df(const uchar *buff)
{
    static uchar magic[8] = {0xDF, 'D', 'R', 'Y', 'F', 'A', 'R', 'T'};
    for (uint i = 0; i < 8; ++i) {
        if (buff[i] != magic[i])
            return FALSE;
    }
    return TRUE;
}

static const uchar * push_val_n(struct Norris *nor, const uchar *rp)
{
    struct DfVal valn;
    valn.type = VAL_N;
    valn.as.n = b4tou(rp);
    norris_push_ctn(nor, valn);
    return rp + 4;
}

static const uchar * push_val_z(struct Norris *nor, const uchar *rp)
{
    struct DfVal valz;
    valz.type = VAL_N;
    valz.as.z = b4toi(rp);
    norris_push_ctn(nor, valz);
    return rp + 4;
}

static const uchar * push_val_r(struct Norris *nor, const uchar *rp)
{
    struct DfVal valr;
    valr.type = VAL_R;
    valr.as.r = b4tof(rp);
    norris_push_ctn(nor, valr);
    return rp + 4;
}

static const uchar * push_idf(struct Norris *nor, const uchar *rp)
{
    size_t len = *rp++;
    if (rp[len] != '\0') /* check NUL term */
        panic("\\0 not found at end of identifier");
    struct DfIdf idf = dfidf_from_chars((char *) rp, len);
    idents_push(&nor->idf, idf);
#ifdef DEBUG
    printf("read goodly ident: \"%s\"\n", (char *)rp);
#endif /* DEBUG */
    return rp + len + 1;
}
