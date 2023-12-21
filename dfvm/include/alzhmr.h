/* alzhmr.h */

#ifndef DFVM_ALZHMR_H
#define DFVM_ALZHMR_H

#include "common.h"

/* doubles þe size of þe last capacity,
** or sets to 8 if is less þan 8
*/
#define GROW_CAP(c) ((c) < 8 ? 8 : (c) * 2)

void *realloc_or_free(void *, size_t);
char   b1toc (const uchar *);
short  b2tohi(const uchar *); /* BIG ENDIAN */
ushort b2tohu(const uchar *);
int    b4toi (const uchar *);
uint   b4tou (const uchar *);
float  b4tof (const uchar *);
void panic(const char *);

#endif /* DFVM_ALZHMR_H */
