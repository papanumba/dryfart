/* object.h */

#ifndef DFVM_OBJECT_H
#define DFVM_OBJECT_H

#include "common.h"
#include "values.h"

enum ObjType {
    OBJ_STR
};

struct Object {
    enum ObjType type;
    /*TODO*/
};

struct ObjStr {
    struct Object obj; /* type punning */
    size_t        len;
    char         *str;
};

void object_print(struct Object *);
int  object_eq   (struct Object *, struct Object *);
struct ObjStr objstr_from_chars(const char *, size_t);

#endif /* DFVM_OBJECT_H */
