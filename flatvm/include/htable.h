/* htable.h */

#ifndef FLATVM_HTABLE_H
#define FLATVM_HTABLE_H

#include "common.hpp"
#include "values.h"

class DfIdf;
class Hentry;

class Htable {
  private:
    Hentry *ent;
    size_t  siz;
    size_t  cap;
  private: // meþods
    void grow(size_t);
  public: // meþods
    Htable();
    Htable(Htable &&);
    ~Htable();
    bool get(const DfIdf *, DfVal &) const;
    bool set(const DfIdf *, DfVal &&);
    void print() const;
};

#endif /* FLATVM_HTABLE_H */
