/* vm-ops.c */

#include <cassert>
#include "values.h"
#include "virmac.h"

#define SIMPLE_ERR(msg) do { \
    eputln(msg);            \
    return ITP_RUNTIME_ERR; \
} while (false)

#define ERR_BINOP(msg) do { \
    err_dif_types(msg, lhs.as_type(), rhs.as_type()); \
    return ITP_RUNTIME_ERR; \
} while (false)

#define ERR_OP_TYPE(msg, valref) do { \
    err_cant_op(msg, valref); \
    return ITP_RUNTIME_ERR;   \
} while (false)

#define READ_U8()   read_u8 (&this->ip)
#define READ_U16()  read_u16(&this->ip)

/*****************************************************************/

case OP_NOP:
    break;

// load ---------------------

case OP_LVV:
    this->push(DfVal());
    break;

#define OP_LXX(xx, val) \
case OP_L ## xx: this->push(DfVal((val))); break;
OP_LXX(BT, true)
OP_LXX(BF, false)

#define OP_LNX(n) OP_LXX(N ## n, (uint32_t)(n))
OP_LNX(0)
OP_LNX(1)
OP_LNX(2)
OP_LNX(3)
#undef OP_LNX

OP_LXX(M1, (int32_t)(-1))

#define OP_LZX(z) OP_LXX(Z ## z, (int32_t)(z))
OP_LZX(0)
OP_LZX(1)
OP_LZX(2)
#undef OP_LZX

#define OP_LRX(r) OP_LXX(R ## r, (float)(r))
OP_LRX(0)
OP_LRX(1)
#undef OP_LRX

case OP_LKS: {
    uint idx = READ_U8();
    this->push(DfVal(this->dat->ctn[idx]));
    break;
}

case OP_LKL: {
    uint idx = READ_U16();
    this->push(this->dat->ctn[idx]);
    break;
}

// ariþmetic ------------------

case OP_NEG: {
    DfVal val = this->pop(); // fastest (checked with peek)
    switch (val.type) {
      case VAL_Z: this->fpush(DfVal(-val.as.z)); break;
      case VAL_R: this->fpush(DfVal(-val.as.r)); break;
      default: ERR_OP_TYPE("unary -", &val);
    }
    break;
}

// only used in OP_ADD
#define ADD_O do { \
    auto lo = lhs.as.o;                     \
    auto ro = rhs.as.o;                     \
    if (lo.get_type() != ro.get_type())     \
        ERR_BINOP("+");                     \
    switch (lo.get_type()) {                \
      case OBJ_ARR: {                       \
        auto a = this->ma->alloc(OBJ_ARR);  \
        a.as_arr()->typ = DfType::V;        \
        a.as_arr()->is_nat = false;         \
        auto r = lo.as_arr()->concat(       \
            *ro.as_arr(), *a.as_arr());     \
        if (AccRes::OK != r)                \
            SIMPLE_ERR(accres_what(r));          \
        this->push(DfVal(a));               \
        break;                              \
      }                                     \
      default: todo("add other objects");   \
    } \
} while (false)

case OP_ADD: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("+");
    switch (lhs.type) {
      case VAL_N: this->push(DfVal(lhs.as.n + rhs.as.n)); break;
      case VAL_Z: this->push(DfVal(lhs.as.z + rhs.as.z)); break;
      case VAL_R: this->push(DfVal(lhs.as.r + rhs.as.r)); break;
      case VAL_O: ADD_O; break;
      default: ERR_OP_TYPE("+", &lhs);
    }
    break;
}

#undef ADD_O

case OP_SUB: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("-");
    switch (lhs.type) {
      case VAL_Z: this->push(DfVal(lhs.as.z - rhs.as.z)); break;
      case VAL_R: this->push(DfVal(lhs.as.r - rhs.as.r)); break;
      default: ERR_OP_TYPE("-", &lhs);
    }
    break;
}

case OP_MUL: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("*");
    switch (lhs.type) {
      case VAL_N: this->push(DfVal(lhs.as.n * rhs.as.n)); break;
      case VAL_Z: this->push(DfVal(lhs.as.z * rhs.as.z)); break;
      case VAL_R: this->push(DfVal(lhs.as.r * rhs.as.r)); break;
      default: ERR_OP_TYPE("*", &lhs);
    }
    break;
}

