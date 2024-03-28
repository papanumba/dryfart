/* native.h */

#ifndef FLATVM_NATIVE_H
#define FLATVM_NATIVE_H

class DfVal;
class DfIdf;
class VirMac;

enum NatTb {
    DF_STD    = 0,
    DF_STD_IO = 1,
    DF_STD_A,
};

enum NatPcTag {
    DF_STD_IO_PUT = 0,
    DF_STD_GC,
    DF_STD_A_EKE,
};

struct NatPc {
    enum NatPcTag tag;
    int (*exec)(VirMac *, DfVal *, size_t);
};

enum NatFnTag {
    DF_STD_A_LEN
};

struct NatFn {
    enum NatFnTag tag;
    int (*eval)(
        VirMac *,
        DfVal *,
        size_t,
        DfVal *
    );
};

void nat_tb_print(enum NatTb);
int  nat_tb_get  (enum NatTb, DfIdf *, DfVal *);

void         nat_pc_print(enum NatPcTag);
struct NatPc nat_pc_from (enum NatPcTag);

void         nat_fn_print(enum NatFnTag);
struct NatFn nat_fn_from (enum NatFnTag);

#endif /* FLATVM_NATIVE_H */
