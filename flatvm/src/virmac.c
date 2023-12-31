/* virmac.c */

#include <stdio.h>
#include <stdlib.h>
#include "virmac.h"
#include "object.h"
#include "alzhmr.h"

#ifdef DEBUG
#include "disasm.h"
#endif

#define READ_BYTE() (*vm->ip++)
#define CMP_ERR     2

/* static functions */
static void print_stack(struct VirMac *);
static void reset_stack(struct VirMac *);
static enum ItpRes run (struct VirMac *);
void err_cant_op  (const char *, enum ValType);
void err_dif_types(const char *, enum ValType, enum ValType);

static int8_t   read_i8 (struct VirMac *);
static uint16_t read_u16(struct VirMac *);
static int16_t  read_i16(struct VirMac *);
static int dfval_eq(struct DfVal *, struct DfVal *);
static int dfval_ne(struct DfVal *, struct DfVal *);
static int dfval_lt(struct DfVal *, struct DfVal *);
static int dfval_le(struct DfVal *, struct DfVal *);
static int dfval_gt(struct DfVal *, struct DfVal *);
static int dfval_ge(struct DfVal *, struct DfVal *);
static void vm_js_if(struct VirMac *, int);
static void vm_jl_if(struct VirMac *, int);

/* pre-built constant loading */
static void op_lvv(struct VirMac *);
static void op_lb(struct VirMac *, int);
static void op_ln(struct VirMac *, uint32_t);
static void op_lz(struct VirMac *, int32_t);
static void op_lr(struct VirMac *, float);

/* numeric */
static int op_neg(struct VirMac *);
static int op_add(struct VirMac *);
static int op_sub(struct VirMac *);
static int op_mul(struct VirMac *);
static int op_div(struct VirMac *);
static int op_inv(struct VirMac *);
static int op_inc(struct VirMac *);
static int op_dec(struct VirMac *);

/* comparison */
static void op_ceq(struct VirMac *);
static void op_cne(struct VirMac *);
static int  op_clt(struct VirMac *);
static int  op_cle(struct VirMac *);
static int  op_cgt(struct VirMac *);
static int  op_cge(struct VirMac *);

/* boolean & bitwise */
static int op_not(struct VirMac *);
static int op_and(struct VirMac *);
static int op_ior(struct VirMac *);

/* casts */
static int op_caz(struct VirMac *);
static int op_car(struct VirMac *);

static int op_tpe(struct VirMac *);
static int op_tge(struct VirMac *);

static int  op_lgl(struct VirMac *);
static int  op_sgl(struct VirMac *);
static void op_lls(struct VirMac *);
static void op_sls(struct VirMac *);
static void op_uls(struct VirMac *);

static int op_jbf(struct VirMac *);
static int op_jfl(struct VirMac *);
static int op_jfs(struct VirMac *);
static int op_jlt(struct VirMac *);
static int op_jle(struct VirMac *);
static int op_jgt(struct VirMac *);
static int op_jge(struct VirMac *);

static void reset_stack(struct VirMac *vm)
{
    vm->sp = &vm->stack[0];
}

void virmac_init(struct VirMac *vm)
{
    reset_stack(vm);
    htable_init(&vm->globals);
}

void virmac_free(struct VirMac *vm)
{
    reset_stack(vm);
    htable_free(&vm->globals);
}

enum ItpRes virmac_run(struct VirMac *vm, struct Norris *bc)
{
    if (vm == NULL || bc == NULL || bc->cod == NULL)
        return ITP_NULLPTR_ERR;
    vm->norris = bc;
    vm->ip = bc->cod;
    return run(vm);
}

void virmac_push(struct VirMac *vm, struct DfVal *v)
{
#ifdef SAFE
    if (vm->sp == &vm->stack[STACK_MAX]) {
        fputs("ERROR: stack overflow\n", stderr);
        exit(1);
    }
#endif /* SAFE */
    *vm->sp = *v;
    vm->sp++;
}

struct DfVal virmac_pop(struct VirMac *vm)
{
#ifdef SAFE
    if (vm->sp == vm->stack) {
        fputs("ERROR: empty stack\n", stderr);
        exit(1);
    }
#endif /* SAFE */
    vm->sp--;
    return *vm->sp;
}

struct DfVal * virmac_peek(struct VirMac *vm)
{
#ifdef SAFE
    if (vm->sp == vm->stack) {
        fputs("ERROR: empty stack\n", stderr);
        exit(1);
    }
#endif /* SAFE */
    return &vm->sp[-1];
}

