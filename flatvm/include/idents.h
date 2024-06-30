/* idents.h */

#ifndef FLATVM_IDENTS_H
#define FLATVM_IDENTS_H

#include "common.h"
#include <new>

class DfIdf {
  private:
    // owns str
    uint8_t *str; // NUL-term'd, so it's len+1
    uint32_t len; // len of printable chars
    uint32_t hsh;
  public:
    DfIdf(DfIdf &&);
    DfIdf(cbyte_p, size_t);
    ~DfIdf();
    uint32_t get_hash() const;
    size_t get_len() const;
    void print() const;
    void eprint() const;
    bool eq(const char *) const;
    DfIdf & operator=(DfIdf &&);
};

#endif /* FLATVM_IDENTS_H */
