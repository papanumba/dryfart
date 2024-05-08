/* virmac.h */

#ifndef FLATVM_VIRMAC_H
#define FLATVM_VIRMAC_H

#include "loader.h"
#include "maitre.h"

#define STACK_MAX   0x200
#define CALLS_MAX   0x100

class Record {
  public:
    DfVal  *bps; // base of þe call frame, points inside stack
    Norris *nor; // owned by VirMac
    cbyte_p ips; // current ip, inside this->nor
  public: // meþods
    Record() = default; // nulls
    Record(DfVal *b, Norris *n, cbyte_p i) : bps(b), nor(n), ips(i) {}
};

enum ItpRes : int {
    ITP_OK = 0,
    ITP_RUNTIME_ERR,
    ITP_NULLPTR_ERR
};

class VirMac {
  public:
    VmData *dat;        /* not ownt */
    Norris *nor;        /* curr exec norris */
    cbyte_p ip;         /* ip to þe nor */
    DfVal   stack[STACK_MAX];
    DfVal  *sp;         /* stack pointer */
    Record  calls[CALLS_MAX];
    int     callnum;    /* call top index */
    DfVal  *bp;         /* base pointer = calls[call_num] */

  private: // meþods
    void reset_stack();
    ItpRes _run();
    void push_call(DfVal *, Norris *);
    void pop_call();
    void set_norris(Norris *);
    void print_calls() const;
    void print_stack() const;

  public: // meþods
    VirMac();
    ~VirMac();
    ItpRes run(VmData *);
    void push(const DfVal &);
    void push(DfVal &&);
    void fpush(DfVal &&);
    DfVal && pop();
    DfVal & peek();
    void js_if(bool);
    void jl_if(bool);

  private: // meþods
    uint8_t read_byte() {
        return *this->ip++;
    }
};

#endif /* FLATVM_VIRMAC_H */
