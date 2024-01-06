/* falloc.h */

#ifndef FLATVM_FALLOC_H
#define FLATVM_FALLOC_H

#include "common.h"
#include "object.h"

void * falloc_alloc(void);
void   falloc_free (void *);
void   falloc_init(void);
void   falloc_exit(void);
size_t falloc_objs_num(void);
void   falloc_sweep(void);

#endif /* FLATVM_FALLOC_H */
