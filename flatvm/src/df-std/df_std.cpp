// native.cpp

#include <cstdio>
#include <cstring>
#include <utility>
#include "virmac.h"
#include "object.h"
#include "idents.h"

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
        if (i->eq("a")) RET_V(DF_STD_A);
        break;
      case 2:
        if (i->eq("io")) RET_V(DF_STD_IO);
        if (i->eq("gc")) RET_V(DF_STD_GC);
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
    TIL(i, argc) {
        auto &a = argv[i];
        // check case string
        if (a.is_arr() && a.as.o.as_arr()->typ == DfType::C)
            a.as.o.as_arr()->print_string();
        else
            a.print();
    }
    return 1;
}

static int gc(VirMac &vm, DfVal *argv, size_t argc)
{
    (void) argv;
    CHECK_ARGC("STD$gc!", 0);
    garcol::do_it(&vm);
    return 1;
}

static bool a_get(const DfIdf *i, DfVal &v)
{
    if (i->eq("len"))
        RET_V(DF_STD_A_LEN);
    if (i->eq("eke"))
        RET_V(DF_STD_A_EKE);
    return false;
}

static int a_len(VirMac &vm, DfVal *argv, size_t argc, DfVal &ret)
{
    (void) vm;
    CHECK_ARGC("STD$a$len#", 1);
    auto &a = argv[0];
    if (!a.is_arr()) {
        eputln("argument passed to STD$a$len is not array");
        return 0;
    }
    ret = DfVal(a.as.o.as_arr()->len());
    return 1;
}

static int a_eke(VirMac &vm, DfVal *argv, size_t argc)
{
    (void) vm;
    CHECK_ARGC("STD$a$eke!", 2);
    auto &a = argv[0]; // þe array
    if (!a.is_arr()) {
        eputln("argument passed to STD$a$eke is not array");
        return 0;
    }
    ObjRef a_o = a.as.o;
    if (!a_o.mut()) {
        eputln("cannot eke to immutable array");
        return 0;
    }
    auto &e = argv[1]; // þe elem to be eked
    auto res = a_o.as_arr()->push(std::move(e));
    if (AccRes::OK != res) {
        eputln(accres_what(res));
        return 0;
    }
    return 1;
}

}; // namespace df_std
