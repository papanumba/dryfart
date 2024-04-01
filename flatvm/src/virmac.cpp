/* virmac.c */

#include <cstdio>
#include <cstdlib>
#include "virmac.h"
#include "object.h"
#include "alzhmr.h"
//#include "falloc.h"
//#include "garcol.h"

#ifdef DEBUG
#include "disasm.h"
#endif

#define READ_BYTE() (read_u8(&this->ip))
#define CMP_ERR     2

/* static functions */
void err_cant_op  (const char *, DfVal *);
void err_dif_types(const char *, DfType, DfType);

// TODO move þis to values.h
static int dfval_eq(DfVal *, DfVal *);
static int dfval_ne(DfVal *, DfVal *);
static int dfval_lt(DfVal *, DfVal *);
static int dfval_le(DfVal *, DfVal *);
static int dfval_gt(DfVal *, DfVal *);
static int dfval_ge(DfVal *, DfVal *);

#include "vm-ops.c"

VirMac::VirMac()
{
    this->reset_stack();
    this->dat = nullptr;
    this->nor = nullptr;
    //falloc_init();
    //garcol_init();
}

VirMac::~VirMac()
{
    this->reset_stack();
    //falloc_exit();
    //garcol_exit();
}

void VirMac::reset_stack()
{
    this->sp = &this->stack[0];
//    this->callnum = -1;
    this->bp = this->sp;
}

ItpRes VirMac::run(VmData *prog)
{
    if (prog == NULL)
        return ITP_NULLPTR_ERR;
    this->dat = prog;
    /* start main */
//    this->push_call(this->stack, &prog->pag.arr[0]);
    // TODO harcode run Norris &prog->pag[0]
    this->set_norris(&prog->pag[0]);
    ItpRes res = this->_run();
    if (res != ITP_OK)
        printf("wlethiwlht");
        //this->print_calls();
    return res;
}

void VirMac::push(const DfVal &v)
{
#ifdef SAFE
    if (this->sp == &this->stack[STACK_MAX])
        panic("ERROR: stack overflow");
#endif /* SAFE */
    *this->sp = v;
    this->sp++;
}

void VirMac::push(DfVal &&v)
{
#ifdef SAFE
    if (this->sp == &this->stack[STACK_MAX])
        panic("ERROR: stack overflow");
#endif /* SAFE */
    *this->sp = std::move(v);
    this->sp++;
}

void VirMac::fpush(DfVal &&v) // fast push
{
    *this->sp++ = std::move(v);
}

DfVal && VirMac::pop()
{
#ifdef SAFE
    if (this->sp == this->bp)
        panic("ERROR: empty stack\n");
#endif /* SAFE */
    this->sp--;
    return std::move(*this->sp);
}

DfVal & VirMac::peek()
{
#ifdef SAFE
    if (this->sp == this->bp)
        panic("ERROR: empty stack");
#endif /* SAFE */
    return this->sp[-1];
}

ItpRes VirMac::_run()
{
    while (true) {
        uint8_t ins;
#ifdef DEBUG
        this->print_stack();
        disasm_instru(this->dat, this->nor, this->ip);
#endif /* DEBUG */
        switch (ins = READ_BYTE()) {
          case OP_NOP: break;


/* void ops */
#define DO_OP(op, fn) case op: fn(this); break;
          DO_OP(OP_LVV, op_lvv)

          DO_OP(OP_LLS, op_lls)
          DO_OP(OP_SLS, op_sls)
          DO_OP(OP_ULS, op_uls)

//          DO_OP(OP_CEQ, op_ceq)
//          DO_OP(OP_CNE, op_cne)
#undef DO_OP

/* consts */
#define DO_L(op, fn, val) case op: fn(this, val); break;
          DO_L(OP_LBT, op_lb, true)
          DO_L(OP_LBF, op_lb, false)
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
            this->push(DfVal(this->dat->ctn[idx]));
            break;
          }
          case OP_LKL: {
            uint idx = read_u16(&this->ip);
            this->push(this->dat->ctn[idx]);
            break;
          }

/* fallible ops */
#define DO_OP(op, fn) case op: fn(this); break;
          DO_OP(OP_NEG, op_neg)
          DO_OP(OP_ADD, op_add)

          DO_OP(OP_SUB, op_sub)
          DO_OP(OP_MUL, op_mul)
          DO_OP(OP_DIV, op_div)
          DO_OP(OP_INV, op_inv)
          DO_OP(OP_INC, op_inc)
#ifdef GRANMERDA
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
#endif
          DO_OP(OP_CAR, op_car)
#if 0
          DO_OP(OP_APE, op_ape)
          DO_OP(OP_AGE, op_age)
          DO_OP(OP_ASE, op_ase)

          DO_OP(OP_TSF, op_tsf)
          DO_OP(OP_TGF, op_tgf)

          DO_OP(OP_PCL, op_pcl)
          DO_OP(OP_FCL, op_fcl)
          DO_OP(OP_RET, op_ret)
#endif

          DO_OP(OP_JBF, op_jbf)
          DO_OP(OP_JFS, op_jfs)
          DO_OP(OP_JFL, op_jfl)
          DO_OP(OP_JLT, op_jlt)
          DO_OP(OP_JLE, op_jle)
          DO_OP(OP_JGT, op_jgt)
          DO_OP(OP_JGE, op_jge)
#undef DO_OP

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

#if 0
          case OP_AMN: {
            DfVal val;
            val.type = VAL_O;
            val.as.o = (struct Object *) objarr_new();
            virmac_push(vm, &val);
            break;
          }
          case OP_TMN: {
            DfVal val;
            val.type = VAL_O;
            val.as.o = (struct Object *) objtbl_new();
            virmac_push(vm, &val);
            break;
          }

          case OP_PMN: {
            DfVal val;
            uint idx = read_u16(&this->ip);
            val.type = VAL_O;
            val.as.o = (void *) objpro_new(&this->dat->pag.arr[idx]);
            virmac_push(vm, &val);
            break;
          }

          case OP_FMN: {
            DfVal val;
            uint idx = read_u16(&this->ip);
            val.type = VAL_O;
            val.as.o = (void *) objfun_new(&this->dat->pag.arr[idx]);
            virmac_push(vm, &val);
            break;
          }

          case OP_END: {
            if (!pop_call(vm)) return ITP_RUNTIME_ERR;
            break;
          }
#endif // GRANMERDA

          case OP_DUP: {
            this->push(DfVal(this->peek()));
            break;
          }
          case OP_POP: (void) this->pop(); break;
          case OP_HLT:
#ifdef DEBUG
            puts("VM HALTED");
            //this->print_calls();
            this->print_stack();
#endif
            this->reset_stack();
//            garcol_do(vm);
            return ITP_OK;

          default:
            fprintf(stderr, "unknown instruction %02x\n", ins);
            return ITP_RUNTIME_ERR;
        }
    }
}

