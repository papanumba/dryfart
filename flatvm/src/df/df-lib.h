/* df-lib.h */

#ifndef DF_LIB_H
#define DF_LIB_H

struct VirMac;
struct DfVal;

#define DF_PROC(name) \
int df_std_ ## name(struct VirMac *, struct DfVal *, size_t)

DF_PROC(io_put);
DF_PROC(gc);
DF_PROC(a_eke);

#undef DF_PROC

#define DF_FUNC(name) \
int df_std_ ## name(struct VirMac *, struct DfVal *, size_t, struct DfVal *)

DF_FUNC(a_len);

#undef DF_FUNC

#endif /* DF_LIB_H */
