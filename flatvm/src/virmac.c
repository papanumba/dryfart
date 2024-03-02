/* virmac.c */

#include <stdio.h>
#include <stdlib.h>
#include "virmac.h"
#include "object.h"
#include "alzhmr.h"
#include "falloc.h"
#include "garcol.h"

#ifdef DEBUG
#include "disasm.h"
#endif

#define READ_BYTE() (*vm->ip++)
#define CMP_ERR     2

/* static functions */
static void print_stack(struct VirMac *);
static void reset_stack(struct VirMac *);
static enum ItpRes run (struct VirMac *);
static int push_call   (struct VirMac *, struct DfVal *, struct Norris *);
static int pop_call    (struct VirMac *);
static void print_calls(const struct VirMac *);
static void set_norris (struct VirMac *, struct Norris *);
void err_cant_op  (const char *, struct DfVal *);
void err_dif_types(const char *, enum DfType, enum DfType);

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
static int op_add_o(
    struct Object *,
    struct Object *,
    struct DfVal  *);
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
static int op_can(struct VirMac *);
static int op_caz(struct VirMac *);
static int op_car(struct VirMac *);

static int op_ape(struct VirMac *);
static int op_age(struct VirMac *);
static int op_ase(struct VirMac *);

static int op_tsf(struct VirMac *);
static int op_tgf(struct VirMac *);

static int op_pcl(struct VirMac *);
static int op_fcl(struct VirMac *);
static int op_ret(struct VirMac *);

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
    vm->callnum = -1;
    vm->bp = vm->sp;
}

void virmac_init(struct VirMac *vm)
{
    reset_stack(vm);
    vm->dat = NULL;
    vm->nor = NULL;
    falloc_init();
    garcol_init();
}

void virmac_free(struct VirMac *vm)
{
    reset_stack(vm);
    falloc_exit();
    garcol_exit();
}

