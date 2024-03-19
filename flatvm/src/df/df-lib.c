/* df-lib.c */

#include <stdio.h>
#include <string.h>
#include "df-lib.h"
#include "virmac.h"
#include "object.h"
#include "garcol.h"

#define CHECK_ARGC(name, a) \
do { \
    if (argc != a) { \
        fprintf(stderr, "not rite numba of args calling %s, must be %u\n", \
            name, a); \
        return FALSE; \
    } \
} while(FALSE)

int df_std_io_put(struct VirMac *vm, struct DfVal *argv, size_t argc)
{
    (void)(vm);
    CHECK_ARGC("STD$io$put", 1);
    values_print(&argv[0]);
    return TRUE;
}

int df_std_gc(struct VirMac *vm, struct DfVal *argv, size_t argc)
{
    (void)(argv);
    CHECK_ARGC("STD$gc", 0);
    garcol_do(vm);
    return TRUE;
}

int df_std_a_eke(struct VirMac *vm, struct DfVal *argv, size_t argc)
{
    (void)(vm);
    CHECK_ARGC("STD$a$eke", 2);
    if (val2type(&argv[0]) != DFTYPE_A) {
        eputln("first arg to STD$a$eke is not _%");
        return FALSE;
    }
    return objarr_try_push(OBJ_AS_ARR(argv[0].as.o), &argv[1]);
}

int df_std_a_len(
    struct VirMac *vm,
    struct DfVal  *argv,
    size_t         argc,
    struct DfVal  *ret)
{
    (void)(vm);
    CHECK_ARGC("STD$a$len", 1);
    if (val2type(&argv[0]) != DFTYPE_A) {
        eputln("first arg to STD$a$len is not _%");
        return FALSE;
    }
    ret->type = VAL_N;
    ret->as.n = objarr_len(OBJ_AS_ARR(argv[0].as.o));
    return TRUE;
}
