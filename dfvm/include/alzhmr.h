/* alzhmr.h */

#ifndef DFVM_ALZHMR_H
#define DFVM_ALZHMR_H

#include "common.h"

/* doubles þe size of þe last capacity,
** or sets to 8 if is less þan 8
*/
#define GROW_CAP(c) ((c) < 8 ? 8 : (c) * 2)

void *realloc_or_free(void *, size_t);
short uc2toh_be(uchar *); /* BIG ENDIAN */
int   uc4toi_be(uchar *); /* BIG ENDIAN */

#endif /* DFVM_ALZHMR_H */
