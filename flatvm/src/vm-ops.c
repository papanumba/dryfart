/* vm-ops.c */

#define ERR_BINOP(msg)  err_dif_types(msg, val2type(&lhs), val2type(&rhs))

static void op_lvv(struct VirMac *vm)
{
    struct DfVal v;
    v.type = VAL_V;
    virmac_push(vm, &v);
}

static void op_lb(struct VirMac *vm, int b)
{
    struct DfVal vb;
    vb.type = VAL_B;
    vb.as.b = b;
    virmac_push(vm, &vb);
}

static void op_ln(struct VirMac *vm, uint32_t n)
{
    struct DfVal vn;
    vn.type = VAL_N;
    vn.as.n = n;
    virmac_push(vm, &vn);
}

static void op_lz(struct VirMac *vm, int32_t z)
{
    struct DfVal vz;
    vz.type = VAL_Z;
    vz.as.z = z;
    virmac_push(vm, &vz);
}

static void op_lr(struct VirMac *vm, float r)
{
    struct DfVal vr;
    vr.type = VAL_R;
    vr.as.r = r;
    virmac_push(vm, &vr);
}

static int op_neg(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = val.type;
    switch (val.type) {
      case VAL_Z: res.as.z = -val.as.z; break;
      case VAL_R: res.as.r = -val.as.r; break;
      default:
        err_cant_op("unary -", &val);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static inline int
op_add_o(
    struct Object *,
    struct Object *,
    struct DfVal  *
);

static int op_add(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("+");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N: res.as.n = lhs.as.n + rhs.as.n; break;
      case VAL_Z: res.as.z = lhs.as.z + rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r + rhs.as.r; break;
      case VAL_O:
        if (!op_add_o(lhs.as.o, rhs.as.o, &res))
            return FALSE;
        break;
      default:
        err_cant_op("+", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

/* helper for object case */
static inline int op_add_o(
    struct Object *lhs,
    struct Object *rhs,
    struct DfVal  *res)
{
    if (lhs->type != rhs->type) {
        eputln("objects of different type in + expr");
        return FALSE;
    }
    switch (lhs->type) {
      case OBJ_ARR: {
        struct ObjArr *arr = NULL;
        arr = objarr_concat(OBJ_AS_ARR(lhs), OBJ_AS_ARR(rhs));
        if (arr == NULL)
            return FALSE;
        res->type = VAL_O;
        res->as.o = (void *) arr;
        break;
      }
      case OBJ_TBL:
        panic("todo $%% + $%%");
        break;
      case OBJ_PRO:
        eputln("cannot add (+) procs");
        return FALSE;
      case OBJ_FUN:
        eputln("cannot add (+) funcs");
        return FALSE;
    }
    return TRUE;
}

static int op_sub(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("-");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_Z: res.as.z = lhs.as.z - rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r - rhs.as.r; break;
      default:
        err_cant_op("-", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_mul(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("*");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N: res.as.n = lhs.as.n * rhs.as.n; break;
      case VAL_Z: res.as.z = lhs.as.z * rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r * rhs.as.r; break;
      default:
        err_cant_op("*", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_div(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("/");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_R:
#ifdef SAFE
        if (rhs.as.r == 0.0f) {
            eputln("ERROR: Division by 0.0");
            return FALSE;
        }
#endif /* SAFE */
        res.as.r = lhs.as.r / rhs.as.r;
        break;
      default:
        err_cant_op("/", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

/* TODO: why is þis slower þan LR1 [expr] DIV ? */
static int op_inv(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    if (val->type != VAL_R) {
        err_cant_op("unary /", val);
        return FALSE;
    }
    val->as.r = 1.0f / val->as.r;
    return TRUE;
}

static int op_inc(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
        case VAL_N: val->as.n += 1; break;
        case VAL_Z: val->as.z += 1; break;
        default:
            err_cant_op("1 +", val);
            return FALSE;
    }
    return TRUE;
}

static int op_dec(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
        case VAL_Z: val->as.z -= 1; break;
        default:
            err_cant_op("1 +", val);
            return FALSE;
    }
    return TRUE;
}

static void op_ceq(struct VirMac *vm)
{
    struct DfVal *lhs, rhs;
    rhs = virmac_pop(vm);
    lhs = virmac_peek(vm);
    lhs->as.b = dfval_eq(lhs, &rhs);
    lhs->type = VAL_B;
}

static void op_cne(struct VirMac *vm)
{
    struct DfVal *lhs, rhs;
    rhs = virmac_pop(vm);
    lhs = virmac_peek(vm);
    lhs->as.b = dfval_ne(lhs, &rhs); /* ! eq */
    lhs->type = VAL_B;
}

#define OP_CMP(name, cmp_fn, msg) \
static int name(struct VirMac *vm)      \
{                                       \
    int cmp;                            \
    struct DfVal lhs, rhs, res;         \
    rhs = virmac_pop(vm);               \
    lhs = virmac_pop(vm);               \
    switch ((cmp = cmp_fn(&lhs, &rhs))) { \
      case CMP_ERR:                     \
        ERR_BINOP(msg);                 \
        return FALSE;                   \
      default:                          \
        res.type = VAL_B;               \
        res.as.b = cmp;                 \
        virmac_push(vm, &res);          \
        return TRUE;                    \
    }                                   \
}

OP_CMP(op_clt, dfval_lt, "<")
OP_CMP(op_cle, dfval_le, "<=")
OP_CMP(op_cgt, dfval_gt, ">")
OP_CMP(op_cge, dfval_ge, ">=")

#undef OP_CMP

static int op_not(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = val.type;
    switch (val.type) {
      case VAL_B: res.as.b = !val.as.b; break;
      case VAL_N: res.as.n = ~val.as.n; break;
      default:
        err_cant_op("unary ~", &val);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_and(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("&");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b && rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n &  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("&", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_ior(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        ERR_BINOP("|");
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b || rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n |  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("|", &lhs);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_can(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
      case VAL_N: return TRUE; /* do noþing */
      case VAL_Z:
        if (val->as.z < 0) {
            fputs("ERROR: casting negative Z% to N%\n", stderr);
            return FALSE;
        }
        val->as.n = (uint32_t) val->as.z; break;
      default:
        /*err_cast(from.type, VAL_R);*/
        eputln("err cast N");
        return FALSE;
    }
    val->type = VAL_N;
    return TRUE;
}

static int op_caz(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
      case VAL_N:
#ifdef SAFE
        if (val->as.n > (uint32_t) INT32_MAX) {
            fputs("ERROR: Overflow casting to Z\n", stderr);
            return FALSE;
        }
#endif /* SAFE */
        val->as.z = (int32_t) val->as.n; break;
      case VAL_Z: return TRUE; /* do noþing */
      default:
        /*err_cast(from.type, VAL_R);*/
        printf("err cast");
        return FALSE;
    }
    val->type = VAL_Z;
    return TRUE;
}

static int op_car(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
      case VAL_N: val->as.r = (float) val->as.n; break;
      case VAL_Z: val->as.r = (float) val->as.z; break;
      case VAL_R: return TRUE; /* do noþing */
      default:
        /*err_cast(from.type, VAL_R);*/
        printf("err cast R");
        return FALSE;
    }
    val->type = VAL_R;
    return TRUE;
}

static int op_ape(struct VirMac *vm)
{
    struct DfVal elem = virmac_pop(vm);
    struct DfVal arr  = virmac_pop(vm);
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        fprintf(stderr, "ERROR: value is not an array\n");
        return FALSE;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    if (!objarr_try_push(a, &elem)) {
        fputs("ERROR: some error pushing into array\n", stderr);
    }
    virmac_push(vm, &arr);
    return TRUE;
}

static int op_age(struct VirMac *vm)
{
    struct DfVal arr, idx;
    idx = virmac_pop(vm);
    arr = virmac_pop(vm);
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        eputln("ERROR: value is not an array");
        return FALSE;
    }
    if (idx.type != VAL_N) {
        eputln("ERROR: index is not N%");
        return FALSE;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    struct DfVal val = objarr_get(a, idx.as.n);
    if (val.type == VAL_V) {
        eputln("ERROR: index out of bounds");
        return FALSE;
    }
    virmac_push(vm, &val);
    return TRUE;
}

static int op_ase(struct VirMac *vm)
{
    struct DfVal arr, idx, val;
    val = virmac_pop(vm);
    idx = virmac_pop(vm);
    arr = virmac_pop(vm);
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        eputln("ERROR: value is not an array");
        return FALSE;
    }
    if (idx.type != VAL_N) {
        eputln("ERROR: index is not N%");
        return FALSE;
    }
    struct ObjArr *a = OBJ_AS_ARR(arr.as.o);
    return objarr_set(a, idx.as.n, &val); /* OK or ERR result */
}

static int op_tsf(struct VirMac *vm)
{
    struct DfVal tbl, val;
    val = virmac_pop(vm);
    tbl = virmac_pop(vm);
#ifdef SAFE
    if (tbl.type != VAL_O || tbl.as.o->type != OBJ_TBL) {
        eputln("ERROR: value is not a table");
        return FALSE;
    }
#endif /* SAFE */
    struct DfIdf *idf = &vm->dat->idf.arr[read_u16(&vm->ip)];
    if (!objtbl_set(OBJ_AS_TBL(tbl.as.o), idf, val))
        return FALSE;
    virmac_push(vm, &tbl);
    return TRUE;
}

static int op_tgf(struct VirMac *vm)
{
    struct DfVal tbl, val;
    tbl = virmac_pop(vm);
#ifdef SAFE
    if (tbl.type != VAL_O || tbl.as.o->type != OBJ_TBL) {
        eputln("ERROR: value is not a table");
        return FALSE;
    }
#endif /* SAFE */
    struct DfIdf *idf = &vm->dat->idf.arr[read_u16(&vm->ip)];
    int res = objtbl_get(OBJ_AS_TBL(tbl.as.o), idf, &val);
    if (!res)
        fprintf(stderr, "field $%s' not found in table\n", idf->str);
    else
        virmac_push(vm, &val);
    return res;
}

static int op_pcl(struct VirMac *vm)
{
    uint8_t arity = READ_BYTE();
    struct DfVal *val = vm->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val->type != VAL_O || val->as.o->type != OBJ_PRO) {
        eputln("cannot !call a not !");
        return FALSE;
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
        return FALSE;
    }
#endif /* SAFE */
    return push_call(vm, val, pro->as.usr);
}

static int op_fcl(struct VirMac *vm)
{
    uint8_t arity = READ_BYTE();
    struct DfVal *val = vm->sp - (arity + 1); /* args + callee */
#ifdef SAFE
    if (val->type != VAL_O || val->as.o->type != OBJ_FUN) {
        eputln("ERROR: cannot #call a not #");
        return FALSE;
    }
#endif /* SAFE */
    struct ObjFun *fun = OBJ_AS_FUN(val->as.o);
#ifdef SAFE
    if (fun->norr->ari != arity) {
        printf("ERROR: wrong arity calling ");
        object_print(val->as.o);
        puts("");
        return FALSE;
    }
#endif /* SAFE */
    if (!push_call(vm, val, fun->norr))
        return FALSE;
    return TRUE;
}

static int op_ret(struct VirMac *vm)
{
    struct DfVal ret = virmac_pop(vm);
    if (!pop_call(vm))
        return FALSE;
    virmac_push(vm, &ret);
    return TRUE;
}

/* Load Local Short (u8) */
static void op_lls(struct VirMac *vm)
{
    uint index = READ_BYTE();
    struct DfVal *loc = &vm->bp[index];
    virmac_push(vm, loc);
}

/* Store Local Short (u8) */
static void op_sls(struct VirMac *vm)
{
    uint index = READ_BYTE();
    vm->bp[index] = virmac_pop(vm);
}

/* Update Local Short (u8) */
static void op_uls(struct VirMac *vm)
{
    uint index = READ_BYTE();
    vm->bp[index] = *virmac_peek(vm);
}

static int op_jbf(struct VirMac *vm)
{
    struct DfVal *b = virmac_peek(vm);
    if (b->type != VAL_B) {
        fputs("condition is not B\n", stderr);
        return FALSE;
    }
    vm_js_if(vm, !b->as.b);
    return TRUE;
}

#define OP_JFX(x) \
static int op_jf ## x (struct VirMac *vm) \
{                                     \
    struct DfVal b = virmac_pop(vm);  \
    if (b.type != VAL_B) {            \
        eputln("condition is not B"); \
        return FALSE;                 \
    }                                 \
    vm_j ## x ## _if(vm, !b.as.b);    \
    return TRUE;                      \
}

OP_JFX(s)
OP_JFX(l)

#undef OP_JFX

#define OP_J_CMP(name, cmp_fn, msg) \
static int name(struct VirMac *vm)      \
{                                       \
    int cmp;                            \
    struct DfVal lhs, rhs;              \
    rhs = virmac_pop(vm);               \
    lhs = virmac_pop(vm);               \
    switch ((cmp = cmp_fn(&lhs, &rhs))) { \
      case CMP_ERR:                     \
        ERR_BINOP(msg);                 \
        return FALSE;                   \
      default:                          \
        vm_jl_if(vm, cmp);              \
        return TRUE;                    \
    }                                   \
}

OP_J_CMP(op_jlt, dfval_lt, ">=")
OP_J_CMP(op_jle, dfval_le, ">")
OP_J_CMP(op_jgt, dfval_gt, "<=")
OP_J_CMP(op_jge, dfval_ge, "<")

#undef OP_J_CMP
