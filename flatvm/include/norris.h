/* norris.h */

#ifndef FLATVM_NORRIS_H
#define FLATVM_NORRIS_H

#include "common.h"
#include "idents.h"

enum Op : uint8_t {
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

    OP_NEG = 0x10,
    OP_ADD = 0x11,
    OP_SUB = 0x12,
    OP_MUL = 0x13,
    OP_DIV = 0x14,
    OP_INV = 0x15,
    OP_INC = 0x16,
    OP_DEC = 0x17,

    OP_CEQ = 0x18,
    OP_CNE = 0x19,
    OP_CLT = 0x1A,
    OP_CLE = 0x1B,
    OP_CGT = 0x1C,
    OP_CGE = 0x1D,

    OP_NOT = 0x20,
    OP_AND = 0x21,
    OP_IOR = 0x22,

    OP_LLS = 0x44,
    OP_SLS = 0x45,
    OP_ULS = 0x46,
    OP_LLL = 0x47,
    OP_SLL = 0x48,
    OP_ULL = 0x49,

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

    OP_AMN = 0x60,
    OP_APE = 0x61,
    OP_AGE = 0x62,
    OP_ASE = 0x63,

    OP_TMN = 0x70,
    OP_TSF = 0x71,
    OP_TGF = 0x72,

    OP_PMN = 0x80,
    OP_PCL = 0x82,
    OP_FMN = 0x88,
    OP_FCL = 0x89,
    OP_LUV = 0x8F,

    OP_CAB = 0xE2,
    OP_CAC = 0xE4,
    OP_CAN = 0xE6,
    OP_CAZ = 0xE8,
    OP_CAR = 0xEA,

    OP_RET = 0xF0,
    OP_END = 0xF1,
    OP_DUP = 0xF4,
    OP_POP = 0xF8,
    OP_HLT = 0xFF
    /* TODO: add opcodes */
};

/* chunk norris */
class Norris {
 public:
    // owns cod, but not nam
    uint8_t *cod; /* bytecode */
    uint32_t len; /* lengþ */
    uint32_t lne; /* line */
    uint8_t ari : 8; /* arity */
    uint8_t uvs : 8; /* upval size */
    const DfIdf *nam;  /* þis points to a Idf in þe idf pool of vmdata
                        ** NULL if it's anonymous */
  public:
    Norris(
        cbyte_p,                // bcode
        size_t,
        uint32_t,               // line
        uint8_t,                // arity
        uint8_t,                // uvs
        const DfIdf * = nullptr // name, anon. by default
    );
    Norris(Norris &&);
    ~Norris();
};

#endif /* FLATVM_NORRIS_H */
