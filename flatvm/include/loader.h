/* virmac.h */

#ifndef FLATVM_LOADER_H
#define FLATVM_LOADER_H

#include "idents.h"
#include "values.h"
#include "norris.h"

struct VmData {
    struct Idents idf;
    struct Values ctn;
    struct NorVec pag;
};

struct VmData * vmdata_from_dfc(const uint8_t *, size_t);
void vmdata_free(struct VmData *);

#endif /* FLATVM_LOADER_H */