#ifdef DEBUG
void VirMac::print_stack() const
{
    const DfVal *slot = nullptr;
    for (slot = &this->stack[0];
         slot != this->sp;
         slot++) {
        printf("[%c%%", (char) slot->as_type());
        slot->print();
        printf("]");
    }
    printf("\n");
}
#endif /* DEBUG */

#ifdef GRANMERDA
static int push_call(VirMac *vm, DfVal *c, Norris *n)
{
#ifdef SAFE
    if (this->callnum == CALLS_MAX) {
        eputln("ERROR: call stack overflow");
        return FALSE;
    }
#endif /* SAFE */
#ifdef DEBUG
    if (c != NULL) {
        printf("calling "); values_print(c); printf("------------\n");
    }
#endif /* DEBUG */
    this->calls[this->callnum] = this->bp;
    this->norrs[this->callnum] = this->nor;
    this->ips  [this->callnum] = this->ip;
    this->callnum++;
    this->bp = c;
    set_norris(vm, n);
    return TRUE;
}

static int pop_call(VirMac *vm)
{
#ifdef SAFE
    if (this->callnum == -1) {
        eputln("ERROR: empty call stack\n");
        return FALSE;
    }
#endif /* SAFE */
#ifdef DEBUG
    printf("end call "); values_print(this->bp); printf("------------\n");
#endif /* DEBUG */
    this->callnum--;
    this->sp = this->bp;
    this->bp  = this->calls[this->callnum];
    this->ip  = this->ips  [this->callnum];
    this->nor = this->norrs[this->callnum];
    return TRUE;
}

static void print_calls(const VirMac *vm)
{
    puts("Call stack (top oldest):\n    !main");
    int last = this->callnum;
#define BASURA(vp) MACRO_STMT(printf("    "); values_print(vp); puts("");)
    for (int i = 1; i < last; ++i)
        BASURA(this->calls[i]);
    if (last > 0)
        BASURA(this->bp);
    puts("");
#undef BASURA
}

#endif // GRANMERDA

void VirMac::set_norris(Norris *n)
{
    this->nor = n;
    this->ip = n->cod;
}

/* error message for same type but invalid operations */
void err_cant_op(const char *op, DfVal *v)
{
    char ty = (char) v->as_type();
    fprintf(stderr, "ERROR: Cannot operate %s with %c value(s)\n", op, ty);
}

void err_dif_types(const char *op, DfType t1, DfType t2)
{
    fprintf(stderr, "ERROR: Cannot operate %s with types %c and %c\n",
        op, (char)t1, (char)t2);
}

//#define ERR_BINOP(msg)  err_dif_types(msg, val2type(&lhs), val2type(&rhs))

/* see C99's §6.5.8 Relational Operators ¶6 */

#define DFVAL_CMP_FN(name, cmpop) \
static int name(DfVal *lhs, DfVal *rhs) \
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

#define VM_JX_IF(x, read_size, adv_size) \
void VirMac::j ## x ## _if(bool cond) \
{                                                   \
    if (cond) {                                     \
        int dist = read_i ## read_size (&this->ip); \
        this->ip += dist;                           \
    } else {                                        \
        this->ip += adv_size;                       \
    }                                               \
}

VM_JX_IF(s,  8, 1) // short
VM_JX_IF(l, 16, 2) // long

#undef VM_JX_IF
