/* common.h */

#ifndef FLATVM_COMMON_H
#define FLATVM_COMMON_H

// laziness at its peak
#define FOR(var, start, end) \
for (size_t var = start; var < end; ++var)
#define TIL(var, end) FOR(var, 0, end)

#ifdef __cplusplus

#include <cstddef>
#include <cstdint>

#define LOOP while (true)

#else // C

#include <stddef.h>
#include <stdint.h>
#include <assert.h>

#define TRUE    1
#define FALSE   0
#define MACRO_STMT(s)  do {s} while (FALSE)

#endif // C++


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
typedef const uint8_t * cbyte_p;

#ifdef __cplusplus
#define NORET [[ noreturn ]]
extern "C" {
#else
#define NORET _Noreturn
#endif

int bytearr_cmp(cbyte_p, cbyte_p, size_t);
void eput  (const char *);
void eputln(const char *);
NORET void todo  (const char *);
NORET void panic (const char *);
NORET void unreachable(void);

#ifdef __cplusplus
}
#endif

#undef NORET

static inline size_t at_least_8(size_t c) {
    return (c < 8 ? 8 : c);
}

#endif /* FLATVM_COMMON_H */
