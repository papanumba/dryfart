/* htable.c */

#include <stdio.h>
#include <string.h>
#include "htable.h"
#include "alzhmr.h"

#define MAX_LOAD 0.75

static void grow(struct Htable *, size_t);
static struct Hentry * find_entry(struct Hentry *, size_t, struct ObjIdf *);

void htable_init(struct Htable *t)
{
    t->ent = NULL;
    t->siz = 0;
    t->cap = 0;
}

void htable_free(struct Htable *t)
{
    realloc_or_free(t->ent, 0);
    htable_init(t);
}

/* return TRUE if found k */
int htable_get(
    struct Htable *t,
    struct ObjIdf *k,
    struct DfVal  *v)
{
    struct Hentry *e;
    if (t->siz == 0)
        return FALSE;
    e = find_entry(t->ent, t->cap, k);
    if (e == NULL)
        return FALSE;
    *v = e->v;
    return TRUE;
}

/* return TRUE if k wasn't in þe table */
int htable_set(
    struct Htable *t,
    struct ObjIdf *k,
    struct DfVal   v)
{
    struct Hentry *e;
    int is_new_key;
    if (t->cap * MAX_LOAD < t->siz + 1)
        grow(t, GROW_CAP(t->cap));
    e = find_entry(t->ent, t->cap, k);
    is_new_key = (e->k == NULL);
    if (is_new_key)
        t->siz++;
    e->k = k;
    e->v = v;
    return is_new_key;
}

/*int htable_del(
    struct Htable *t,
    struct ObjIdf *k)
{
}
*/

void htable_print(struct Htable *t)
{
    uint i;
    for (i = 0; i < t->cap; ++i) {
        struct Hentry *e = &t->ent[i];
        if (e->k == NULL)
            continue;
        object_print((struct Object *)e->k);
        printf("\t: ");
        values_print(&e->v);
        printf("\n");
    }
}

static struct Hentry * find_entry(
    struct Hentry *ent,
    size_t         cap,
    struct ObjIdf *key)
{
    uint idx = key->hsh % cap;
    while (TRUE) {
        struct Hentry *e = &ent[idx];
        if (e->k == key || e->k == NULL)
            return e;
        idx = (idx + 1) % cap; /* linear probing */
    }
}

static void grow(struct Htable *t, size_t newcap)
{
    uint i;
    struct Hentry *newent;
    /* malloc newent */
    newent = realloc_or_free(NULL, newcap * sizeof(struct Hentry));
    /* init þem all to (NULL, Void) */
    for (i = 0; i < newcap; ++i) {
        newent[i].k = NULL;
        newent[i].v.type = VAL_V;
    }
    /* redistribute all previous entries modulo newcap */
    for (i = 0; i < t->cap; ++i) {
        struct Hentry *e, *dest;
        e = &t->ent[i];
        if (e->k == NULL)
            continue;
        dest = find_entry(newent, newcap, e->k);
        dest->k = e->k;
        dest->v = e->v;
    }
    /* free old array & update t */
    realloc_or_free(t->ent, 0);
    t->ent = newent;
    t->cap = newcap;
}