case OP_DIV: { // fastest: checked with {pop, peek, ->} & {2pop, fpush}
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("/");
    }
    switch (lhs.type) {
//      case VAL_N: this->push(DfVal(lhs.as.n * rhs.as.n)); break;
//      case VAL_Z: this->push(DfVal(lhs.as.z * rhs.as.z)); break;
      case VAL_R: this->push(DfVal(lhs.as.r / rhs.as.r)); break;
      default: ERR_OP_TYPE("/", &lhs);
    }
    break;
}

case OP_INV: {
/* TODO: why is þis slower þan LR1 [expr] DIV ? */
    DfVal &val = this->peek();
    if (val.type != VAL_R)
        ERR_OP_TYPE("unary /", &val);
    val.as.r = 1.0f / val.as.r;
    break;
}

case OP_INC: {
    DfVal &val = this->peek();
    switch (val.type) {
        case VAL_N: val.as.n += 1; break;
        case VAL_Z: val.as.z += 1; break;
        default: ERR_OP_TYPE("1 +", &val);
    }
    break;
}

case OP_DEC: {
    DfVal &val = this->peek();
    if (val.type != VAL_Z)
        ERR_OP_TYPE("- 1", &val);
    val.as.z -= 1;
    break;
}

// compare ----------------------

#define OP_CEX(XX, op) \
case OP_C ## XX: {                 \
    DfVal rhs = this->pop();       \
    DfVal lhs = this->pop();       \
    this->push(DfVal(lhs op rhs)); \
    break;                         \
}

// overloaded operators
OP_CEX(EQ, ==)
OP_CEX(NE, !=)

#undef OP_CEX

// locals -----------------------

case OP_LLS: {
    uint index = READ_U8();
    this->push(this->bp[index]);
    break;
}

case OP_SLS: {
    uint index = READ_U8();
    this->bp[index] = this->pop();
    break;
}

case OP_ULS: {
    uint index = READ_U8();
    this->bp[index] = this->peek(); // fastest, checked with sp[-1]
    break;
}

// jumps -----------------------------

#define CHECK_IS_B(var) do { \
    if ((var).type != VAL_B) \
        SIMPLE_ERR("condition is not B%"); \
} while (false)

case OP_JJS: {
    int dist = read_i8(&this->ip);
    this->ip += dist;
    break;
}

case OP_JJL: {
    int dist = read_i16(&this->ip);
    this->ip += dist;
    break;
}

case OP_JBF: {
    DfVal &b = this->peek();
    CHECK_IS_B(b);
    this->js_if(!b.as.b);
    break;
}

#define OP_JFX(X, x) \
case OP_JF ## X: {                \
    DfVal b = this->pop();        \
    CHECK_IS_B(b);                \
    this->j ## x ## _if(!b.as.b); \
    break;                        \
}

OP_JFX(S, s)
OP_JFX(L, l)

#undef OP_JFX

#define OP_J_CMP(CMP, cmp, msg) \
case OP_J ## CMP: {          \
    DfVal rhs = this->pop(); \
    DfVal lhs = this->pop(); \
    int cmp = dfval_ ## cmp(&lhs, &rhs); \
    if (CMP_ERR == cmp) {    \
        ERR_BINOP(msg);      \
    } else {                 \
        this->jl_if(cmp);    \
    }                        \
    break;                   \
}

OP_J_CMP(LT, lt, ">=")
OP_J_CMP(LE, le, ">")
OP_J_CMP(GT, gt, "<=")
OP_J_CMP(GE, ge, "<")

#undef OP_J_CMP

// array ---------------------------

case OP_AMN: {
    auto a = this->ma->alloc(OBJ_ARR);
    a.as_arr()->typ = DfType::V;
    a.as_arr()->is_nat = false;
    this->push(DfVal(a));
    break;
}

case OP_APE: {
    DfVal elem = this->pop();
    DfVal arr  = this->pop();
    if (arr.type != VAL_O || arr.as.o.get_type() != OBJ_ARR)
        SIMPLE_ERR("ERROR: value is not an array");
    ArrObj *a = arr.as.o.as_arr();
    if (AccRes::OK != a->push(std::move(elem)))
        SIMPLE_ERR("ERROR: some error pushing into array");
    this->push(std::move(arr));
    break;
}

