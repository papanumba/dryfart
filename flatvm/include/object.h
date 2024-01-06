/* object.h */

#ifndef FLATVM_OBJECT_H
#define FLATVM_OBJECT_H

#include "common.h"
#include "values.h"

#define OBJ_AS_ARR(o)   ((struct ObjArr *) (o))

enum ObjType {
    OBJ_ARR
};

struct Object {
    enum ObjType type;
    int gc_mark;
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

/* aux union for þe allocator */
typedef union {
    struct Object o;
    struct ObjArr a;
    /* eke here */
} objs_u;

void object_print(struct Object *);
int  object_eq   (struct Object *, struct Object *);
void object_free (struct Object *);
struct ObjArr * objarr_new     ();
int             objarr_try_push(struct ObjArr *, struct DfVal *);
struct DfVal    objarr_get     (struct ObjArr *, uint32_t);
struct ObjArr * objarr_concat  (struct ObjArr *, struct ObjArr *);

#endif /* FLATVM_OBJECT_H */