static enum ItpRes run(struct VirMac *vm)
{
    while (1) {
        uchar ins;
#ifdef DEBUG
        print_stack(vm);
        disasm_instru(vm->norris, (uint) (vm->ip - vm->norris->cod));
#endif /* DEBUG */
        ins = READ_BYTE();
        switch (ins) {
          case OP_NOP: break;

/* void ops */
#define DO_OP(op, fn) case op: fn(vm); break;
          DO_OP(OP_LVV, op_lvv)

          DO_OP(OP_LLS, op_lls)
          DO_OP(OP_SLS, op_sls)
          DO_OP(OP_ULS, op_uls)

          DO_OP(OP_CEQ, op_ceq)
          DO_OP(OP_CNE, op_cne)
#undef DO_OP

/* consts */
#define DO_L(op, fn, val) case op: fn(vm, val); break;
          DO_L(OP_LBT, op_lb, TRUE)
          DO_L(OP_LBF, op_lb, FALSE)
          DO_L(OP_LN0, op_ln, 0)
          DO_L(OP_LN1, op_ln, 1)
          DO_L(OP_LN2, op_ln, 2)
          DO_L(OP_LN3, op_ln, 3)
          DO_L(OP_LM1, op_lz, -1)
          DO_L(OP_LZ0, op_lz, 0)
          DO_L(OP_LZ1, op_lz, 1)
          DO_L(OP_LZ2, op_lz, 2)
          DO_L(OP_LR0, op_lr, 0.0f)
          DO_L(OP_LR1, op_lr, 1.0f)
#undef DO_L

          case OP_LKS: {
            uchar idx = READ_BYTE();
            virmac_push(vm, &vm->norris->ctn.arr[idx]);
            break;
          }
          case OP_LKL: {
            ushort idx = read_u16(vm);
            virmac_push(vm, &vm->norris->ctn.arr[idx]);
            break;
          }

/* fallible ops */
#define DO_OP(op, fn) case op: if (!fn(vm)) return ITP_RUNTIME_ERR; break;
          DO_OP(OP_NEG, op_neg)
          DO_OP(OP_ADD, op_add)
          DO_OP(OP_SUB, op_sub)
          DO_OP(OP_MUL, op_mul)
          DO_OP(OP_DIV, op_div)
          DO_OP(OP_INV, op_inv)
          DO_OP(OP_INC, op_inc)
          DO_OP(OP_DEC, op_dec)

          DO_OP(OP_CLT, op_clt)
          DO_OP(OP_CLE, op_cle)
          DO_OP(OP_CGT, op_cgt)
          DO_OP(OP_CGE, op_cge)

          DO_OP(OP_NOT, op_not)
          DO_OP(OP_AND, op_and)
          DO_OP(OP_IOR, op_ior)

          DO_OP(OP_CAZ, op_caz)
          DO_OP(OP_CAR, op_car)

          DO_OP(OP_TPE, op_tpe)
          DO_OP(OP_TGE, op_tge)

          DO_OP(OP_LGL, op_lgl)
          DO_OP(OP_SGL, op_sgl)

          DO_OP(OP_JBF, op_jbf)
          DO_OP(OP_JFS, op_jfs)
          DO_OP(OP_JFL, op_jfl)
          DO_OP(OP_JLT, op_jlt)
          DO_OP(OP_JLE, op_jle)
          DO_OP(OP_JGT, op_jgt)
          DO_OP(OP_JGE, op_jge)
#undef DO_OP

          case OP_JJS: {
            int dist = read_i8(vm);
            vm->ip += dist;
            break;
          }
          case OP_JJL: {
            int dist = read_i16(vm);
            vm->ip += dist;
            break;
          }

          case OP_MEA: {
            struct DfVal val;
            val.type = VAL_O;
            val.as.o = (struct Object *) objarr_new();
            virmac_push(vm, &val);
            break;
          }

          case OP_RET: {
            struct DfVal v = virmac_pop(vm);
            values_print(&v);
            fputs("\n", stdout);
            return ITP_OK;
          }
          case OP_DUP: {
            struct DfVal *val = virmac_peek(vm);
            virmac_push(vm, val);
            break;
          }
          case OP_POP: virmac_pop(vm); break;
          case OP_HLT:
            print_stack(vm);
            puts("globals: ");
            htable_print(&vm->globals);
            htable_free (&vm->globals);
            return ITP_OK;

          default:
            fprintf(stderr, "unknown instruction %02x\n", ins);
        }
    }
}

