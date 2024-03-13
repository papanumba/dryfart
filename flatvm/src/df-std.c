/* df-std.c */

#include <stdio.h>
#include <string.h>
#include "common.h"
#include "df-std.h"
#include "object.h"
#include "garcol.h"

/* struct DfIdf *ip */
#define IDF_EQ(ip, s)  (strcmp((ip)->str, s) == 0)

#define CHECK_ARGC(name, a) \
do { \
    if (argc != a) { \
        fprintf(stderr, "not rite numba of args calling %s, must be %u\n", \
            name, a); \
        return FALSE; \
    } \
} while(FALSE)

static int df_std_get   (struct DfIdf *, struct DfVal *);
static int df_std_io_get(struct DfIdf *, struct DfVal *);

static int df_std_io_put(struct VirMac *, struct DfVal *, size_t);
static int df_std_gc    (struct VirMac *, struct DfVal *, size_t);

void nat_tb_print(enum NatTb t)
{
    switch (t) {
      case DF_STD:    printf("<STD>");    break;
      case DF_STD_IO: printf("<STD$io>"); break;
    }
}

int nat_tb_get(enum NatTb t, struct DfIdf *k, struct DfVal *v)
{
    switch (t) {
      case DF_STD:
        return df_std_get(k, v);
      case DF_STD_IO:
        return df_std_io_get(k, v);
      default: unreachable();
    }
    return FALSE;
}

void nat_pc_print(enum NatPcTag t)
{
    switch (t) {
      case DF_STD_IO_PUT: printf("! \"STD$io$put\""); break;
      case DF_STD_GC: printf("! STD$gc"); break;
    }
}

struct NatPc nat_pc_from(enum NatPcTag t)
{
    struct NatPc np;
    np.tag = t;
    switch (t) {
      case DF_STD_IO_PUT: np.exec = df_std_io_put;  break;
      case DF_STD_GC:     np.exec = df_std_gc;      break;
    }
    return np;
}

static int df_std_get(struct DfIdf *i, struct DfVal *v)
{
    switch (i->len) {
      case 2:
        if (IDF_EQ(i, "io")) {
            v->type = VAL_O;
            v->as.o = (void *) objtbl_new_nat(DF_STD_IO);
            return TRUE;
        } else if (IDF_EQ(i, "gc")) {
            v->type = VAL_O;
            v->as.o = (void *) objpro_new_nat(DF_STD_GC);
            return TRUE;
        }
        break;
      default:
        fprintf(stderr, "ERROR: can't get field $%s from STD\n", i->str);
    }
    return FALSE;
}

static int df_std_io_get(struct DfIdf *i, struct DfVal *v)
{
    if (IDF_EQ(i, "put")) {
        v->type = VAL_O;
        v->as.o = (void *) objpro_new_nat(DF_STD_IO_PUT);
        return TRUE;
    }
    return FALSE;
}

static int df_std_io_put(struct VirMac *vm, struct DfVal *argv, size_t argc)
{
    (void)(vm);
    CHECK_ARGC("STD$io$put", 1);
    values_print(&argv[0]);
    return TRUE;
}

static int df_std_gc(struct VirMac *vm, struct DfVal *argv, size_t argc)
{
    (void)(argv);
    CHECK_ARGC("STD$gc", 0);
    garcol_do(vm);
    return TRUE;
}
