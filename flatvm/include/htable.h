/* htable.h */

#ifndef FLATVM_HTABLE_H
#define FLATVM_HTABLE_H

#include "common.h"

class DfIdf;
class DfVal;
class Hentry;

class Htable {
    typedef const DfIdf * key_t;
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
    bool get(key_t, DfVal &) const;
    bool set(key_t, DfVal &&);
    void print() const;
};

#endif /* FLATVM_HTABLE_H */
