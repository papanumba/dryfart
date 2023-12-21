/* norris.h */

#ifndef FLATVM_NORRIS_H
#define FLATVM_NORRIS_H

#include "common.h"
#include "values.h"

enum OpCode {
    OP_NOP = 0x00,
    OP_LVV = 0x01,
    OP_LBT = 0x02,
    OP_LBF = 0x03,
    OP_LN0 = 0x04,
    OP_LN1 = 0x05,
    OP_LN2 = 0x06,
    OP_LN3 = 0x07,
    OP_LM1 = 0x08,
    OP_LZ0 = 0x09,
    OP_LZ1 = 0x0A,
    OP_LZ2 = 0x0B,
    OP_LR0 = 0x0C,
    OP_LR1 = 0x0D,
    OP_LKS = 0x0E,
    OP_LKL = 0x0F,

    OP_NEG = 0x10, /* unary int negate */
    OP_ADD = 0x11,
    OP_SUB = 0x12,
    OP_MUL = 0x13,
    OP_DIV = 0x14,
    OP_INV = 0x15, /* unary float invert */

    OP_CEQ = 0x18,
    OP_CNE = 0x19,
    OP_CLT = 0x1A,
    OP_CLE = 0x1B,
    OP_CGT = 0x1C,
    OP_CGE = 0x1D,

    OP_NOT = 0x20,
    OP_AND = 0x21,
    OP_IOR = 0x22,

    OP_LGL = 0x40, /* Load   Global Long  (u16) */
    OP_SGL = 0x41, /* Store  Global Long  (u16) */
    OP_LLS = 0x44, /* Load   Local  Short (u8)  */
    OP_SLS = 0x45, /* Store  Local  Short (u8)  */
    OP_ULS = 0x46, /* Update Local  Short (u8)  */
    OP_LLL = 0x47, /* Load   Local  Long  (u16) */
    OP_SLL = 0x48, /* Store  Local  Long  (u16) */
    OP_ULL = 0x49, /* Update Local  Long  (u16) */

    OP_CAB = 0xE2,
    OP_CAC = 0xE4,
    OP_CAN = 0xE6,
    OP_CAZ = 0xE8,
    OP_CAR = 0xEA,
    OP_CAT = 0xEE, /* cast a value into its type */

    OP_JJS = 0x50,
    OP_JJL = 0x51,
    OP_JBT = 0x52,
    OP_JBF = 0x53,
    OP_JTS = 0x54,
    OP_JTL = 0x55,
    OP_JFS = 0x56,
    OP_JFL = 0x57,
    OP_JES = 0x58,
    OP_JEL = 0x59,
    OP_JNS = 0x5A,
    OP_JNL = 0x5B,
    OP_JLT = 0x5C,
    OP_JLE = 0x5D,
    OP_JGT = 0x5E,
    OP_JGE = 0x5F,

    OP_RET = 0xF0, /* return from current function */
    OP_DUP = 0xF1,
    OP_POP = 0xF8,
    OP_HLT = 0xFF /* halt */
    /* TODO: add opcodes */
};

/* chunk norris */
struct Norris {
    uchar        *cod; /* bytecode */
    size_t        len; /* used length */
    size_t        cap; /* allocd capanacity */
    struct Values idf; /* identifiers*/
    struct Values ctn; /* constants */
};

void norris_init     (struct Norris *);
int  norris_from_buff(struct Norris *, const uchar *, size_t);
void norris_free     (struct Norris *);
void norris_push_byte(struct Norris *, uchar);
uint norris_push_ctn (struct Norris *, struct DfVal);

#endif /* FLATVM_NORRIS_H */
