/* values.h */

#ifndef DFVM_VALUES_H
#define DFVM_VALUES_H

#include "common.h"

enum ValType {
    VAL_B,
    VAL_C,
    VAL_N,
    VAL_Z,
    VAL_R
};

struct DfVal {
    enum ValType type;
    union {
        int b; /* used as 0 or !0 */
        char c;
        uint n;
        int z;
        float r;
    } as;
};

struct Values {
    struct DfVal *arr;
    uint          len;
    uint          cap;
};

void values_init(struct Values *);
void values_grow(struct Values *, uint);
void values_free(struct Values *);
void values_push(struct Values *, struct DfVal);
void values_print(struct DfVal);

#endif /* DFVM_VALUES_H */
