/* virmac.c */

#include <stdio.h>
#include <stdlib.h>
#include "common.h"
#include "virmac.h"
#include "values.h"
#include "disasm.h"

#define READ_BYTE() (*vm->ip++)

/* static functions */
static void reset_stack(struct VirMac *);
static void print_stack(struct VirMac *);
static enum ItpRes run (struct VirMac *);

/* pre-built constant loading */
static void op_lbt(struct VirMac *vm);
static void op_lbf(struct VirMac *vm);
static void op_ln0(struct VirMac *vm);
static void op_ln1(struct VirMac *vm);
static void op_lm1(struct VirMac *vm);
static void op_lz0(struct VirMac *vm);
static void op_lz1(struct VirMac *vm);

/* numeric */
static int op_neg(struct VirMac *);
static int op_add(struct VirMac *);
static int op_sub(struct VirMac *);
static int op_mul(struct VirMac *);
static int op_div(struct VirMac *);

/* comparison */
static int op_ceq(struct VirMac *);
static int op_cne(struct VirMac *);
/*static int op_clt(struct VirMac *);
static int op_cle(struct VirMac *);
static int op_cgt(struct VirMac *);
static int op_cge(struct VirMac *);*/

/* boolean & bitwise */
static int op_not(struct VirMac *);
static int op_ior(struct VirMac *);
static int op_and(struct VirMac *);


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
    if (vm == NULL)
        return ITP_NULLPTR_ERR;
    vm->norris = bc;
    vm->ip = bc->cod;
    return run(vm);
}

void virmac_push(struct VirMac *vm, struct DfVal *v)
{
    if (vm->sp == &vm->stack[STACK_MAX]) {
        fprintf(stderr, "ERROR @ virmac_push: stack overflow\n");
        exit(1);
    }
    *vm->sp = *v;
    vm->sp++;
}

struct DfVal virmac_pop(struct VirMac *vm)
{
    if (vm->sp == vm->stack) {
        fprintf(stderr, "ERROR @ virmac_pop: empty_stack\n");
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

          case OP_LBT: op_lbt(vm); break;
          case OP_LBF: op_lbf(vm); break;
          case OP_LN0: op_ln0(vm); break;
          case OP_LN1: op_ln1(vm); break;
          case OP_LM1: op_lm1(vm); break;
          case OP_LZ0: op_lz0(vm); break;
          case OP_LZ1: op_lz1(vm); break;

          case OP_NEG: if (!op_neg(vm)) return ITP_RUNTIME_ERR; break;
          case OP_ADD: if (!op_add(vm)) return ITP_RUNTIME_ERR; break;
          case OP_SUB: if (!op_sub(vm)) return ITP_RUNTIME_ERR; break;
          case OP_MUL: if (!op_mul(vm)) return ITP_RUNTIME_ERR; break;
          case OP_DIV: if (!op_div(vm)) return ITP_RUNTIME_ERR; break;

          case OP_CEQ: if (!op_ceq(vm)) return ITP_RUNTIME_ERR; break;
          case OP_CNE: if (!op_cne(vm)) return ITP_RUNTIME_ERR; break;
/*          case OP_CLT: if (!op_clt(vm)) return ITP_RUNTIME_ERR; break;
          case OP_CLE: if (!op_cle(vm)) return ITP_RUNTIME_ERR; break;
          case OP_CGT: if (!op_cgt(vm)) return ITP_RUNTIME_ERR; break;
          case OP_CGE: if (!op_cge(vm)) return ITP_RUNTIME_ERR; break;*/

          case OP_NOT: if (!op_not(vm)) return ITP_RUNTIME_ERR; break;
          case OP_IOR: if (!op_ior(vm)) return ITP_RUNTIME_ERR; break;
          case OP_AND: if (!op_and(vm)) return ITP_RUNTIME_ERR; break;

          case OP_RET:
            values_print(virmac_pop(vm));
            fputs("\n", stdout);
            return ITP_OK;
          default:
            fputs("unknown instruction\n", stderr);
        }
    }
}

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