enum ItpRes virmac_run(struct VirMac *vm, struct VmData *prog)
{
    assert(vm != NULL);
    vm->dat = prog;
    /* start main */
    push_call(vm, vm->stack, &prog->pag.arr[0]);
    enum ItpRes res = run(vm);
    if (res != ITP_OK)
        print_calls(vm);
    return res;
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
    if (vm->sp == vm->bp) {
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
    if (vm->sp == vm->bp) {
        fputs("ERROR: empty stack\n", stderr);
        exit(1);
    }
#endif /* SAFE */
    return &vm->sp[-1];
}

static enum ItpRes run(struct VirMac *vm)
{
    while (TRUE) {
        uint8_t ins;
#ifdef DEBUG
        print_stack(vm);
        disasm_instru(vm->dat, vm->nor, vm->ip);
#endif /* DEBUG */
        switch (ins = READ_BYTE()) {
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
            uint idx = READ_BYTE();
            virmac_push(vm, &vm->dat->ctn.arr[idx]);
            break;
          }
          case OP_LKL: {
            uint idx = read_u16(&vm->ip);
            virmac_push(vm, &vm->dat->ctn.arr[idx]);
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

          DO_OP(OP_CAN, op_can)
          DO_OP(OP_CAZ, op_caz)
          DO_OP(OP_CAR, op_car)

          DO_OP(OP_APE, op_ape)
          DO_OP(OP_AGE, op_age)
          DO_OP(OP_ASE, op_ase)

          DO_OP(OP_TSF, op_tsf)
          DO_OP(OP_TGF, op_tgf)

          DO_OP(OP_PCL, op_pcl)
          DO_OP(OP_FCL, op_fcl)
          DO_OP(OP_RET, op_ret)

          DO_OP(OP_JBF, op_jbf)
          DO_OP(OP_JFS, op_jfs)
          DO_OP(OP_JFL, op_jfl)
          DO_OP(OP_JLT, op_jlt)
          DO_OP(OP_JLE, op_jle)
          DO_OP(OP_JGT, op_jgt)
          DO_OP(OP_JGE, op_jge)
#undef DO_OP

          case OP_JJS: {
            int dist = read_i8(&vm->ip);
            vm->ip += dist;
            break;
          }
          case OP_JJL: {
            int dist = read_i16(&vm->ip);
            vm->ip += dist;
            break;
          }

          case OP_AMN: {
            struct DfVal val;
            val.type = VAL_O;
            val.as.o = (struct Object *) objarr_new();
            virmac_push(vm, &val);
            break;
          }
          case OP_TMN: {
            struct DfVal val;
            val.type = VAL_O;
            val.as.o = (struct Object *) objtbl_new();
            virmac_push(vm, &val);
            break;
          }

          case OP_PMN: {
            struct DfVal val;
            uint idx = read_u16(&vm->ip);
            val.type = VAL_O;
            val.as.o = (void *) objpro_new(&vm->dat->pag.arr[idx]);
            virmac_push(vm, &val);
            break;
          }

          case OP_FMN: {
            struct DfVal val;
            uint idx = read_u16(&vm->ip);
            val.type = VAL_O;
            val.as.o = (void *) objfun_new(&vm->dat->pag.arr[idx]);
            virmac_push(vm, &val);
            break;
          }

          case OP_END: {
            if (!pop_call(vm)) return ITP_RUNTIME_ERR;
            break;
          }
          case OP_DUP: {
            struct DfVal *val = virmac_peek(vm);
            virmac_push(vm, val);
            break;
          }
          case OP_POP: virmac_pop(vm); break;
          case OP_HLT:
            puts("VM HALTED");
            print_calls(vm);
            print_stack(vm);
            reset_stack(vm);
            garcol_do(vm);
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
        printf("[%c%%", val2type(slot));
        values_print(slot);
        printf("]");
    }
    printf("\n");
}

static int push_call(struct VirMac *vm, struct DfVal *c, struct Norris *n)
{
#ifdef SAFE
    if (vm->callnum == CALLS_MAX) {
        eputln("ERROR: call stack overflow");
        return FALSE;
    }
#endif /* SAFE */
#ifdef DEBUG
    if (c != NULL) {
        printf("calling "); values_print(c); printf("------------\n");
    }
#endif /* DEBUG */
    vm->calls[vm->callnum] = vm->bp;
    vm->norrs[vm->callnum] = vm->nor;
    vm->ips  [vm->callnum] = vm->ip;
    vm->callnum++;
    vm->bp = c;
    set_norris(vm, n);
    return TRUE;
}

static int pop_call(struct VirMac *vm)
{
#ifdef SAFE
    if (vm->callnum == -1) {
        eputln("ERROR: empty call stack\n");
        return FALSE;
    }
#endif /* SAFE */
#ifdef DEBUG
    printf("end call "); values_print(vm->bp); printf("------------\n");
#endif /* DEBUG */
    vm->callnum--;
    vm->sp = vm->bp;
    vm->bp  = vm->calls[vm->callnum];
    vm->ip  = vm->ips  [vm->callnum];
    vm->nor = vm->norrs[vm->callnum];
    return TRUE;
}

static void print_calls(const struct VirMac *vm)
{
    puts("Call stack (top oldest):\n    !main");
    int last = vm->callnum;
#define BASURA(vp) MACRO_STMT(printf("    "); values_print(vp); puts("");)
    for (int i = 1; i < last; ++i)
        BASURA(vm->calls[i]);
    if (last > 0)
        BASURA(vm->bp);
    puts("");
#undef BASURA
}

static void set_norris(struct VirMac *vm, struct Norris *n)
{
    vm->nor = n;
    vm->ip = n->cod;
}

/* error message for same type but invalid operations */
void err_cant_op(const char *op, struct DfVal *v)
{
    char ty = val2type(v);
    fprintf(stderr, "ERROR: Cannot operate %s with %c value(s)\n", op, ty);
}

void err_dif_types(const char *op, enum DfType t1, enum DfType t2)
{
    fprintf(stderr, "ERROR: Cannot operate %s with types %c and %c\n",
        op, t1, t2);
}

#define ERR_BINOP(msg)  err_dif_types(msg, val2type(&lhs), val2type(&rhs))

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
        eputln("unknown type in dfval_ne");
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
        int dist = read_i8(&vm->ip);
        vm->ip += dist;
    } else {
        vm->ip += 1;
    }
}

/* jump long if `cond` */
static void vm_jl_if(struct VirMac *vm, int cond)
{
    if (cond) {
        int dist = read_i16(&vm->ip);
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
        err_cant_op("unary -", &val);
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
static int op_add_o(
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
    htable_set(&OBJ_AS_TBL(tbl.as.o)->tbl, idf, val);
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
    int res = htable_get(&OBJ_AS_TBL(tbl.as.o)->tbl, idf, &val);
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
#ifdef SAFE
    if (pro->norr->ari != arity) {
        printf("wrong arity calling ");
        object_print(val->as.o);
        puts("");
        return FALSE;
    }
#endif /* SAFE */
    if (!push_call(vm, val, pro->norr))
        return FALSE;
    return TRUE;
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
