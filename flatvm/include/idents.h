/* idents.h */

#ifndef FLATVM_IDENTS_H
#define FLATVM_IDENTS_H

#include "common.h"
#include "dynarr.h"

/* once malloc'd, stays const */
struct DfIdf {
    char    *str; /* NUL-term'd, so len+1 */
    size_t   len; /* counts only non-NUL  */
    uint32_t hsh;
};

STRUCT_DYNARR(Idents, struct DfIdf);

struct DfIdf dfidf_from_chars(const char *, size_t);
void dfidf_free (struct DfIdf *);
void dfidf_print(struct DfIdf *);
void idents_init(struct Idents *);
void idents_w_cap(struct Idents *, size_t);
void idents_free(struct Idents *);
void idents_push(struct Idents *, struct DfIdf);

#endif /* FLATVM_IDENTS_H */
