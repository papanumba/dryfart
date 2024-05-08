// native.cpp

#include <cstdio>
#include <cstring>
#include <utility>
#include "common.h"
#include "virmac.h"
#include "native.h"
#include "object.h"
#include "maitre.h"
#include "dynarr.h"
//#include "garcol.h"
//#include "df-lib.h"

#define CHECK_ARGC(name, a) do { \
    if (argc != a) { \
        fprintf(stderr, "not rite numba of args calling %s, must be %u\n", \
            name, a); \
        return false; \
    } \
} while(false)

#define RET_V(tag) do { \
    v = DfVal(NatFactory::get(tag)); \
    return true; \
} while (false)

namespace df_std {

static bool get(const DfIdf *i, DfVal &v)
{
    switch (i->get_len()) {
      case 1:
        if (i->eq("a"))
            RET_V(DF_STD_A);
        break;
      case 2:
        if (i->eq("io"))
            RET_V(DF_STD_IO);
        if (i->eq("gc"))
//            RET_V_PRO(DF_STD_IO);
            todo("nat pro fac");
        break;
      default:
        eput("ERROR: can't get field $");
        i->eprint();
        eputln("from STD");
    }
    return false;
}

static bool io_get(const DfIdf *i, DfVal &v)
{
    if (i->eq("put"))
        RET_V(DF_STD_IO_PUT);
    return false;
}

static int io_put(VirMac &vm, DfVal *argv, size_t argc)
{
    (void)vm;
    CHECK_ARGC("STD$io$put!", 1);
    // check case string
    auto &a = argv[0];
    if (a.is_arr() && a.as.o.as_arr()->typ == DfType::C)
        a.as.o.as_arr()->print_string();
    else
        a.print();
    return 1;
}

#if 0
static bool a_get(const DfIdf *i, DfVal &v)
{
    if (i->eq("len")) {
        v->type = VAL_O;
        v->as.o = (void *) objfun_new_nat(DF_STD_A_LEN);
        return true;
    }
    if (i->eq("eke")) {
        v->type = VAL_O;
        v->as.o = (void *) objpro_new_nat(DF_STD_A_EKE);
        return true;
    }
    return false;
}

#endif

}; // namespace df_std