case OP_AGE: {
    DfVal idx = this->pop();
    DfVal arr = this->pop();
    if (!arr.is_arr())
        SIMPLE_ERR("ERROR: value is not an array");
    uint32_t idx_n = 0;
    switch (idx.type) {
      case VAL_N: idx_n = idx.as.n; break;
      case VAL_Z:
        if (idx.as.z < 0)
            SIMPLE_ERR("ERROR: Z% index is negative");
        idx_n = (uint32_t) idx.as.z;
        break;
      default: SIMPLE_ERR("ERROR: index is not N% or Z%");
    }
    auto a = arr.as.o.as_arr();
    DfVal val;
    auto res = a->get(idx_n, val);
    if (res != AccRes::OK) {
        printf("len is %u, idx is %u\n", a->len(), idx_n);
        SIMPLE_ERR(accres_what(res));
    }
    this->push(std::move(val));
    break;
}

// table ---------------------------

case OP_TMN: {
    auto t = this->ma->alloc(OBJ_TBL);
    t.as_tbl()->set(Htable());
    this->push(DfVal(t));
    break;
}

#define CHECK_IS_T(x) do { \
    if (!(x).is_tbl()) {            \
        fprintf(stderr, "ERROR: value of type %c%% is not a table\n", \
            (char) (x).as_type());  \
        return ITP_RUNTIME_ERR;     \
    }                               \
} while (false)

#define NOT_FOUND(x) do { \
    eput("field '"); idf->eprint(); eput("' not found in table\n"); \
    return ITP_RUNTIME_ERR; \
} while (false)

case OP_TSF: {
    DfVal val = this->pop();
    DfVal tbl = this->pop();
#ifdef SAFE
    CHECK_IS_T(tbl);
    if (!tbl.as.o.mut()) // must be mut
        SIMPLE_ERR("trying to set field of immutable table");
#endif
    const DfIdf *idf = &this->dat->idf[READ_U16()];
    if (!tbl.as.o.as_tbl()->set(idf, std::move(val)))
        NOT_FOUND(idf);
    this->push(std::move(tbl));
    break;
}

case OP_TGF: {
    DfVal tbl = this->pop();
#ifdef SAFE
    CHECK_IS_T(tbl);
#endif
    const DfIdf *idf = &this->dat->idf[READ_U16()];
    DfVal val;
    if (!tbl.as.o.as_tbl()->get(idf, val))
        NOT_FOUND(idf);
#ifdef SAFE
    if (!tbl.as.o.mut()) // propagate mutability
        val.set_mut(false);
#endif
    this->push(std::move(val));
    break;
}

#undef CHECK_IS_T

// subrts ---------------------------

case OP_FMN: {
    auto f = this->ma->alloc(OBJ_FUN);
    uint nor_idx = READ_U16();
    auto nor = &this->nor[nor_idx];
    auto uvs = nor->uvs;
    auto uvp = this->sp - uvs; // base uv pointer
#ifdef SAFE
    TIL(i, uvs) {
        if (uvp[i].type == VAL_O)
            uvp[i].as.o.set_mut(false);
    }
#endif
    f.as_fun()->set(UsrSrt(nor, uvp));
    TIL(i, uvs) this->pop();
    this->push(DfVal(f));
    break;
}

case OP_FCL: {
    uint8_t arity = READ_U8();
    DfVal *cle = this->sp - (arity + 1); // args + callee
#ifdef SAFE
    if (!cle->is_fun())
        SIMPLE_ERR("cannot !call a not !");
#endif
    auto *fun = cle->as.o.as_fun();
    if (fun->is_nat) {
        /*int res = fun->as.nat.exec(vm, this->sp - arity, arity);
        this->sp -= arity + 1;*/
        todo("call nat fun");
    }
#ifdef SAFE
    // user funs
    if (fun->as.usr.nrs->ari != arity) {
        printf("ERROR: wrong arity calling ");
        fun->print();
        puts("");
        return ITP_RUNTIME_ERR;
    }
    // set immutable arguments
    TIL(i, arity)
        cle[1+arity].set_mut(false);
#endif // SAFE
    this->push_call(cle, fun->as.usr.nrs);
    break;
}

