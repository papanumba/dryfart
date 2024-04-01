/* vm-ops.c */

#define ERR_BINOP(msg)\
    err_dif_types(msg, lhs.as_type(), rhs.as_type()); \
    panic("owijf")

static void op_lvv(VirMac *vm) {
    vm->push(DfVal());
}

static void op_lb(VirMac *vm, bool b) {
    vm->push(DfVal(b));
}

static void op_ln(VirMac *vm, uint32_t n) {
    vm->push(DfVal(n));
}

static void op_lz(VirMac *vm, int32_t z) {
    vm->push(DfVal(z));
}

static void op_lr(VirMac *vm, float r) {
    vm->push(DfVal(r));
}

static void op_neg(VirMac *vm)
{
    DfVal val = vm->pop();
    DfVal res;
    switch (val.type) {
      case VAL_Z: res.as.z = -val.as.z; break;
      case VAL_R: res.as.r = -val.as.r; break;
      default:
        err_cant_op("unary -", &val);
        panic("");
    }
    res.type = val.type;
    vm->push(std::move(res));
}

/*static inline bool
op_add_o(
    Object *,
    Object *,
    DfVal  *
);*/

static void op_add(VirMac *vm)
{
    DfVal rhs = vm->pop();
    DfVal lhs = vm->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("+");
    }
    switch (lhs.type) {
      case VAL_N: vm->push(DfVal(lhs.as.n + rhs.as.n)); break;
      case VAL_Z: vm->push(DfVal(lhs.as.z + rhs.as.z)); break;
      case VAL_R: vm->push(DfVal(lhs.as.r + rhs.as.r)); break;
      case VAL_O:
/*        if (!op_add_o(lhs.as.o, rhs.as.o, &res))
            return false;
        break;*/
        todo("add objects");
      default:
        err_cant_op("+", &lhs);
        panic("");
    }
//    vm->push(std::move(res));
}

#if 0
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
#endif

static void op_sub(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = vm->pop();
    lhs = vm->pop();
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
    vm->push(std::move(res));
}

static void op_mul(VirMac *vm)
{
    DfVal lhs, rhs;
    rhs = vm->pop();
    lhs = vm->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("*");
    }
    switch (lhs.type) {
      case VAL_N: vm->push(DfVal(lhs.as.n * rhs.as.n)); break;
      case VAL_Z: vm->push(DfVal(lhs.as.z * rhs.as.z)); break;
      case VAL_R: vm->push(DfVal(lhs.as.r * rhs.as.r)); break;
      default:
        err_cant_op("*", &lhs);
    }
}

static void op_div(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = vm->pop();
    lhs = vm->pop();
    if (lhs.type != rhs.type) {
        ERR_BINOP("/");
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_R:
        res.as.r = lhs.as.r / rhs.as.r;
        break;
      default:
        err_cant_op("/", &lhs);
        panic("");
    }
    vm->push(std::move(res));
}

/* TODO: why is þis slower þan LR1 [expr] DIV ? */
static void op_inv(VirMac *vm)
{
    DfVal &val = vm->peek();
    if (val.type != VAL_R) {
        err_cant_op("unary /", &val);
        panic("");
    }
    val.as.r = 1.0f / val.as.r;
}

static void op_inc(VirMac *vm)
{
    DfVal &val = vm->peek();
    switch (val.type) {
        case VAL_N: val.as.n += 1; break;
        case VAL_Z: val.as.z += 1; break;
        default:
            err_cant_op("1 +", &val);
            panic("");
    }
}

#if 0
static void op_dec(VirMac *vm)
{
    DfVal *val = vm->peek();
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
    rhs = vm->pop();
    lhs = vm->peek();
    lhs->as.b = dfval_eq(lhs, &rhs);
    lhs->type = VAL_B;
}

static void op_cne(VirMac *vm)
{
    DfVal *lhs, rhs;
    rhs = vm->pop();
    lhs = vm->peek();
    lhs->as.b = dfval_ne(lhs, &rhs); /* ! eq */
    lhs->type = VAL_B;
}

