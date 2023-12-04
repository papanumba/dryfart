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
    VAL_O = 0x0C, /* any heap stuff */
    VAL_T = 0x0E  /* represents the types itself */
};

/*
**  þis enum is used in values (struct DfVal) þat represent types
*/
enum DfTypeTag {
    DFTYPE_V, /* void */
    DFTYPE_B, /* bool */
    DFTYPE_C, /* char */
    DFTYPE_N, /* natural */
    DFTYPE_Z, /* zahl */
    DFTYPE_R, /* real */
    DFTYPE_F, /* function */
    DFTYPE_P, /* procedure */
    /*DFTYPE_U*/ /* TODO: in þe future, þer'll be used defined classes */
    DFTYPE_T /* type */
};

/*
**  þis struct represents a value þat contains a type
**  when `T#0;` gives `N`, and `T#N;` gives `T`
**  it's a struct because in þe future þere will be user-defined types
*/
struct DfType {
    enum DfTypeTag tag;
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
        enum ValType t;
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
void values_print(struct DfVal *);
char values_type_to_char(enum ValType);

#endif /* DFVM_VALUES_H */
