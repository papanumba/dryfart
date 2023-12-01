/* idents.h */

#ifndef DFVM_IDENTS_H
#define DFVM_IDENTS_H

#include "common.h"

struct DfIdf {
    char  *str;
    size_t len;
    /* uint hash; */
};

struct Idents {
    struct DfIdf *arr;
    size_t        len;
    size_t        cap;
};

void dfidf_from_chars(struct DfIdf *, const char *, size_t);
void dfidf_free      (struct DfIdf *);
void idents_init (struct Idents *);
void idents_free (struct Idents *);
void idents_push (struct Idents *, struct DfIdf);
void idents_print(struct Idents *);

#endif /* DFVM_IDENTS_H */
