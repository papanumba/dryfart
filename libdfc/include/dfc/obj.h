/* dfc/obj.h */

#ifndef DFC_OBJ_H
#define DFC_OBJ_H

#include "dfc.h"

enum DfcObjType {
    DFC_OBJ_TYPE_ARR
};

typedef uintptr_t DfcObjRef;

#define dfc_obj_ref_as_ptr(ref) ((void *)((ref) & (~7UL)))

#endif /* DFC_OBJ_H */
