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
void * realloc_or_free(void *, size_t);
void eput  (const char *);
void eputln(const char *);
NORET void todo  (const char *);
NORET void panic (const char *);
NORET void unreachable(void);

// data-serializing functions

static inline uint8_t read_u8(const uint8_t **rpp)
{
    uint8_t val = **rpp;
    *rpp += 1;
    return val;
}

static inline int8_t read_i8(const uint8_t **rpp)
{
    union { uint8_t u; int8_t s; } aux;
    aux.u = read_u8(rpp);
    return aux.s;
}

static inline uint16_t read_u16(const uint8_t **rpp)
{
    const uint8_t *rp = *rpp;
    uint16_t val = (rp[0] << 8) | rp[1];
    *rpp += 2;
    return val;
}

static inline int16_t read_i16(const uint8_t **rpp)
{
    union { uint16_t u; int16_t s; } aux;
    aux.u = read_u16(rpp);
    return aux.s;
}

static inline uint32_t read_u32(const uint8_t **rpp)
{
    const uint8_t *rp = *rpp;
    uint32_t val =
        (rp[0] << 24) |
        (rp[1] << 16) |
        (rp[2] <<  8) |
         rp[3];
    *rpp += 4;
    return val;
}

static inline int32_t read_i32(const uint8_t **rpp)
{
    union { uint32_t u; int32_t s; } aux;
    aux.u = read_u32(rpp);
    return aux.s;
}

static inline float read_f32(const uint8_t **rpp)
{
    union { uint32_t u; float f; } aux;
    aux.u = read_u32(rpp);
    return aux.f;
}

#ifdef __cplusplus
}
#endif

#undef NORET

static inline size_t at_least_8(size_t c) {
    return (c < 8 ? 8 : c);
}

#endif /* FLATVM_COMMON_H */
