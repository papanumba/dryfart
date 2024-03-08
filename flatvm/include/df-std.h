/* df-std.h */

#ifndef FLATVM_DF_STD_H
#define FLATVM_DF_STD_H

#include "idents.h"
#include "values.h"

struct VirMac;

enum NatTb {
    DF_STD    = 0,
    DF_STD_IO = 1,
};

enum NatPcTag {
    DF_STD_IO_PUT = 0,
};

struct NatPc {
    enum NatPcTag tag;
    int (*exec)(struct VirMac *, struct DfVal *, size_t);
};

void nat_tb_print(enum NatTb);
int  nat_tb_get  (enum NatTb, struct DfIdf *, struct DfVal *);

void         nat_pc_print(enum NatPcTag);
struct NatPc nat_pc_from (enum NatPcTag);

#endif /* FLATVM_DF_STD_H */
