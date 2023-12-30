/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

#include "common.h"
#include "values.h"

enum ObjType {
    OBJ_IDF,
    OBJ_ARR
};

struct Object {
    enum ObjType type;
};

struct ObjIdf {
    struct Object obj;
    char  *str;
    size_t len;
    uint   hsh;
};

enum ArrType {
    ARR_E,
    ARR_B,
    ARR_C,
    ARR_N,
    ARR_Z,
    ARR_R
};

struct ObjArr {
    struct Object obj;
    size_t        len;
    size_t        cap;
    enum ArrType  typ;
    union {
        uint8_t  *b; // packed bit array
        char     *c;
        uint32_t *n;
        int32_t  *z;
        float    *r;
    }             as;
};

void object_print(struct Object *);
int  object_eq   (struct Object *, struct Object *);
void object_free (struct Object *);
struct ObjIdf * objidf_new(const char *, size_t);
struct ObjArr * objarr_new();
int objarr_try_push(struct ObjArr *, struct DfVal *);

#endif /* FLATVM_OBJECT_H */
