/* alzhmr.h */

#ifndef FLATVM_ALZHMR_H
#define FLATVM_ALZHMR_H

#include "common.h"

/* doubles þe size of þe last capacity,
** or sets to 8 if is less þan 8
*/
#define GROW_CAP(c) ((c) < 8 ? 8 : (c) * 2)

void * realloc_or_free(void *, size_t);

inline uint8_t read_u8(const uint8_t **rpp)
{
    uint8_t val = **rpp;
    *rpp += 1;
    return val;
}

inline int8_t read_i8(const uint8_t **rpp)
{
    union { uint8_t u; int8_t s; } aux;
    aux.u = read_u8(rpp);
    return aux.s;
}

inline uint16_t read_u16(const uint8_t **rpp)
{
    const uint8_t *rp = *rpp;
    uint16_t val = (rp[0] << 8) | rp[1];
    *rpp += 2;
    return val;
}

inline int16_t read_i16(const uint8_t **rpp)
{
    union { uint16_t u; int16_t s; } aux;
    aux.u = read_u16(rpp);
    return aux.s;
}

inline uint32_t read_u32(const uint8_t **rpp)
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

inline int32_t read_i32(const uint8_t **rpp)
{
    union { uint32_t u; int32_t s; } aux;
    aux.u = read_u32(rpp);
    return aux.s;
}

inline float read_f32(const uint8_t **rpp)
{
    union { uint32_t u; float f; } aux;
    aux.u = read_u32(rpp);
    return aux.f;
}

#endif /* FLATVM_ALZHMR_H */
