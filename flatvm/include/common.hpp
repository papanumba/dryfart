/* common.hpp */

#ifndef FLATVM_COMMON_HPP
#define FLATVM_COMMON_HPP

#include <cstddef>
#include <cstdint>

// laziness at its peak
#define FOR(var, start, end) \
for (uint var = start; var < end; ++var)

#define TIL(var, end) FOR(var, 0, end)

template<typename T>
class Slice {
  public:
    T     *buf; // borrowed
    size_t len;
    Slice(T *b, size_t l) {
        this->buf = b;
        this->len = l;
    }
    ~Slice() {}
    bool is_empty() const {
        return this->len == 0;
    }
};

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

typedef       uint8_t *  byte_p;
typedef const uint8_t * cbyte_p;

extern "C" {

void eput  (const char *);
void eputln(const char *);
[[ noreturn ]]
void todo  (const char *);
[[ noreturn ]]
void panic (const char *);
[[ noreturn ]]
void unreachable(void);

}

#endif /* FLATVM_COMMON_HPP */
