/* values.c */

#include <stdio.h>
#include <string.h>
#include "idents.h"
#include "alzhmr.h"

static void grow(struct Idents *, size_t);
static uint32_t hash_buff(const uint8_t *, size_t);

static void dfidf_init(struct DfIdf *i)
{
    i->str = NULL;
    i->len = 0;
    i->hsh = 0;
}

/* str mayn't be NUL-term'd, but idf.str will */
struct DfIdf dfidf_from_chars(const char *str, size_t len)
{
    struct DfIdf idf;
    dfidf_init(&idf);
    idf.str = realloc_or_free(NULL, (len+1) * sizeof(char));
    memcpy(idf.str, str, len);
    idf.str[len] = '\0';
    idf.len = len;
    idf.hsh = hash_buff((uint8_t *) str, len);
    return idf;
}

void dfidf_print(struct DfIdf *idf)
{
    printf("%s", idf->str);
}

void dfidf_free(struct DfIdf *idf)
{
    realloc_or_free(idf->str, 0);
    dfidf_init(idf);
}

void idents_init(struct Idents *i)
{
    i->arr = NULL;
    i->len = 0;
    i->cap = 0;
}

void idents_free(struct Idents *ids)
{
    for (size_t i = 0; i < ids->len; ++i)
        dfidf_free(&ids->arr[i]);
    realloc_or_free(ids->arr, 0);
    idents_init(ids); /* set all to 0 */
}

void idents_push(struct Idents *i, struct DfIdf idf)
{
    if (i->cap < i->len + 1)
        grow(i, GROW_CAP(i->cap));
    i->arr[i->len] = idf;
    i->len++;
}

static void grow(struct Idents *i, size_t newcap)
{
    size_t new_size = newcap * sizeof(struct DfIdf);
    i->arr = realloc_or_free(i->arr, new_size);
    i->cap = newcap;
}

/* FNV-1a (Fowler-Noll-Vo) hash function for 32 bit */
static uint32_t hash_buff(const uint8_t *buf, size_t len)
{
    uint32_t hash = 2166136261;
    for (size_t i = 0; i < len; ++i) {
        hash ^= buf[i];
        hash *= 16777619;
    }
    return hash;
}
