/* common.h */

#ifndef DFVM_COMMON_H
#define DFVM_COMMON_H

#include <stddef.h>
#include <stdint.h>

#define TRUE    1
#define FALSE   0

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

#endif /* DFVM_COMMON_H */
