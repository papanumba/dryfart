/* htable.h */

#ifndef FLATVM_HTABLE_H
#define FLATVM_HTABLE_H

#include "common.h"

class DfIdf;
class DfVal;
class Hentry;

class Htable {
    typedef const DfIdf * key_t;
    class HtIter {
        Hentry *e;
      public:
        HtIter(Hentry *f) : e(f) {}
        key_t key() const;
        const DfVal & val() const;
        DfVal & val();
        bool operator!=(const HtIter &that) const;
        void next();
    };
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
    HtIter begin();
    HtIter end() const;
    void next(HtIter &) const; // advances þe iterator
};

#endif /* FLATVM_HTABLE_H */
