/* virmac.h */

#ifndef FLATVM_VIRMAC_H
#define FLATVM_VIRMAC_H

#include "common.h"
#include "loader.h"
#include "norris.h"
#include "values.h"

#define STACK_MAX   0x200
#define CALLS_MAX   0x100

typedef const uint8_t * cbyte_p;

class Record {
  public:
    DfVal  *bps = nullptr; // base of þe call frame
    Norris *nor = nullptr;
    cbyte_p ips = nullptr;
};

enum class ItpRes : int {
    OK = 0,
    RUNTIME_ERR,
    NULLPTR_ERR
};

class VirMac {
  private:
    VmData *dat;         /* not ownt */
    Norris *nor;         /* curr exec norris */
    cbyte_p ip;          /* ip to þe nor */
    DfVal   stack[STACK_MAX];
    DfVal  *sp;          /* stack pointer */
    Record  calls[CALLS_MAX];
    int     callnum;     /* call top index */
    DfVal  *bp;          /* base pointer = calls[call_num] */

  public: // meþods
    ItpRes run(VmData *);
    void push(DfVal &&);
    DfVal && pop();
    DfVal & peek();

  private: // meþods
    uint8_t read_byte() {
        return *this->ip++;
    }
};

#endif /* FLATVM_VIRMAC_H */
