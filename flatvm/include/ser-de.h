/* ser-de.h */

#ifndef FLATVM_SER_DE_H
#define FLATVM_SER_DE_H

// serialize in BIG endian

#include "common.h"

template <typename T> // T must be some of uint(8|16|32|64)_t
static inline T read_u(cbyte_p *rpp)
{
    auto rp = *rpp;
    T val = 0;
    TIL(i, sizeof(T)) {
        val <<= 8;
        val |= rp[i];
    }
    *rpp += sizeof(T);
    return val;
}

static inline uint8_t read_u8(cbyte_p *rpp)
{
    return read_u<uint8_t>(rpp);
}

static inline int8_t read_i8(cbyte_p *rpp)
{
    union { uint8_t u; int8_t s; } aux;
    aux.u = read_u8(rpp);
    return aux.s;
}

static inline uint16_t read_u16(cbyte_p *rpp)
{
    return read_u<uint16_t>(rpp);
}

static inline int16_t read_i16(cbyte_p *rpp)
{
    union { uint16_t u; int16_t s; } aux;
    aux.u = read_u16(rpp);
    return aux.s;
}

static inline uint32_t read_u32(cbyte_p *rpp)
{
    return read_u<uint32_t>(rpp);
}

static inline int32_t read_i32(cbyte_p *rpp)
{
    union { uint32_t u; int32_t s; } aux;
    aux.u = read_u32(rpp);
    return aux.s;
}

static inline double read_f64(cbyte_p *rpp)
{
    union { uint64_t u; double d; } aux;
    aux.u = read_u<uint64_t>(rpp);
    return aux.d;
}

#endif /* FLATVM_SER_DE_H */