static void print_stack(struct VirMac *vm)
{
    struct DfVal *slot = NULL;
    for (slot = &vm->stack[0];
         slot != vm->sp;
         slot++) {
        printf("[%c%%", values_type_to_char(slot->type));
        values_print(slot);
        printf("]");
    }
    printf("\n");
}

/* error message for same type but invalid operations */
void err_cant_op(const char *op, enum ValType ty)
{
    fprintf(stderr, "ERROR: Cannot operate %s with %c value(s)\n",
        op, values_type_to_char(ty));
}

void err_dif_types(const char *op, enum ValType t1, enum ValType t2)
{
    fprintf(stderr, "ERROR: Cannot operate %s with types %c and %c\n",
        op, values_type_to_char(t1), values_type_to_char(t2));
}

static uint16_t read_u16(struct VirMac *vm)
{
    uint8_t b0, b1;
    b0 = READ_BYTE();
    b1 = READ_BYTE();
    return (b0 << 8) | b1;
}

static int16_t read_i16(struct VirMac *vm)
{
    union {uint16_t u; int16_t s;} aux;
    aux.u = read_u16(vm);
    return aux.s;
}

static int8_t read_i8(struct VirMac *vm)
{
    union {uint8_t u; int8_t s;} aux;
    aux.u = READ_BYTE();
    return aux.s;
}

static int dfval_eq(struct DfVal *v, struct DfVal *w)
{
    if (v->type != w->type)
        return FALSE;
    switch (v->type) {
      case VAL_V: return TRUE;
      case VAL_B: return !!v->as.b == !!w->as.b; /* for oþer non-0 values */
      case VAL_C: return v->as.c == w->as.c;
      case VAL_N: return v->as.n == w->as.n;
      case VAL_Z: return v->as.z == w->as.z;
      case VAL_R: return FALSE;
      case VAL_O: return object_eq(v->as.o, w->as.o);
      default:
        fputs("unknown type in dfval_eq\n", stderr);
        return FALSE;
    }
}

static int dfval_ne(struct DfVal *v, struct DfVal *w)
{
    if (v->type != w->type)
        return TRUE;
    switch (v->type) {
      case VAL_V: return FALSE;
      case VAL_B: return v->as.b != w->as.b;
      case VAL_C: return v->as.c != w->as.c;
      case VAL_N: return v->as.n != w->as.n;
      case VAL_Z: return v->as.z != w->as.z;
      case VAL_R: return TRUE;
      case VAL_O: return !object_eq(v->as.o, w->as.o); /* ! eq */
      default:
        fputs("unknown type in dfval_ne\n", stderr);
        return TRUE;
    }
}

/* see C99's §6.5.8 Relational Operators ¶6 */

#define DFVAL_CMP_FN(name, cmpop) \
static int name(struct DfVal *lhs, struct DfVal *rhs) \
{                                                   \
    if (lhs->type != rhs->type)                     \
        return CMP_ERR;                             \
    switch (lhs->type) {                            \
      case VAL_N: return lhs->as.n cmpop rhs->as.n; \
      case VAL_Z: return lhs->as.z cmpop rhs->as.z; \
      case VAL_R: return lhs->as.r cmpop rhs->as.r; \
      default:    return CMP_ERR;                   \
    }                                               \
}

DFVAL_CMP_FN(dfval_lt, <)
DFVAL_CMP_FN(dfval_le, <=)
DFVAL_CMP_FN(dfval_gt, >)
DFVAL_CMP_FN(dfval_ge, >=)

#undef DFVAL_CMP_FN

/* jump short if `cond` */
static void vm_js_if(struct VirMac *vm, int cond)
{
    if (cond) {
        int dist = read_i8(vm);
        vm->ip += dist;
    } else {
        vm->ip += 1;
    }
}

/* jump long if `cond` */
static void vm_jl_if(struct VirMac *vm, int cond)
{
    if (cond) {
        int dist = read_i16(vm);
        vm->ip += dist;
    } else {
        vm->ip += 2;
    }
}

