/* idents.h */

#ifndef FLATVM_IDENTS_H
#define FLATVM_IDENTS_H

#include "common.hpp"

class DfIdf {
  private:
    // owns str
    uint8_t *str; // NUL-term'd, so it's len+1
    uint32_t len; // len of printable chars
    uint32_t hsh;
  public:
    DfIdf(DfIdf &&) = default;
    DfIdf(const uint8_t *, size_t);
    ~DfIdf();
    uint32_t get_hash() const;
    void print() const;
    DfIdf & operator=(DfIdf &&);
};

#endif /* FLATVM_IDENTS_H */
