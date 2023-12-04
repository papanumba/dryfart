/* object.c */

#include <stdio.h>
#include <string.h>
#include "alzhmr.h"
#include "object.h"

static struct Object * alloc_object(enum ObjType);
static uint hash_string(const char *, size_t);
static void objidf_print(struct ObjIdf *);
static void objidf_free (struct ObjIdf *);

void object_print(struct Object *o)
{
    switch (o->type) {
      case OBJ_IDF:
        objidf_print((struct ObjIdf *)o);
        break;
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

static struct Object * alloc_object(enum ObjType type)
{
    size_t size;
    struct Object *obj;
    switch (type) {
      case OBJ_IDF: size = sizeof(struct ObjIdf); break;
    }
    obj = realloc_or_free(NULL, size);
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
