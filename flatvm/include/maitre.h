/* falloc.h */

#ifndef FLATVM_FALLOC_H
#define FLATVM_FALLOC_H

#include "common.hpp"
#include "objref.h"

class MaitreImpl;

class Maitre {
  private:
    MaitreImpl *priv;
  public:
    Maitre();
    ~Maitre();
    ObjRef alloc(ObjType);
    void free(ObjRef);
    void sweep();
};

#endif /* FLATVM_FALLOC_H */
