/* htable.h */

#ifndef DFVM_HTABLE_H
#define DFVM_HTABLE_H

#include "common.h"
#include "values.h"

struct Entry {
    struct ObjStr *k; /* key */
    struct DfVal   v; /* value */
};

struct HashTable {
    size_t        siz;
    size_t        cap;
    struct Entry *ent;
};

void htable_init(struct HashTable *);


#endif /* DFVM_HTABLE_H */
