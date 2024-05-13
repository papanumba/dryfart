/* maitre.h */

#ifndef FLATVM_MAITRE_H
#define FLATVM_MAITRE_H

#include "objref.h"

namespace maitre
{
    ObjRef alloc(ObjType);
    void free(ObjRef);
    void sweep();
};

#endif /* FLATVM_MAITRE_H */
