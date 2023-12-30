/* object.c */

#include <stdio.h>
#include <string.h>
#include "alzhmr.h"
#include "object.h"

static void objidf_print(struct ObjIdf *);
static void objidf_free (struct ObjIdf *);
static void objarr_print(struct ObjArr *);
static void objarr_free (struct ObjArr *);

static struct Object * alloc_object(enum ObjType);
static uint hash_string(const char *, size_t);

void object_print(struct Object *o)
{
    switch (o->type) {
      case OBJ_IDF: objidf_print((struct ObjIdf *)o); break;
      case OBJ_ARR: objarr_print((struct ObjArr *)o); break;
    }
}

int object_eq(struct Object *o0, struct Object *o1)
{
    if (o0->type != o1->type)
        return FALSE;
/*    switch (o0->type) {
    }*/
    return FALSE;
}

void object_free(struct Object *o)
{
    switch (o->type) {
      case OBJ_IDF: objidf_free((struct ObjIdf *)o); break;
      case OBJ_ARR: objarr_free((struct ObjArr *)o); break;
    }
}

struct ObjIdf * objidf_new(const char *str, size_t len)
{
    struct ObjIdf *idf;
    idf = (struct ObjIdf *) alloc_object(OBJ_IDF);
    idf->str = realloc_or_free(NULL, (len + 1) * sizeof(char));
    memcpy(idf->str, str, len);
    idf->str[len] = '\0';
    idf->len = len;
    idf->hsh = hash_string(str, len);
    return idf;
}

/* create empty array */
struct ObjArr * objarr_new()
{
    struct ObjArr *arr = (struct ObjArr *) alloc_object(OBJ_ARR);
    arr->len = 0;
    arr->cap = 0;
    arr->typ = ARR_E;
    arr->as.c = NULL; /* for safety */
    return arr;
}

static void objidf_free(struct ObjIdf *idf)
{
    realloc_or_free(idf->str, 0);
    realloc_or_free(idf, 0);
}

static void objidf_print(struct ObjIdf *idf)
{
    if (idf == NULL)
        return;
    printf("%s", idf->str);
}

static void objarr_print(struct ObjArr *arr)
{
    size_t i;
    size_t len = arr->len;
    if (arr->typ == ARR_C) { // string special case
        putchar('"');
        for (uint i = 0; i < arr->len; ++i)
            putchar(arr->as.c[i]);
        putchar('"');
        return;
    }
    putchar('_');
    switch (arr->typ) {
      case ARR_E: break;
      case ARR_B:
        for (i = 0; i < len; ++i) {
            int b = arr->as.b[i/8] & (1 << (i%8));
            printf("%c, ", b?'T':'F');
        }
        break;
      case ARR_C: break; // unreachable
      case ARR_N:
        for (i = 0; i < len; ++i)
            printf("%lu", (ulong) arr->as.n[i]);
        break;
      case ARR_Z:
        for (i = 0; i < len; ++i)
            printf("%ld", (long) arr->as.z[i]);
        break;
      case ARR_R:
        for (i = 0; i < len; ++i)
            printf("%f", (float) arr->as.r[i]);
        break;
    }
    putchar(';');
}

static void objarr_free(struct ObjArr *arr)
{
    if (arr->typ != ARR_E)
        return;
    realloc_or_free(arr->as.c, 0); // any would do
}

static struct Object * alloc_object(enum ObjType type)
{
    size_t size;
    switch (type) {
      case OBJ_IDF: size = sizeof(struct ObjIdf); break;
      case OBJ_ARR: size = sizeof(struct ObjArr); break;
    }
    struct Object *obj = realloc_or_free(NULL, size);
    obj->type = type;
    return obj;
}

/* FNV-1a (Fowler-Noll-Vo) hash function for 32 bit */
static uint hash_string(const char *str, size_t len)
{
    uint i;
    uint hash = 2166136261;
    for (i = 0; i < len; ++i) {
        hash ^= (uchar) str[i];
        hash *= 16777619;
    }
    return hash;
}
