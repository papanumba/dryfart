/* htable.h */

#ifndef FLATVM_HTABLE_H
#define FLATVM_HTABLE_H

#include "common.h"
#include "idents.h"
#include "values.h"

struct Hentry {
    struct DfIdf *k; /* key */
    struct DfVal  v; /* value */
};

struct Htable {
    struct Hentry *ent;
    size_t         siz;
    size_t         cap;
};

void htable_init (struct Htable *);
void htable_free (struct Htable *);
int  htable_get  (struct Htable *, struct DfIdf *, struct DfVal *);
int  htable_set  (struct Htable *, struct DfIdf *, struct DfVal);
void htable_print(struct Htable *);
/*int  htable_del (struct Htable *, struct DfIdf *);*/

#endif /* FLATVM_HTABLE_H */
