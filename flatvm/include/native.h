// native.h

#ifndef FLATVM_NATIVE_H
#define FLATVM_NATIVE_H

class DfVal;
class DfIdf;
class VirMac;

enum NatTblTag {
    DF_STD    = 0,
    DF_STD_IO = 1,
    DF_STD_A,
};

class NatTbl {
  typedef const DfIdf * key_t;
  private:
    void *priv;
  public:
    NatTblTag tag;
  public:
    NatTbl(NatTblTag);
    NatTbl(NatTbl &&);
    ~NatTbl();
    bool get(key_t, DfVal &) const;
    bool set(key_t, DfVal &&);
    void print() const;
};

enum NatProTag {
    DF_STD_IO_PUT = 0,
    DF_STD_GC,
    DF_STD_A_EKE,
};

class NatPro {
  public:
    NatProTag tag;
    int (*exec)(VirMac &, DfVal *, size_t);
  public:
    NatPro(NatProTag);
    void print() const;
};

enum NatFunTag {
    DF_STD_A_LEN,
    DF_STD_IO_READFILE,
};

class NatFun {
  public:
    NatFunTag tag;
    int (*eval)(
        VirMac &,
        DfVal *,
        size_t,
        DfVal & // return
    );
  public:
    NatFun(NatFunTag);
    void print() const;
};

namespace NatFactory {
    ObjRef get(NatTblTag);
    ObjRef get(NatFunTag);
    ObjRef get(NatProTag);
    void mark_all();
};

#endif /* FLATVM_NATIVE_H */