/* most funcs */

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
        err_cant_op("unary -", val.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_add(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("+", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N: res.as.n = lhs.as.n + rhs.as.n; break;
      case VAL_Z: res.as.z = lhs.as.z + rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r + rhs.as.r; break;
      default:
        err_cant_op("+", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_sub(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("-", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_Z: res.as.z = lhs.as.z - rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r - rhs.as.r; break;
      default:
        err_cant_op("-", lhs.type);
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
        err_dif_types("*", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N: res.as.n = lhs.as.n * rhs.as.n; break;
      case VAL_Z: res.as.z = lhs.as.z * rhs.as.z; break;
      case VAL_R: res.as.r = lhs.as.r * rhs.as.r; break;
      default:
        err_cant_op("*", lhs.type);
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
        err_dif_types("/", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_R:
#ifdef SAFE
        if (rhs.as.r == 0.0f)
            panic("ERROR: Division by 0.0");
#endif /* SAFE */
        res.as.r = lhs.as.r / rhs.as.r;
        break;
      default:
        err_cant_op("/", lhs.type);
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
        err_cant_op("unary /", val->type);
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
            err_cant_op("1 +", val->type);
            return FALSE;
    }
    return TRUE;
}

static int op_dec(struct VirMac *vm)
{
    struct DfVal *val = virmac_peek(vm);
    switch (val->type) {
/*        case VAL_N: val->as.n += 1; break;*/
        case VAL_Z: val->as.z -= 1; break;
        default:
            err_cant_op("1 +", val->type);
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
        err_dif_types(msg, lhs.type, rhs.type); \
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
        err_cant_op("unary ~", val.type);
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
    if (lhs.type != rhs.type)
        return FALSE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b && rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n &  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("&", lhs.type);
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
    if (lhs.type != rhs.type)
        return FALSE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B: res.as.b = lhs.as.b || rhs.as.b; break;
      case VAL_N: res.as.n = lhs.as.n |  rhs.as.n; break; /* bitwise */
      default:
        err_cant_op("|", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
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

static int op_tpe(struct VirMac *vm)
{
    struct DfVal elem = virmac_pop(vm);
    struct DfVal arr = virmac_pop(vm);
    if (arr.type != VAL_O || arr.as.o->type != OBJ_ARR) {
        fprintf(stderr, "ERROR: value is not an array\n");
        return FALSE;
    }
    struct ObjArr *a = (struct ObjArr *) arr.as.o;
    if (!objarr_try_push(a, &elem)) {
        fputs("ERROR: some error pushing into array\n", stderr);
    }
    virmac_push(vm, &arr);
    return TRUE;
}

static int op_tge(struct VirMac *vm)
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
    struct ObjArr *a = (struct ObjArr *) arr.as.o;
    struct DfVal val = objarr_get(a, idx.as.n);
    if (val.type == VAL_V) {
        eputln("ERROR: index out of bounds");
        return FALSE;
    }
    virmac_push(vm, &val);
    return TRUE;
}

static int op_lgl(struct VirMac *vm)
{
    struct DfIdf *idf = &vm->norris->idf.arr[read_u16(vm)];
    struct DfVal glo;
    if (!htable_get(&vm->globals, idf, &glo)) { /* key not found */
        fprintf(stderr, "global identifier '%s' not found\n", idf->str);
        return FALSE;
    }
    virmac_push(vm, &glo);
    return TRUE;
}

static int op_sgl(struct VirMac *vm)
{
    struct DfIdf *idf = &vm->norris->idf.arr[read_u16(vm)];
    htable_set(&vm->globals, idf, virmac_pop(vm));
    return TRUE;
}

/* Load Local Short */
static void op_lls(struct VirMac *vm)
{
    struct DfVal *loc = &vm->stack[READ_BYTE()];
    virmac_push(vm, loc);
}

/* Store Local Short (u8) */
static void op_sls(struct VirMac *vm)
{
    uint index = READ_BYTE();
    vm->stack[index] = virmac_pop(vm);
}

/* Update Local Short (u8) */
static void op_uls(struct VirMac *vm)
{
    uint index = READ_BYTE();
    vm->stack[index] = *virmac_peek(vm);
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

static int op_jfs(struct VirMac *vm)
{
    struct DfVal b = virmac_pop(vm);
    if (b.type != VAL_B) {
        fputs("condition is not B\n", stderr);
        return FALSE;
    }
    vm_js_if(vm, !b.as.b);
    return TRUE;
}

static int op_jfl(struct VirMac *vm)
{
    struct DfVal b = virmac_pop(vm);
    if (b.type != VAL_B) {
        fputs("condition is not B\n", stderr);
        return FALSE;
    }
    vm_jl_if(vm, !b.as.b);
    return TRUE;
}

#define OP_J_CMP(name, cmp_fn, msg) \
static int name(struct VirMac *vm)      \
{                                       \
    int cmp;                            \
    struct DfVal lhs, rhs;              \
    rhs = virmac_pop(vm);               \
    lhs = virmac_pop(vm);               \
    switch ((cmp = cmp_fn(&lhs, &rhs))) { \
      case CMP_ERR:                     \
        err_dif_types(msg, lhs.type, rhs.type); \
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