case OP_PMN: {
    auto p = this->ma->alloc(OBJ_PRO);
    uint nor_idx = READ_U16();
    auto nor = &this->nor[nor_idx];
    auto uvs = nor->uvs;
    p.as_pro()->set(UsrSrt(nor, this->sp - uvs));
    TIL(i, uvs) this->pop();
    this->push(DfVal(p));
    break;
}

case OP_PCL: {
    uint8_t arity = READ_U8();
    DfVal *cle = this->sp - (arity + 1); // args + callee
#ifdef SAFE
    if (!cle->is_pro()) {
        eputln("cannot !call a not !");
        return ITP_RUNTIME_ERR;
    }
#endif // SAFE
    auto *pro = cle->as.o.as_pro();
    if (pro->is_nat) {
        /*int res = pro->as.nat.exec(vm, this->sp - arity, arity);
        this->sp -= arity + 1;*/
        todo("call nat pro");
    }
#ifdef SAFE
    // user procs
    if (pro->as.usr.nrs->ari != arity) {
        printf("ERROR: wrong arity calling ");
        pro->print();
        puts("");
        return ITP_RUNTIME_ERR;
    }
#endif // SAFE
    this->push_call(cle, pro->as.usr.nrs);
    break;
}

case OP_LUV: {
    assert(this->bp->type == VAL_O);
    auto bpo = this->bp->as.o;
    auto upvidx = READ_U8();
    switch (bpo.get_type()) {
      case OBJ_FUN: {
        auto bpf = bpo.as_fun();
        assert(!bpf->is_nat);
        this->push(DfVal(bpf->as.usr.upv[upvidx]));
        break;
      }
      case OBJ_PRO: {
        auto bpp = bpo.as_pro();
        assert(!bpp->is_nat);
        this->push(DfVal(bpp->as.usr.upv[upvidx]));
        break;
      }
      default:
        unreachable();
    }
    break;
}

// casts ---------------------------

case OP_CAR: {
    DfVal &val = this->peek();
    switch (val.type) {
      case VAL_N: val.as.r = (float) val.as.n; break;
      case VAL_Z: val.as.r = (float) val.as.z; break;
      case VAL_R: break; /* do noþing */
      default:
        /*err_cast(from.type, VAL_R);*/
        eputln("err cast R%");
        return ITP_RUNTIME_ERR;
    }
    val.type = VAL_R;
    break;
}

// stack stuff -----------------------

case OP_END:
    this->pop_call();
    break;

case OP_RET: {
    DfVal ret = this->pop();
    this->pop_call();
    this->push(std::move(ret));
    break;
}

case OP_DUP:
    this->push(DfVal(this->peek()));
    break;

case OP_POP:
    (void) this->pop();
    break;

case OP_HLT:
#ifdef DEBUG
    puts("VM HALTED");
    //this->print_calls();
#endif
    if (this->callnum != 0) { // not halted at main
        this->print_stack();
        this->reset_stack();
        return ITP_RUNTIME_ERR;
    }
    this->reset_stack();
//            garcol_do(vm);
    return ITP_OK;

#if 0 // -----------------------------------------------------------

/*static inline bool
op_add_o(
    Object *,
    Object *,
    DfVal  *
);*/

/* helper for object case */
static inline int op_add_o(
    struct Object *lhs,
    struct Object *rhs,
    DfVal  *res)
{
    if (lhs->type != rhs->type) {
        eputln("objects of different type in + expr");
        return false;
    }
    switch (lhs->type) {
      case OBJ_ARR: {
        struct ObjArr *arr = NULL;
        arr = objarr_concat(OBJ_AS_ARR(lhs), OBJ_AS_ARR(rhs));
        if (arr == NULL)
            return false;
        res->type = VAL_O;
        res->as.o = (void *) arr;
        break;
      }
      case OBJ_TBL:
        SIMPLE_ERR("todo $%% + $%%");
        break;
      case OBJ_PRO:
        eputln("cannot add (+) procs");
        return false;
      case OBJ_FUN:
        eputln("cannot add (+) funcs");
        return false;
    }
    return true;
}

static void op_cne(VirMac *vm)
{
    DfVal *lhs, rhs;
    rhs = this->pop();
    lhs = this->peek();
    lhs->as.b = dfval_ne(lhs, &rhs); /* ! eq */
    lhs->type = VAL_B;
}

