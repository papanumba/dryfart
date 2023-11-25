/* norris.h */

#ifndef DFVM_NORRIS_H
#define DFVM_NORRIS_H

#include "common.h"
#include "values.h"

enum OpCode {
    OP_CTN = 0x00, /* load constant to Norris->ctn */
    OP_L00 = 0x01,
    OP_LBT = 0x02,
    OP_LBF = 0x03,
    OP_LN0 = 0x04,
    OP_LN1 = 0x05,
    OP_LM1 = 0x06, /* -Z%1 */
    OP_LZ0 = 0x07,
    OP_LZ1 = 0x08,
    OP_LR0 = 0x0B,
    OP_LR1 = 0x0C,

    OP_NEG = 0x10, /* unary int negate */
    OP_ADD = 0x11,
    OP_SUB = 0x12,
    OP_MUL = 0x13,
    OP_DIV = 0x14,

    OP_CEQ = 0x18,
    OP_CNE = 0x19,
    OP_CLT = 0x1A,
    OP_CLE = 0x1B,
    OP_CGT = 0x1C,
    OP_CGE = 0x1D,

    OP_NOT = 0x20,
    OP_AND = 0x21,
    OP_IOR = 0x22,

    OP_RET = 0xF0 /* return from current function */
    /* TODO: add opcodes */
};

/* chunk norris */
struct Norris {
    uchar        *cod; /* bytecode */
    uint          len; /* used length */
    uint          cap; /* allocd capanacity */
    struct Values ctn; /* constants */
};

void norris_init     (struct Norris *);
void norris_grow     (struct Norris *, uint);
void norris_free     (struct Norris *);
void norris_push_byte(struct Norris *, uchar);
uint norris_push_ctn (struct Norris *, struct DfVal);

#endif /* DFVM_NORRIS_H */
