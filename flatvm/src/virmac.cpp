/* virmac.c */

#include <cstdio>
#include <cstdlib>
#include <cassert>
#include "virmac.h"
#include "maitre.h"
#include "object.h"
#include "idents.h"

#ifdef DEBUG
#include "disasm.h"
#endif

#define READ_BYTE() (read_u8(&this->ip))

/* static functions */
void err_cant_op  (const char *, DfVal *);
void err_dif_types(const char *, DfType, DfType);

VirMac::VirMac()
{
    this->reset_stack();
    this->dat = nullptr;
    this->nor = nullptr;
}

VirMac::~VirMac()
{
    this->reset_stack();
}

void VirMac::reset_stack()
{
    this->sp = &this->stack[0];
    this->callnum = -1;
    this->bp = this->sp;
}

ItpRes VirMac::run(VmData *prog)
{
    if (prog == NULL)
        return ITP_NULLPTR_ERR;
    this->dat = prog;
    /* start main */
    this->push_call(&this->stack[0], &prog->pag[0]);
    ItpRes res = this->_run();
    if (res != ITP_OK) {
//        this->print_stack();
        this->print_calls();
    }
    return res;
}

void VirMac::push(const DfVal &v)
{
#ifdef SAFE
    if (this->sp == &this->stack[STACK_MAX])
        panic("ERROR: stack overflow");
#endif
    *this->sp = v;
    this->sp++;
}

void VirMac::push(DfVal &&v)
{
#ifdef SAFE
    if (this->sp == &this->stack[STACK_MAX])
        panic("ERROR: stack overflow");
#endif
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
#endif
    this->sp--;
    return std::move(*this->sp);
}

DfVal & VirMac::peek()
{
#ifdef SAFE
    if (this->sp == this->bp)
        panic("ERROR: empty stack");
#endif
    return this->sp[-1];
}

ItpRes VirMac::_run()
{
    LOOP {
        uint8_t ins;
#ifdef DEBUG
        this->print_stack();
        disasm_instru(this->dat, this->nor, this->ip);
#endif /* DEBUG */
        switch (ins = READ_BYTE()) {
#include "vm-ops.cpp"
          default:
            fprintf(stderr, "unknown instruction %02x\n", ins);
            return ITP_RUNTIME_ERR;
        }
    }
}

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

void VirMac::push_call(DfVal *c, Norris *n)
{
#ifdef SAFE
    if (this->callnum == CALLS_MAX)
        panic("ERROR: call stack overflow");
#endif
    this->callnum++;
    new (&this->calls[this->callnum]) Record(this->bp, this->nor, this->ip);
    this->bp = c;
    this->set_norris(n);
}

void VirMac::pop_call()
{
#ifdef SAFE
    if (this->callnum == -1)
        panic("ERROR: empty call stack\n");
#endif /* SAFE */
    this->sp = this->bp;
    auto &r = this->calls[this->callnum];
    this->bp  = r.bps;
    this->ip  = r.ips;
    this->nor = r.nor;
    this->callnum--;
}

void VirMac::print_calls() const
{
    puts("Call stack (top oldest):\n    !main");
    int last = this->callnum;
#define BASURA(vp) \
    do {printf("    %c", (char) (vp)->as_type()); \
        (vp)->print(); puts(""); \
    } while(0)
    for (int i = 1; i < last; ++i)
        BASURA(this->calls[i].bps);
    if (last > 0)
        BASURA(this->bp);
    puts("");
#undef BASURA
}

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
