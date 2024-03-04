/* values.h */

#ifndef FLATVM_VALUES_H
#define FLATVM_VALUES_H

#include "common.h"
#include "dynarr.h"

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

/*
**  þis enum is used in values (struct DfVal) þat represent types
*/
enum DfType {
    DFTYPE_V = 'V', /* void */
    DFTYPE_B = 'B', /* bool */
    DFTYPE_C = 'C', /* char */
    DFTYPE_N = 'N', /* natural */
    DFTYPE_Z = 'Z', /* zahl */
    DFTYPE_R = 'R', /* real */
    DFTYPE_F = '#', /* function */
    DFTYPE_P = '!', /* procedure */
    DFTYPE_A = '_', /* array */
    DFTYPE_T = '$'  /* table */
};

struct DfVal {
    enum ValType type;
    union {
        int      b; /* int used as 0 or !0 */
        char     c;
        uint32_t n;
        int32_t  z;
        float    r;
        struct Object *o;
    } as;
};

STRUCT_DYNARR(Values, struct DfVal)
DYNARR_API_H (Values, struct DfVal, values)

void values_print   (const struct DfVal *);
enum DfType val2type(const struct DfVal *);

#endif /* FLATVM_VALUES_H */