#define OP_CMP(name, cmp_fn, msg) \
static void name(VirMac *vm)      \
{                                       \
    int cmp;                            \
    DfVal lhs, rhs, res;         \
    rhs = this->pop();               \
    lhs = this->pop();               \
    switch ((cmp = cmp_fn(&lhs, &rhs))) { \
      case CMP_ERR:                     \
        ERR_BINOP(msg);                 \
        return false;                   \
      default:                          \
        res.type = VAL_B;               \
        res.as.b = cmp;                 \
        this->push(std::move(res);          \
        return true;                    \
    }                                   \
}

OP_CMP(op_clt, dfval_lt, "<")
OP_CMP(op_cle, dfval_le, "<=")
OP_CMP(op_cgt, dfval_gt, ">")
OP_CMP(op_cge, dfval_ge, ">=")

#undef OP_CMP

static void op_not(VirMac *vm)
{
    DfVal val, res;
    val = this->pop();
    res.type = val.type;
    switch (val.type) {
      case VAL_B: res.as.b = !val.as.b; break;
      case VAL_N: res.as.n = ~val.as.n; break;
      default:
        err_cant_op("unary ~", &val);
        return false;
    }
    this->push(std::move(res);
    return true;
}

static void op_and(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = this->pop();
    lhs = this->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("&");
        return false;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b && rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n &  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("&", &lhs);
        return false;
    }
    this->push(std::move(res);
    return true;
}

static void op_ior(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = this->pop();
    lhs = this->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("|");
        return false;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b || rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n |  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("|", &lhs);
        return false;
    }
    this->push(std::move(res);
    return true;
}

static void op_can(VirMac *vm)
{
    DfVal *val = this->peek();
    switch (val->type) {
      case VAL_N: return true; /* do noþing */
      case VAL_Z:
        if (val->as.z < 0) {
            fputs("ERROR: casting negative Z% to N%\n", stderr);
            return false;
        }
        val->as.n = (uint32_t) val->as.z; break;
      default:
        /*err_cast(from.type, VAL_R);*/
        eputln("err cast N");
        return false;
    }
    val->type = VAL_N;
    return true;
}

static void op_caz(VirMac *vm)
{
    DfVal *val = this->peek();
    switch (val->type) {
      case VAL_N:
#ifdef SAFE
        if (val->as.n > (uint32_t) INT32_MAX) {
            fputs("ERROR: Overflow casting to Z\n", stderr);
            return false;
        }
#endif /* SAFE */
        val->as.z = (int32_t) val->as.n; break;
      case VAL_Z: return true; /* do noþing */
      default:
        /*err_cast(from.type, VAL_R);*/
        printf("err cast");
        return false;
    }
    val->type = VAL_Z;
    return true;
}

static void op_ase(VirMac *vm)
{
    DfVal arr, idx, val;
    val = this->pop();
    idx = this->pop();
    arr = this->pop();
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        eputln("ERROR: value is not an array");
        return false;
    }
    if (idx.type != VAL_N) {
        eputln("ERROR: index is not N%");
        return false;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    return objarr_set(a, idx.as.n, val); /* OK or ERR result */
}

static void op_pcl(VirMac *vm)
{
}

static void op_fcl(VirMac *vm)
{
    uint8_t arity = READ_U8();
    DfVal *val = this->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val2type(val) != DFTYPE_F) {
        eputln("ERROR: cannot #call a not #");
        return false;
    }
#endif /* SAFE */
    struct ObjFun *fun = OBJ_AS_FUN(val->as.o);
    if (fun->obj.is_nat) {
        DfVal ret;
        int res = fun->as.nat.eval(vm, this->sp - arity, arity, &ret);
        if (!res)
            return false;
        this->sp -= arity + 1;
        this->push(std::move(ret);
        return res;
    }
#ifdef SAFE
    if (fun->as.usr->ari != arity) {
        printf("wrong arity calling ");
        object_print(val->as.o);
        puts("");
        return false;
    }
#endif /* SAFE */
    return push_call(vm, val, fun->as.usr);
}

static void op_ret(VirMac *vm)
{
    DfVal ret = this->pop();
    if (!pop_call(vm))
        return false;
    this->push(std::move(ret);
    return true;
}

#endif // biggest 0

