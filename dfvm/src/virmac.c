/* virmac.c */

#include <stdio.h>
#include <stdlib.h>
#include "virmac.h"
#include "idents.h"
#include "object.h"
#include "alzhmr.h"

#ifdef DEBUG
#include "disasm.h"
#endif

#define READ_BYTE() (*vm->ip++)

/* static functions */
#ifdef DEBUG
static void print_stack(struct VirMac *);
#endif
static void reset_stack(struct VirMac *);
static enum ItpRes run (struct VirMac *);
void err_cant_op  (const char *, enum ValType);
void err_dif_types(const char *, enum ValType, enum ValType);

/* pre-built constant loading */
static void op_lvv(struct VirMac *vm);
static void op_lbt(struct VirMac *vm);
static void op_lbf(struct VirMac *vm);
static void op_ln0(struct VirMac *vm);
static void op_ln1(struct VirMac *vm);
static void op_lm1(struct VirMac *vm);
static void op_lz0(struct VirMac *vm);
static void op_lz1(struct VirMac *vm);
static void op_lr0(struct VirMac *vm);
static void op_lr1(struct VirMac *vm);

/* numeric */
static int op_neg(struct VirMac *);
static int op_add(struct VirMac *);
static int op_sub(struct VirMac *);
static int op_mul(struct VirMac *);
static int op_div(struct VirMac *);
static int op_inv(struct VirMac *);

/* comparison */
static int op_ceq(struct VirMac *);
static int op_cne(struct VirMac *);
static int op_clt(struct VirMac *);
static int op_cle(struct VirMac *);
static int op_cgt(struct VirMac *);
static int op_cge(struct VirMac *);

/* boolean & bitwise */
static int op_not(struct VirMac *);
static int op_and(struct VirMac *);
static int op_ior(struct VirMac *);


static void reset_stack(struct VirMac *vm)
{
    vm->sp = &vm->stack[0];
}

void virmac_init(struct VirMac *vm)
{
    reset_stack(vm);
}

void virmac_free(struct VirMac *vm)
{
    virmac_init(vm);
}

enum ItpRes virmac_run(struct VirMac *vm, struct Norris *bc)
{
    if (vm == NULL || bc == NULL || bc->cod == NULL)
        return ITP_NULLPTR_ERR;
    disasm_norris(bc, "main");
    vm->norris = bc;
    vm->ip = bc->cod;
    return run(vm);
}

void virmac_push(struct VirMac *vm, struct DfVal *v)
{
    if (vm->sp == &vm->stack[STACK_MAX]) {
        fputs("ERROR: stack overflow\n", stderr);
        exit(1);
    }
    *vm->sp = *v;
    vm->sp++;
}

struct DfVal virmac_pop(struct VirMac *vm)
{
    if (vm->sp == vm->stack) {
        fputs("ERROR: empty stack\n", stderr);
        exit(1);
    }
    vm->sp--;
    return *vm->sp;
}

static enum ItpRes run(struct VirMac *vm)
{
    while (1) {
#ifdef DEBUG
        print_stack(vm);
        disasm_instru(vm->norris, (uint) (vm->ip - vm->norris->cod));
#endif
        switch (READ_BYTE()) {
          case OP_CTN:
            virmac_push(vm, &vm->norris->ctn.arr[READ_BYTE()]);
            break;
          case OP_CTL:
            virmac_push(vm, &vm->norris->ctn.arr[b2toh(vm->ip)]);
            vm->ip += 2;
            break;

          case OP_LVV: op_lvv(vm); break;
          case OP_LBT: op_lbt(vm); break;
          case OP_LBF: op_lbf(vm); break;
          case OP_LN0: op_ln0(vm); break;
          case OP_LN1: op_ln1(vm); break;
          case OP_LM1: op_lm1(vm); break;
          case OP_LZ0: op_lz0(vm); break;
          case OP_LZ1: op_lz1(vm); break;
          case OP_LR0: op_lr0(vm); break;
          case OP_LR1: op_lr1(vm); break;

#define DO_OP(op, fn) case op: if (!fn(vm)) return ITP_RUNTIME_ERR; break;
          DO_OP(OP_NEG, op_neg)
          DO_OP(OP_ADD, op_add)
          DO_OP(OP_SUB, op_sub)
          DO_OP(OP_MUL, op_mul)
          DO_OP(OP_DIV, op_div)
          DO_OP(OP_INV, op_inv)

          DO_OP(OP_CEQ, op_ceq)
          DO_OP(OP_CNE, op_cne)
          DO_OP(OP_CLT, op_clt)
          DO_OP(OP_CLE, op_cle)
          DO_OP(OP_CGT, op_cgt)
          DO_OP(OP_CGE, op_cge)

          DO_OP(OP_NOT, op_not)
          DO_OP(OP_AND, op_and)
          DO_OP(OP_IOR, op_ior)
#undef DO_OP

          case OP_RET:
            values_print(virmac_pop(vm));
            fputs("\n", stdout);
            return ITP_OK;
          default:
            fputs("unknown instruction\n", stderr);
        }
    }
}

