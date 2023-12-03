/* htable.h */

#ifndef DFVM_HTABLE_H
#define DFVM_HTABLE_H

#include "common.h"
#include "object.h"
#include "values.h"

struct Hentry {
    struct ObjIdf *k; /* key */
    struct DfVal   v; /* value */
};

struct Htable {
    struct Hentry *ent;
    size_t         siz;
    size_t         cap;
};

void htable_init(struct Htable *);
void htable_free(struct Htable *);
int  htable_get (struct Htable *, struct ObjIdf *, struct DfVal *);
int  htable_set (struct Htable *, struct ObjIdf *, struct DfVal);
/*int  htable_del (struct Htable *, struct ObjIdf *);*/


#endif /* DFVM_HTABLE_H */
