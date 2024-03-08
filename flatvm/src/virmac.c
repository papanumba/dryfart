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
static inline void vm_js_if(struct VirMac *, int);
static inline void vm_jl_if(struct VirMac *, int);

#include "vm-ops.c"

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
#ifdef DEBUG
            puts("VM HALTED");
            print_calls(vm);
            print_stack(vm);
#endif
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
static inline void vm_js_if(struct VirMac *vm, int cond)
{
    if (cond) {
        int dist = read_i8(&vm->ip);
        vm->ip += dist;
    } else {
        vm->ip += 1;
    }
}

/* jump long if `cond` */
static inline void vm_jl_if(struct VirMac *vm, int cond)
{
    if (cond) {
        int dist = read_i16(&vm->ip);
        vm->ip += dist;
    } else {
        vm->ip += 2;
    }
}
