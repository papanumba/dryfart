/* bitarr.h */

#ifndef FLATVM_BITARR_H
#define FLATVM_BITARR_H

#include "dynarr.h"

class BitArr {
  private:
    DynArr<uint8_t> _buf;
    das_t           _len; // in bits
  public:
    BitArr() = default;
    BitArr(BitArr &&) = default;
    BitArr(das_t);
    ~BitArr() = default;
    // getters
    das_t len() const;
    bool is_empty() const;
    // modifiers
    void push(bool);
    bool pop();
    void set(das_t, bool);
    // operators
    bool operator[](das_t i) const;
    BitArr & operator=(BitArr &&that) = default;
};

#endif // FLATVM_BITARR_H