#ifdef DEBUG
static void print_stack(struct VirMac *vm)
{
    struct DfVal *slot = NULL;
    for (slot = &vm->stack[0];
         slot != vm->sp;
         slot++) {
        printf("[");
        values_print(*slot);
        printf("]");
    }
    printf("\n");
}
#endif

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

/* most funcs */

static void op_lvv(struct VirMac *vm)
{
    struct DfVal v;
    v.type = VAL_V;
/*    v.as.b = FALSE;*/ /* FIXME should initialize? */
    virmac_push(vm, &v);
}

static void op_lbt(struct VirMac *vm)
{
    struct DfVal bt;
    bt.type = VAL_B;
    bt.as.b = TRUE;
    virmac_push(vm, &bt);
}

static void op_lbf(struct VirMac *vm)
{
    struct DfVal bf;
    bf.type = VAL_B;
    bf.as.b = FALSE;
    virmac_push(vm, &bf);
}

static void op_ln0(struct VirMac *vm)
{
    struct DfVal n0;
    n0.type = VAL_N;
    n0.as.n = 0;
    virmac_push(vm, &n0);
}

static void op_ln1(struct VirMac *vm)
{
    struct DfVal n1;
    n1.type = VAL_N;
    n1.as.n = 1;
    virmac_push(vm, &n1);
}

static void op_lm1(struct VirMac *vm)
{
    struct DfVal m1;
    m1.type = VAL_Z;
    m1.as.z = 1;
    virmac_push(vm, &m1);
}

static void op_lz0(struct VirMac *vm)
{
    struct DfVal z0;
    z0.type = VAL_Z;
    z0.as.z = 0;
    virmac_push(vm, &z0);
}

static void op_lz1(struct VirMac *vm)
{
    struct DfVal z1;
    z1.type = VAL_Z;
    z1.as.z = 1;
    virmac_push(vm, &z1);
}

static void op_lr0(struct VirMac *vm)
{
    struct DfVal r0;
    r0.type = VAL_R;
    r0.as.r = 0.0;
    virmac_push(vm, &r0);
}

static void op_lr1(struct VirMac *vm)
{
    struct DfVal r1;
    r1.type = VAL_R;
    r1.as.r = 1.0;
    virmac_push(vm, &r1);
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
      case VAL_R: res.as.r = lhs.as.r / rhs.as.r; break;
      default:
        err_cant_op("/", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_inv(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = val.type;
    switch (val.type) {
      case VAL_R: res.as.r = 1.0f / val.as.r; break;
      default:
        err_cant_op("unary /", val.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_ceq(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("==", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_B: res.as.b = (lhs.as.b == rhs.as.b); break;
      case VAL_C: res.as.b = (lhs.as.c == rhs.as.c); break;
      case VAL_N: res.as.b = (lhs.as.n == rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z == rhs.as.z); break;
      case VAL_O: res.as.b = object_eq(lhs.as.o, rhs.as.o); break;
      case VAL_R:
        fputs("ERROR: use an epsilon to compare R% values u idiot\n", stderr);
        return FALSE;
      default:
        err_cant_op("==", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_cne(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("~=", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_B: res.as.b = (lhs.as.b != rhs.as.b); break;
      case VAL_C: res.as.b = (lhs.as.c != rhs.as.c); break;
      case VAL_N: res.as.b = (lhs.as.n != rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z != rhs.as.z); break;
      case VAL_R:
        fputs("ERROR: use an epsilon to compare R% values u idiot\n", stderr);
        return FALSE;
      default:
        err_cant_op("~=", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_clt(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("<", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_N: res.as.b = (lhs.as.n < rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z < rhs.as.z); break;
      case VAL_R: res.as.b = (lhs.as.r < rhs.as.r); break;
      default:
        err_cant_op("<", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_cle(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types("<=", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_N: res.as.b = (lhs.as.n <= rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z <= rhs.as.z); break;
      case VAL_R: res.as.b = (lhs.as.r <= rhs.as.r); break;
      default:
        err_cant_op("<=", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_cgt(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types(">", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_N: res.as.b = (lhs.as.n > rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z > rhs.as.z); break;
      case VAL_R: res.as.b = (lhs.as.r > rhs.as.r); break;
      default:
        err_cant_op(">", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_cge(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    rhs = virmac_pop(vm);
    lhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        err_dif_types(">=", lhs.type, rhs.type);
        return FALSE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_N: res.as.b = (lhs.as.n >= rhs.as.n); break;
      case VAL_Z: res.as.b = (lhs.as.z >= rhs.as.z); break;
      case VAL_R: res.as.b = (lhs.as.r >= rhs.as.r); break;
      default:
        err_cant_op(">=", lhs.type);
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

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