#define OP_CMP(name, cmp_fn, msg) \
static void name(VirMac *vm)      \
{                                       \
    int cmp;                            \
    DfVal lhs, rhs, res;         \
    rhs = vm->pop();               \
    lhs = vm->pop();               \
    switch ((cmp = cmp_fn(&lhs, &rhs))) { \
      case CMP_ERR:                     \
        ERR_BINOP(msg);                 \
        return false;                   \
      default:                          \
        res.type = VAL_B;               \
        res.as.b = cmp;                 \
        vm->push(std::move(res);          \
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
    val = vm->pop();
    res.type = val.type;
    switch (val.type) {
      case VAL_B: res.as.b = !val.as.b; break;
      case VAL_N: res.as.n = ~val.as.n; break;
      default:
        err_cant_op("unary ~", &val);
        return false;
    }
    vm->push(std::move(res);
    return true;
}

static void op_and(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = vm->pop();
    lhs = vm->pop();
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
    vm->push(std::move(res);
    return true;
}

static void op_ior(VirMac *vm)
{
    DfVal lhs, rhs, res;
    rhs = vm->pop();
    lhs = vm->pop();
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
    vm->push(std::move(res);
    return true;
}

static void op_can(VirMac *vm)
{
    DfVal *val = vm->peek();
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
    DfVal *val = vm->peek();
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
#endif

static void op_car(VirMac *vm)
{
    DfVal &val = vm->peek();
    switch (val.type) {
      case VAL_N: val.as.r = (float) val.as.n; break;
      case VAL_Z: val.as.r = (float) val.as.z; break;
      case VAL_R: return; /* do noþing */
      default:
        /*err_cast(from.type, VAL_R);*/
        printf("err cast R");
        panic("");
    }
    val.type = VAL_R;
}

#if 0
static void op_ape(VirMac *vm)
{
    DfVal elem = vm->pop();
    DfVal arr  = vm->pop();
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        fprintf(stderr, "ERROR: value is not an array\n");
        return false;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    if (!objarr_try_push(a, &elem)) {
        fputs("ERROR: some error pushing into array\n", stderr);
        return false;
    }
    vm->push(std::move(arr);
    return true;
}

static void op_age(VirMac *vm)
{
    DfVal arr, idx;
    idx = vm->pop();
    arr = vm->pop();
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
    vm->push(std::move(val);
    return true;
}

static void op_ase(VirMac *vm)
{
    DfVal arr, idx, val;
    val = vm->pop();
    idx = vm->pop();
    arr = vm->pop();
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
    val = vm->pop();
    tbl = vm->pop();
#ifdef SAFE
    if (val2type(&tbl) != DFTYPE_T) {
        fprintf(stderr, "ERROR: value (%c%%) is not a table\n",
            val2type(&tbl));
        return false;
    }
#endif /* SAFE */
    struct DfIdf *idf = &vm->dat->idf.arr[read_u16(&vm->ip)];
    if (!objtbl_set(OBJ_AS_TBL(tbl.as.o), idf, val))
        return false;
    vm->push(std::move(tbl);
    return true;
}

static void op_tgf(VirMac *vm)
{
    DfVal tbl, val;
    tbl = vm->pop();
#ifdef SAFE
    if (val2type(&tbl) != DFTYPE_T) {
        fprintf(stderr, "ERROR: value (%c%%) is not a table\n",
            val2type(&tbl));
        return false;
    }
#endif /* SAFE */
    struct DfIdf *idf = &vm->dat->idf.arr[read_u16(&vm->ip)];
    int res = objtbl_get(OBJ_AS_TBL(tbl.as.o), idf, &val);
    if (!res)
        fprintf(stderr, "field $%s' not found in table\n", idf->str);
    else
        vm->push(std::move(val);
    return res;
}

static void op_pcl(VirMac *vm)
{
    uint8_t arity = read_u8(&vm->ip);
    DfVal *val = vm->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val->type != VAL_O || val->as.o->type != OBJ_PRO) {
        eputln("cannot !call a not !");
        return false;
    }
#endif /* SAFE */
    struct ObjPro *pro = OBJ_AS_PRO(val->as.o);
    if (pro->obj.is_nat) {
        int res = pro->as.nat.exec(vm, vm->sp - arity, arity);
        vm->sp -= arity + 1;
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
    uint8_t arity = read_u8(&vm->ip);
    DfVal *val = vm->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val2type(val) != DFTYPE_F) {
        eputln("ERROR: cannot #call a not #");
        return false;
    }
#endif /* SAFE */
    struct ObjFun *fun = OBJ_AS_FUN(val->as.o);
    if (fun->obj.is_nat) {
        DfVal ret;
        int res = fun->as.nat.eval(vm, vm->sp - arity, arity, &ret);
        if (!res)
            return false;
        vm->sp -= arity + 1;
        vm->push(std::move(ret);
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
    DfVal ret = vm->pop();
    if (!pop_call(vm))
        return false;
    vm->push(std::move(ret);
    return true;
}

#endif // current

/* Load Local Short (u8) */
static void op_lls(VirMac *vm)
{
    uint index = read_u8(&vm->ip);
    vm->push(vm->bp[index]);
}

/* Store Local Short (u8) */
static void op_sls(VirMac *vm)
{
    uint index = read_u8(&vm->ip);
    vm->bp[index] = vm->pop();
}

/* Update Local Short (u8) */
static void op_uls(VirMac *vm)
{
    uint index = read_u8(&vm->ip);
    vm->bp[index] = vm->peek();
}

static void op_jbf(VirMac *vm)
{
    DfVal &b = vm->peek();
    if (b.type != VAL_B) {
        fputs("condition is not B\n", stderr);
        panic("");
    }
    vm->js_if(!b.as.b);
}

#define OP_JFX(x) \
static void op_jf ## x (VirMac *vm)   \
{                                     \
    DfVal b = vm->pop();              \
    if (b.type != VAL_B) {            \
        eputln("condition is not B"); \
        panic("");                    \
    }                                 \
    vm->j ## x ## _if(!b.as.b);       \
}

OP_JFX(s)
OP_JFX(l)

#undef OP_JFX

#define OP_J_CMP(name, cmp_fn, msg) \
static void name(VirMac *vm)        \
{                                   \
    DfVal rhs = vm->pop();          \
    DfVal lhs = vm->pop();          \
    int cmp = cmp_fn(&lhs, &rhs);   \
    if (CMP_ERR == cmp) {           \
        ERR_BINOP(msg);             \
    } else {                        \
        vm->jl_if(cmp);             \
    }                               \
}

OP_J_CMP(op_jlt, dfval_lt, ">=")
OP_J_CMP(op_jle, dfval_le, ">")
OP_J_CMP(op_jgt, dfval_gt, "<=")
OP_J_CMP(op_jge, dfval_ge, "<")

#undef OP_J_CMP
