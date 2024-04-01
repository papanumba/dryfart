/* vm-ops.c */

#include "values.h"
#include "virmac.h"

#define ERR_BINOP(msg) do { \
    err_dif_types(msg, lhs.as_type(), rhs.as_type()); \
    return ITP_RUNTIME_ERR; \
} while (false)

#define ERR_OP_TYPE(msg, valref) do { \
    err_cant_op(msg, valref); \
    return ITP_RUNTIME_ERR; \
} while (false)

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

OP_LXX(R0, 0.0f)
OP_LXX(R1, 1.0f)

case OP_LKS: {
    uint idx = read_u8(&this->ip);
    this->push(DfVal(this->dat->ctn[idx]));
    break;
}

case OP_LKL: {
    uint idx = read_u16(&this->ip);
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

case OP_ADD: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("+");
    switch (lhs.type) {
      case VAL_N: this->push(DfVal(lhs.as.n + rhs.as.n)); break;
      case VAL_Z: this->push(DfVal(lhs.as.z + rhs.as.z)); break;
      case VAL_R: this->push(DfVal(lhs.as.r + rhs.as.r)); break;
      case VAL_O:
/*        if (!op_add_o(lhs.as.o, rhs.as.o, &res))
            return false;
        break;*/
        todo("add objects");
      default: ERR_OP_TYPE("+", &lhs);
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

case OP_INC: {
    DfVal &val = this->peek();
    switch (val.type) {
        case VAL_N: val.as.n += 1; break;
        case VAL_Z: val.as.z += 1; break;
        default: ERR_OP_TYPE("1 +", &val);
    }
    break;
}

// locals -----------------------

case OP_LLS: {
    uint index = read_u8(&this->ip);
    this->push(this->bp[index]);
    break;
}

case OP_SLS: {
    uint index = read_u8(&this->ip);
    this->bp[index] = this->pop();
    break;
}

case OP_ULS: {
    uint index = read_u8(&this->ip);
    this->bp[index] = this->peek(); // fastest, checked with sp[-1]
    break;
}

// jumps -----------------------------

#define CHECK_IS_B(var) do { \
    if ((var).type != VAL_B) {          \
        eputln("condition is not B%");  \
        return ITP_RUNTIME_ERR;         \
    }                                   \
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

case OP_DUP:
    this->push(DfVal(this->peek()));
    break;

case OP_POP:
    (void) this->pop();
    break;

case OP_HLT: {
#ifdef DEBUG
    puts("VM HALTED");
    //this->print_calls();
    this->print_stack();
#endif
    this->reset_stack();
//            garcol_do(vm);
    return ITP_OK;
}

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
        panic("todo $%% + $%%");
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

static void op_sub(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = this->pop();
    lhs = this->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("-");
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_Z: res.as.z = lhs.as.z - rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r - rhs.as.r; break;
      default:
        err_cant_op("-", &lhs);
        panic("");
    }
    this->push(std::move(res));
}

/* TODO: why is þis slower þan LR1 [expr] DIV ? */
static void op_inv(VirMac *vm)
{
    DfVal &val = this->peek();
    if (val.type != VAL_R) {
        err_cant_op("unary /", &val);
        panic("");
    }
    val.as.r = 1.0f / val.as.r;
}

#if 0
static void op_dec(VirMac *vm)
{
    DfVal *val = this->peek();
    switch (val->type) {
        case VAL_Z: val->as.z -= 1; break;
        default:
            err_cant_op("1 +", val);
            return false;
    }
    return true;
}

static void op_ceq(VirMac *vm)
{
    DfVal *lhs, rhs;
    rhs = this->pop();
    lhs = this->peek();
    lhs->as.b = dfval_eq(lhs, &rhs);
    lhs->type = VAL_B;
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

static void op_ape(VirMac *vm)
{
    DfVal elem = this->pop();
    DfVal arr  = this->pop();
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        fprintf(stderr, "ERROR: value is not an array\n");
        return false;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    if (!objarr_try_push(a, &elem)) {
        fputs("ERROR: some error pushing into array\n", stderr);
        return false;
    }
    this->push(std::move(arr);
    return true;
}

static void op_age(VirMac *vm)
{
    DfVal arr, idx;
    idx = this->pop();
    arr = this->pop();
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        eputln("ERROR: value is not an array");
        return false;
    }
    uint32_t idx_n = 0;
    switch (idx.type) {
      case VAL_N: idx_n = idx.as.n; break;
      case VAL_Z:
        if (idx.as.z < 0) {
            eputln("ERROR: Z% index is negative");
            return false;
        }
        idx_n = (uint32_t) idx.as.z;
        break;
      default:
        eputln("ERROR: index is not N% or Z%");
        return false;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    DfVal val = objarr_get(a, idx_n);
    if (val.type == VAL_V)
        return false;
    this->push(std::move(val);
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

static void op_tsf(VirMac *vm)
{
    DfVal tbl, val;
    val = this->pop();
    tbl = this->pop();
#ifdef SAFE
    if (val2type(&tbl) != DFTYPE_T) {
        fprintf(stderr, "ERROR: value (%c%%) is not a table\n",
            val2type(&tbl));
        return false;
    }
#endif /* SAFE */
    struct DfIdf *idf = &this->dat->idf.arr[read_u16(&this->ip)];
    if (!objtbl_set(OBJ_AS_TBL(tbl.as.o), idf, val))
        return false;
    this->push(std::move(tbl);
    return true;
}

static void op_tgf(VirMac *vm)
{
    DfVal tbl, val;
    tbl = this->pop();
#ifdef SAFE
    if (val2type(&tbl) != DFTYPE_T) {
        fprintf(stderr, "ERROR: value (%c%%) is not a table\n",
            val2type(&tbl));
        return false;
    }
#endif /* SAFE */
    struct DfIdf *idf = &this->dat->idf.arr[read_u16(&this->ip)];
    int res = objtbl_get(OBJ_AS_TBL(tbl.as.o), idf, &val);
    if (!res)
        fprintf(stderr, "field $%s' not found in table\n", idf->str);
    else
        this->push(std::move(val);
    return res;
}

static void op_pcl(VirMac *vm)
{
    uint8_t arity = read_u8(&this->ip);
    DfVal *val = this->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val->type != VAL_O || val->as.o->type != OBJ_PRO) {
        eputln("cannot !call a not !");
        return false;
    }
#endif /* SAFE */
    struct ObjPro *pro = OBJ_AS_PRO(val->as.o);
    if (pro->obj.is_nat) {
        int res = pro->as.nat.exec(vm, this->sp - arity, arity);
        this->sp -= arity + 1;
        return res;
    }
#ifdef SAFE
    if (pro->as.usr->ari != arity) {
        printf("wrong arity calling ");
        object_print(val->as.o);
        puts("");
        return false;
    }
#endif /* SAFE */
    return push_call(vm, val, pro->as.usr);
}

static void op_fcl(VirMac *vm)
{
    uint8_t arity = read_u8(&this->ip);
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

#endif // current



#endif // biggest 0
