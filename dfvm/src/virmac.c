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

/* static functions */
static void print_stack(struct VirMac *);
static void reset_stack(struct VirMac *);
static enum ItpRes run (struct VirMac *);
void err_cant_op  (const char *, enum ValType);
void err_dif_types(const char *, enum ValType, enum ValType);
static ushort read_u16(uchar **);
static short  read_i16(uchar **);

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

/* casts */
static int op_car(struct VirMac *);
static int op_cat(struct VirMac *);

static int  op_lgl(struct VirMac *);
static int  op_sgl(struct VirMac *);
static void op_lls(struct VirMac *);
static void op_sls(struct VirMac *);
static void op_uls(struct VirMac *);

static int op_jbf(struct VirMac *);
static int op_jpf(struct VirMac *);

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
#ifdef DEBUG
/*    disasm_norris(bc, "main");*/
#endif
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

struct DfVal * virmac_peek(struct VirMac *vm)
{
    if (vm->sp == vm->stack) {
        fputs("ERROR: empty stack\n", stderr);
        exit(1);
    }
    return &vm->sp[-1];
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
            virmac_push(vm, &vm->norris->ctn.arr[read_u16(&vm->ip)]);
            break;

/* void ops */
#define DO_OP(op, fn) case op: fn(vm); break;
          DO_OP(OP_LVV, op_lvv)
          DO_OP(OP_LBT, op_lbt)
          DO_OP(OP_LBF, op_lbf)
          DO_OP(OP_LN0, op_ln0)
          DO_OP(OP_LN1, op_ln1)
          DO_OP(OP_LM1, op_lm1)
          DO_OP(OP_LZ0, op_lz0)
          DO_OP(OP_LZ1, op_lz1)
          DO_OP(OP_LR0, op_lr0)
          DO_OP(OP_LR1, op_lr1)

          DO_OP(OP_LLS, op_lls)
          DO_OP(OP_SLS, op_sls)
          DO_OP(OP_ULS, op_uls)
#undef DO_OP

/* fallible ops */
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

          DO_OP(OP_CAR, op_car)
          DO_OP(OP_CAT, op_cat)

          DO_OP(OP_LGL, op_lgl)
          DO_OP(OP_SGL, op_sgl)

          DO_OP(OP_JBF, op_jbf)
          DO_OP(OP_JPF, op_jpf)
#undef DO_OP

          case OP_JMP: {
            short dist = read_i16(&vm->ip);
            vm->ip += dist;
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
            fprintf(stderr, "unknown instruction 02x\n");
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

static ushort read_u16(uchar **ip)
{
    uchar *aux;
    uchar b0, b1;
    aux = *ip;
    b0 = *aux++;
    b1 = *aux++;
    *ip = aux;
    return (b0 << 8) | b1;
}

static short read_i16(uchar **ip)
{
    union {ushort u; short s;} aux;
    aux.u = read_u16(ip);
    return aux.s;
}


/* most funcs */

static void op_lvv(struct VirMac *vm)
{
    struct DfVal v;
    v.type = VAL_V;
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
      /*case VAL_O: res.as.b = object_eq(lhs.as.o, rhs.as.o); break;*/
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

static int op_car(struct VirMac *vm)
{
    struct DfVal val, res;
    val = virmac_pop(vm);
    res.type = VAL_R;
    switch (val.type) {
      case VAL_N: res.as.r = (float) val.as.n; break;
      case VAL_Z: res.as.r = (float) val.as.z; break;
      default:
        /*err_cast(from.type, VAL_R);*/
        printf("err cast");
        return FALSE;
    }
    virmac_push(vm, &res);
    return TRUE;
}

static int op_cat(struct VirMac *vm)
{
    struct DfVal res;
    res.type = VAL_T;
    res.as.t = virmac_pop(vm).type;
    virmac_push(vm, &res);
    return TRUE;
}

static int op_lgl(struct VirMac *vm)
{
    struct DfVal  *idf_val;
    struct ObjIdf *idf;
    struct DfVal   ret_val;
    /* its next operand will be a u16 index*/
    idf_val = &vm->norris->idf.arr[read_u16(&vm->ip)];
    if (idf_val->type != VAL_O && idf_val->as.o->type != OBJ_IDF) {
        fprintf(stderr, "ERROR: not an identifier\n");
        return FALSE;
    }
    idf = (struct ObjIdf *) idf_val->as.o;
    if (!htable_get(&vm->globals, idf, &ret_val)) { /* key not found */
        fprintf(stderr, "global identifier '%s' not found\n", idf->str);
        return FALSE;
    }
    virmac_push(vm, &ret_val);
    return TRUE;
}

static int op_sgl(struct VirMac *vm)
{
    struct DfVal  *idf_val;
    struct ObjIdf *idf;
    /* its next operand will be a u16 index*/
    idf_val = &vm->norris->idf.arr[read_u16(&vm->ip)];
    if (idf_val->type != VAL_O && idf_val->as.o->type != OBJ_IDF) {
        fprintf(stderr, "ERROR: not an identifier\n");
        return FALSE;
    }
    idf = (struct ObjIdf *) idf_val->as.o;
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
    short dist;
    struct DfVal *b = virmac_peek(vm);
    dist = read_i16(&vm->ip);
    if (b->type != VAL_B) {
        fputs("condition is not B\n", stderr);
        return FALSE;
    }
    if (!b->as.b)
        vm->ip += dist;
    return TRUE;
}

static int op_jpf(struct VirMac *vm)
{
    short dist;
    struct DfVal b = virmac_pop(vm);
    dist = read_i16(&vm->ip);
    if (b.type != VAL_B) {
        fputs("condition is not B\n", stderr);
        return FALSE;
    }
    if (!b.as.b)
        vm->ip += dist;
    return TRUE;
}
