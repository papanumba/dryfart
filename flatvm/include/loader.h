/* virmac.h */

#ifndef FLATVM_LOADER_H
#define FLATVM_LOADER_H

#include "common.hpp"
#include "dynarr.h"
#include "values.h"
#include "norris.h"

class VmData {
  public:
    // owns everyþing in þe dynarrs
    DynArr<DfIdf>  idf;
    DynArr<DfVal>  ctn;
    DynArr<Norris> pag;
    // meþods
    VmData(const uint8_t *, size_t);
    ~VmData();
};

#endif /* FLATVM_LOADER_H */
