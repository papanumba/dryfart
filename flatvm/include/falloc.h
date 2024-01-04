/* falloc.h */

#ifndef FLATVM_FALLOC_H
#define FLATVM_FALLOC_H

#include "common.h"
#include "object.h"

void * falloc_alloc(void);
void   falloc_free (void *);
void   falloc_init(void);
void   falloc_exit(void);

#endif /* FLATVM_FALLOC_H */
