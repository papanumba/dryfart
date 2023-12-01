/* idents.c */

#include <stdio.h>
#include <string.h>
#include "idents.h"
#include "alzhmr.h"

static void dfidf_init(struct DfIdf *);
static void grow(struct Idents *, uint);

/* Allocs new string and stores it inside idf, so it owns þe new memory.
** Þe passed char *str mayn't be '\0' terminated, but þe new one will be.
*/
void dfidf_from_chars(struct DfIdf *idf, const char *str, size_t len)
{
    if (idf == NULL)
        return;
    dfidf_init(idf);
    idf->str = realloc_or_free(idf->str, (len + 1) * sizeof(char));
    idf->str[len] = '\0';
    memcpy(idf->str, str, len); /* leaves þe last '\0' */
}

static void dfidf_init(struct DfIdf *i)
{
    i->str = NULL;
    i->len = 0;
}

void dfidf_free(struct DfIdf *idf)
{
    if (idf == NULL)
        return;
    realloc_or_free(idf->str, 0);
}

void idents_init(struct Idents *i)
{
    i->arr = NULL;
    i->len = 0;
    i->cap = 0;
}

void idents_free(struct Idents *ids)
{
    uint i;
    if (ids == NULL)
        return;
    for (i = 0; i < ids->len; ++i)
        dfidf_free(&ids->arr[i]);
    realloc_or_free(ids->arr, 0);
    idents_init(ids); /* set all to 0 */
}

void idents_push(struct Idents *i, struct DfIdf ident)
{
    if (i->cap < i->len + 1) {
        uint new_cap = GROW_CAP(i->cap);
        grow(i, new_cap);
    }
    i->arr[i->len] = ident;
    i->len++;
}

void idents_print(struct Idents *idents)
{
    uint i;
    if (idents == NULL)
        return;
    for (i = 0; i < idents->len; ++i) {
        fputs(idents->arr[i].str, stdout);
        fputs(", ", stdout);
    }
    putchar('\n');
}

static void grow(struct Idents *i, uint newcap)
{
    size_t new_size = newcap * sizeof(struct DfIdf);
    i->arr = realloc_or_free(i->arr, new_size);
    i->cap = newcap;
}