/* most funcs */

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

static int op_neg(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = val.type;
    switch (val.type) {
      case VAL_Z:
        res.as.z = -val.as.z;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        res.as.r = -val.as.r;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: value not Z% or R% in unary - expr\n", stderr);
        return FALSE;
    }
}

static int op_add(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return TRUE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N:
        res.as.n = lhs.as.n + rhs.as.n;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_Z:
        res.as.z = lhs.as.z + rhs.as.z;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        res.as.r = lhs.as.r + rhs.as.r;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not N%, Z% or R% in + expr\n", stderr);
        return FALSE;
    }
}

static int op_sub(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return TRUE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_Z:
        res.as.z = lhs.as.z - rhs.as.z;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        res.as.r = lhs.as.r - rhs.as.r;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not Z% or R% in - expr\n", stderr);
        return FALSE;
    }
}

static int op_mul(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return TRUE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_N:
        res.as.n = lhs.as.n * rhs.as.n;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_Z:
        res.as.z = lhs.as.z * rhs.as.z;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        res.as.r = lhs.as.r * rhs.as.r;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not N%, Z% or N% in * expr\n", stderr);
        return FALSE;
    }
}

static int op_div(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return TRUE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_R:
        res.as.r = lhs.as.r / rhs.as.r;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not R% in / expr\n", stderr);
        return FALSE;
    }
}

static int op_ceq(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        fputs("ERROR: cannot compare values of different type\n", stderr);
        return TRUE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_B:
        res.as.b = (lhs.as.b == rhs.as.b);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_C:
        res.as.b = (lhs.as.c == rhs.as.c);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_N:
        res.as.b = (lhs.as.n == rhs.as.n);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_Z:
        res.as.b = (lhs.as.z == rhs.as.z);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        fputs("ERROR: use an epsilon to compare R% values u idiot\n", stderr);
        return FALSE;
      default:
        fputs("ERROR: values not B%, C%, N% or Z% in == expr\n", stderr);
        return FALSE;
    }
}

static int op_cne(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type) {
        fputs("ERROR: cannot compare values of different type\n", stderr);
        return TRUE;
    }
    res.type = VAL_B;
    switch (lhs.type) {
      case VAL_B:
        res.as.b = (lhs.as.b != rhs.as.b);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_C:
        res.as.b = (lhs.as.c != rhs.as.c);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_N:
        res.as.b = (lhs.as.n != rhs.as.n);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_Z:
        res.as.b = (lhs.as.z != rhs.as.z);
        virmac_push(vm, &res);
        return TRUE;
      case VAL_R:
        fputs("ERROR: use an epsilon to compare R% values u idiot\n", stderr);
        return FALSE;
      default:
        fputs("ERROR: values not B%, C%, N% or Z% in ~= expr\n", stderr);
        return FALSE;
    }
}

static int op_not(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = val.type;
    switch (val.type) {
      case VAL_B:
        res.as.b = !val.as.b;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_N:
        res.as.n = ~val.as.n;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: value not B% or N% in ~ expr\n", stderr);
        return FALSE;
    }
}

static int op_ior(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return FALSE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B:
        res.as.b = lhs.as.b || rhs.as.b;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_N: /* bitwise */
        res.as.n = lhs.as.n | rhs.as.n;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not B% or N% in | expr\n", stderr);
        return FALSE;
    }
}

static int op_and(struct VirMac *vm)
{
    struct DfVal lhs, rhs, res;
    lhs = virmac_pop(vm);
    rhs = virmac_pop(vm);
    if (lhs.type != rhs.type)
        return TRUE;
    res.type = lhs.type;
    switch (lhs.type) {
      case VAL_B:
        res.as.b = lhs.as.b && rhs.as.b;
        virmac_push(vm, &res);
        return TRUE;
      case VAL_N: /* bitwise */
        res.as.n = lhs.as.n & rhs.as.n;
        virmac_push(vm, &res);
        return TRUE;
      default:
        fputs("ERROR: values not B% or N% in & expr\n", stderr);
        return FALSE;
    }
}
