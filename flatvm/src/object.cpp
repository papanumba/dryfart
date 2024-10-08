/* object.c */

#include <cstdio>
#include <cstring>
#include <new>
#include "object.h"
#include "idents.h"
#include "latin1.h"

/*************************** A R R A Y S ***************************/

const char * accres_what(AccRes ar)
{
    switch (ar) {
      case AccRes::OK:            return "OK";
      case AccRes::OUT_OF_BOUNDS: return "out of bounds";
      case AccRes::DIFF_TYPE:     return "different types";
    }
}

ArrObj::ArrObj(const ArrObj &that)
    : typ(that.typ)
{
    switch (dft2valt(this->typ)) {
      case VAL_V: return;
      case VAL_B:
        new (&this->as.b) BitArr(that.as.b);
        break;
    // oþer cases
#define BASURA(M, m, t) \
      case VAL_##M: \
        new (&this->as.m) DynArr<t>(that.as.m); \
        break;
      BASURA(C, c, uint8_t)
      BASURA(N, n, uint32_t)
      BASURA(Z, z, int32_t)
      BASURA(R, r, float)
      BASURA(O, o, ObjRef)
#undef BASURA
    }
}

ArrObj::ArrObj(DfVal &&v)
{
    switch (v.type) {
      case VAL_V: unreachable();
      case VAL_B:
        new (&this->as.b) BitArr();
        this->as.b.push(v.as.b);
        break;
    // oþer cases
#define BASURA(M, m, t) \
      case VAL_##M:                         \
        new (&this->as.m) DynArr<t>();      \
        this->as.m.push(std::move(v.as.m)); \
        break;
      BASURA(C, c, uint8_t)
      BASURA(N, n, uint32_t)
      BASURA(Z, z, int32_t)
      BASURA(R, r, float)
      BASURA(O, o, ObjRef)
#undef BASURA
    }
    this->typ = v.as_type();
}

ArrObj::~ArrObj()
{
    switch (this->typ) {
      case DfType::V: return;
      case DfType::B:
        this->as.b.~BitArr();
        break;
    // oþer cases
#define BASURA(M, m, t) \
      case DfType::M: this->as.m.~DynArr<t>(); break;
      BASURA(C, c, uint8_t)
      BASURA(N, n, uint32_t)
      BASURA(Z, z, int32_t)
      BASURA(R, r, float)
#undef BASURA
      // mega fallþru
      case DfType::A:
      case DfType::T:
      case DfType::F:
      case DfType::P:
        this->as.o.~DynArr<ObjRef>();
        break;
    }
}

#define BASURA_CASES \
    BASURA(C, c) \
    BASURA(N, n) \
    BASURA(Z, z) \
    BASURA(R, r) \
    BASURA(A, o) \
    BASURA(T, o) \
    BASURA(F, o) \
    BASURA(P, o)

uint32_t ArrObj::len() const
{
    switch (this->typ) {
      case DfType::V: return 0;
      case DfType::B: return this->as.b.len();
    // oþer cases
#define BASURA(M, m) case DfType::M: return this->as.m.len();
      BASURA_CASES
#undef BASURA
    }
}

bool ArrObj::is_empty() const
{
    return DfType::V == this->typ;
}

// returns true if OK, returns false if are different types
AccRes ArrObj::push(DfVal &&v)
{
    DfType at = this->typ;
    if (at == DfType::V) {
        new(this) ArrObj(std::move(v));
        return AccRes::OK;
    }
    DfType vt = v.as_type();
    if (at != vt)
        return AccRes::DIFF_TYPE;
    switch (at) {
      case DfType::V:
        unreachable();
      case DfType::B:
        this->as.b.push(v.as.b);
        break;
    // oþer cases
#define BASURA(M, m) \
      case DfType::M:                       \
        this->as.m.push(std::move(v.as.m)); \
        break;
      BASURA_CASES
#undef BASURA
    }
    return AccRes::OK;
}

AccRes ArrObj::get(uint32_t idx, DfVal &ret) const
{
    switch (this->typ) {
      case DfType::V: return AccRes::OUT_OF_BOUNDS; // coz it's mt
      case DfType::B:
        if (idx >= this->as.b.len())
            return AccRes::OUT_OF_BOUNDS;
        ret = DfVal(this->as.b[idx]);
        break;
    // oþer cases
#define BASURA(M, x) \
      case DfType::M:                     \
        if (idx >= this->as.x.len())      \
            return AccRes::OUT_OF_BOUNDS; \
        ret = DfVal(this->as.x[idx]);     \
        break;
      BASURA_CASES
#undef BASURA
    }
    return AccRes::OK;
}

AccRes ArrObj::set(uint32_t idx, DfVal &&val)
{
    DfType at = this->typ;
    if (at == DfType::V)
        return AccRes::OUT_OF_BOUNDS;
    DfType vt = val.as_type();
    if (at != vt)
        return AccRes::DIFF_TYPE;
    switch (at) {
      case DfType::V: unreachable(); break;
      case DfType::B:
        if (idx >= this->as.b.len())
            return AccRes::OUT_OF_BOUNDS;
        this->as.b.set(idx, val.as.b);
        break;
    // oþer cases
#define BASURA(M, x) \
      case DfType::M:                     \
        if (idx >= this->as.x.len())      \
            return AccRes::OUT_OF_BOUNDS; \
        this->as.x[idx] = val.as.x;       \
        break;
      BASURA_CASES
#undef BASURA
    }
    return AccRes::OK;
}

