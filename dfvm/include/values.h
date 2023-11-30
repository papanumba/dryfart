/* values.h */

#ifndef DFVM_VALUES_H
#define DFVM_VALUES_H

#include "common.h"

struct Object; /* to avoid cyclic dependency */

enum ValType {
    VAL_V = 0x00,
    VAL_B = 0x02,
    VAL_C = 0x04,
    VAL_N = 0x06,
    VAL_Z = 0x08,
    VAL_R = 0x0A,
    VAL_O = 0x0C /* any heap stuff */
};

struct DfVal {
    enum ValType type;
    union {
        int b; /* int used as 0 or !0 */
        char c;
        uint n;
        int z;
        float r;
        struct Object *o;
    } as;
};

struct Values {
    struct DfVal *arr;
    size_t        len;
    size_t        cap;
};

void values_init(struct Values *);
void values_free(struct Values *);
void values_push(struct Values *, struct DfVal);
void values_print(struct DfVal);
char values_type_to_char(enum ValType);

#endif /* DFVM_VALUES_H */
