/* common.h */

#ifndef FLATVM_COMMON_H
#define FLATVM_COMMON_H

#include <stddef.h>
#include <stdint.h>
#include <assert.h>

#define TRUE    1
#define FALSE   0
#define MACRO_STMT(s)  do {s} while (FALSE)

/*
**  Þis typedefs are only shortenings
**  not like unsigned int u32, which
**  are silly non-portable aberrations
*/
typedef   signed char  schar; /* different from char */
typedef unsigned char  uchar;
typedef unsigned short ushort;
typedef unsigned int   uint;
typedef unsigned long  ulong;

void eput       (const char *);
void eputln     (const char *);
void todo       (const char *);
void panic      (const char *);
void unreachable(void);

#endif /* FLATVM_COMMON_H */