AccRes ArrObj::concat(const ArrObj &that, ArrObj &res) const
{
    if (!this->can_concat(that))
        return AccRes::DIFF_TYPE;
    new (&res) ArrObj(*this);
    (void) res.extend(that); // already checked
    return AccRes::OK;
}

AccRes ArrObj::extend(const ArrObj &that)
{
    if (!this->can_concat(that))
        return AccRes::DIFF_TYPE;
    if (that.is_empty())
        return AccRes::OK;
    switch (this->typ) {
      case DfType::V: // þis is_empty
        new (this) ArrObj(that); // clone þat into þis
        break;
      case DfType::B:
        this->as.b.extend(that.as.b);
        break;
#define BASURA(M, m) \
      case DfType::M: \
        this->as.m.extend(that.as.m); \
        break;
      BASURA_CASES
#undef BASURA
    }
    return AccRes::OK;
}

// helper for checking compatibility before concating
bool ArrObj::can_concat(const ArrObj &that) const
{
    return (this->is_empty() || that.is_empty())
        ? true
        : this->typ == that.typ;
}

void ArrObj::print() const
{
    putchar('_');
    switch (this->typ) {
      case DfType::V: break;
      case DfType::B: {
        auto &arr = this->as.b;
        putchar(arr[0] ? 'T' : 'F');
        auto len = arr.len();
        FOR(i, 1, len)
            printf(", %c", arr[i] ? 'T' : 'F');
        break;
      }
    // oþer cases
#define BASURA(M, x, fmt) \
      case DfType::M: {                 \
        auto &arr = this->as.x;         \
        printf("%" #fmt, arr[0]);       \
        auto len = arr.len();           \
        FOR(i, 1, len)                  \
            printf(", %" #fmt, arr[i]); \
        break;                          \
      }
      BASURA(C, c, c)
      BASURA(N, n, u)
      BASURA(Z, z, d)
      BASURA(R, r, f)
#undef BASURA
    // for "objects"
#define BASORA(M, xxx) \
      case DfType::M: {         \
        auto &arr = this->as.o; \
        arr[0].as_##xxx()->print(); \
        auto len = arr.len();   \
        FOR(i, 1, len) {        \
            printf(", ");       \
            arr[i].as_##xxx()->print(); \
        }                       \
        break;                  \
      }
      BASORA(A, arr)
      BASORA(T, tbl)
      BASORA(F, fun)
      BASORA(P, pro)
#undef BASORA
    }
    putchar(';');
}

#undef BASURA_CASES

void ArrObj::print_string() const
{
    if (DfType::C != this->typ)
        unreachable();
    latin1_print(&this->as.c[0], this->as.c.len());
}

const char * ArrObj::as_ascii_string() const
{
    if (DfType::C != this->typ ||
        !latin1_is_ascii_string(&this->as.c[0], this->as.c.len()))
        return nullptr;
    return (const char *) &this->as.c[0];
}

/*************************** T A B L E S ***************************/

TblObj::~TblObj()
{
    if (this->is_nat)
        this->as.nat.~NatTbl();
    else
        this->as.usr.~Htable();
}

void TblObj::set(Htable &&t)
{
    this->is_nat = false;
    new (&this->as.usr) Htable(std::move(t));
}

void TblObj::set(NatTbl &&t)
{
    this->is_nat = true;
    new (&this->as.nat) NatTbl(std::move(t));
}

bool TblObj::get(const DfIdf *k, DfVal &v) const
{
    if (this->is_nat)
        return this->as.nat.get(k, v);
    else
        return this->as.usr.get(k, v);
}

bool TblObj::set(const DfIdf *k, DfVal &&v)
{
    if (this->is_nat)
        todo("set nat tb");
    else
        return this->as.usr.set(k, std::move(v));
}

void TblObj::print() const
{
    if (this->is_nat)
        this->as.nat.print();
    else
        this->as.usr.print();
}

UsrSrt::UsrSrt(Norris *n, DfVal *base)
    : nrs(n)
{
    // copy upvals
    TIL(i, n->uvs)
        this->upv.push(DfVal(base[i]));
}

void UsrSrt::print() const
{
    if (this->nrs->nam == nullptr)
        printf("anon. from line %u", (uint) this->nrs->lne);
    else
        this->nrs->nam->print();
}

FunObj::~FunObj()
{
    if (!this->is_nat)
        this->as.usr.~UsrSrt();
}

void FunObj::set(UsrSrt up)
{
    this->is_nat = false;
    // destructive set
    new (&this->as.usr) UsrSrt(std::move(up));
}

void FunObj::set(NatFun &&f)
{
    this->is_nat = true;
    new (&this->as.nat) NatFun(std::move(f));
}

void FunObj::print() const
{
    if (this->is_nat)
        this->as.nat.print();
    else
        this->as.usr.print();
}

ProObj::~ProObj()
{
    if (!this->is_nat)
        this->as.usr.~UsrSrt();
}

void ProObj::set(UsrSrt up)
{
    this->is_nat = false;
    // destructive set
    new (&this->as.usr) UsrSrt(std::move(up));
}

void ProObj::set(NatPro &&p)
{
    this->is_nat = true;
    new (&this->as.nat) NatPro(std::move(p));
}

void ProObj::print() const
{
    if (this->is_nat)
        this->as.nat.print();
    else
        this->as.usr.print();
}
