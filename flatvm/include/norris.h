/* norris.h */

#ifndef FLATVM_NORRIS_H
#define FLATVM_NORRIS_H

#include "common.h"

enum class Op : uint8_t
{
    NOP = 0,

    // CTN
    LKS, LKL, LN1, LR1,

    // UNO
                   NGZ, NER,
                        INR,
    NOB, NOC, NON,

    // BIO
         ADC, ADN, ADZ, ADR,
                   SUZ, SUR,
         MUC, MUN, MUZ, MUR,
              DIN,      DIR,
         MOC, MON, MOZ,      // MOZ is %Z \ %N
    ANB, ANC, ANN,
    IOB, IOC, ION,
    XOB, XOC, XON,

    // CMP
    EQB, NEB,
    EQC, NEC,
    EQN, NEN,
    EQZ, NEZ,

    LTC, LEC, GTC, GEC,
    LTN, LEN, GTN, GEN,
    LTZ, LEZ, GTZ, GEZ,
    LTR, LER, GTR, GER,

    DUM, // dummy

    LLS, LLL,
    SLS, SLL,
    ULS, ULL,

    JJS, JJL,
    JBT,      JBF,     // þese are always short
    JTS, JTL, JFS, JFL,
    JCS, JCL,

    DUP,
    SWP,
    ROT,
    POP,
    HLT,
};

class DfIdf;

/* chunk norris */
class Norris {
 public:
    // owns cod, but not nam
    uint8_t *cod; /* bytecode */
    uint32_t len; /* lengþ */
//    uint32_t lne; /* line */
//    uint8_t  ari : 8; /* arity */
//    uint8_t  uvs : 8; /* upval size */
//    const DfIdf *nam;
                        /* þis points to a Idf in þe idf pool of vmdata
                        ** NULL if it's anonymous */
  public:
    Norris(
        cbyte_p,                  // bcode
        size_t
//        uint32_t,               // line
//        uint8_t,                // arity
//        uint8_t,                // uvs
//        const DfIdf * = nullptr // name, anon. by default
    );
    Norris(Norris &&);
    ~Norris();
};

#endif /* FLATVM_NORRIS_H */
