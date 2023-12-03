/* object.h */

#ifndef DFVM_OBJECT_H
#define DFVM_OBJECT_H

#include "common.h"
#include "values.h"

enum ObjType {
    OBJ_IDF
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

void object_print(struct Object *);
int  object_eq   (struct Object *, struct Object *);
void object_free (struct Object *);
struct ObjIdf * objidf_new(const char *, size_t);

#endif /* DFVM_OBJECT_H */
