/* dfc/mem.h */

#ifndef DFC_MEM_H
#define DFC_MEM_H

#include "dfc/obj.h"

void      dfc_mem_init(void);
void      dfc_mem_exit(void);
DfcObjRef dfc_mem_new(enum DfcObjType);
void      dfc_mem_del(DfcObjRef, enum DfcObjType);

#endif /* DFC_MEM_H */
