/* disasm.c */

#include <cstdio>
#include <cstdlib>
#include "disasm.h"

/* local state for disassembling */
static VmData *dat = nullptr;
static Norris *nor = nullptr; /* points to dat->pag.nor */
static cbyte_p ip  = nullptr; /* points to nor->cod */

static void disasm_ins_fast();
static void set_vmdata(VmData *);
static void set_all   (VmData *, Norris *, cbyte_p );
static void one_ins(const char *);
static void ctn_ins(uint);
static void idf_ins(const char *, uint);
static void num_ins(const char *, uint);
static void jmp_ins(const char *, uint);


void disasm_vmdata(VmData *vmd, const char *name)
{
    set_vmdata(vmd);
    printf("size of Norris = %u\n", (uint) sizeof(Norris));
    printf("=== %s ===\n", name);
    size_t pag_len = dat->pag.len();
    for (size_t p = 0; p < pag_len; ++p) {
        nor = &dat->pag[p];
        if (nor->nam != nullptr) {
            printf("-------- %u-ary @ line: %u \"",
                nor->ari, nor->lne);
            nor->nam->print();
            puts("\" --------");
        } else {
            printf("-------- %u-ary @ line: %u --------\n",
                nor->ari, nor->lne);
        }
        ip = nor->cod;
        cbyte_p end = &nor->cod[nor->len];
        while (ip < end)
            disasm_ins_fast();
    }
}

void disasm_instru(VmData *vmd, Norris *code, cbyte_p rp)
{
    set_all(vmd, code, rp);
    disasm_ins_fast();
}

/* statics */

static void disasm_ins_fast()
{
    ptrdiff_t offset = ip - nor->cod;
    printf("%04td ", offset);
    uint8_t instru = read_u8(&ip);

    switch (instru) {

#define ONE(XXX) case OP_ ## XXX: one_ins(#XXX); break;
#define CTN(XXX, size) case OP_ ## XXX: ctn_ins(size); break;

      ONE(NOP)
      ONE(LVV)
      ONE(LBT)
      ONE(LBF)
      ONE(LN0)
      ONE(LN1)
      ONE(LN2)
      ONE(LN3)
      ONE(LM1)
      ONE(LZ0)
      ONE(LZ1)
      ONE(LZ2)
      ONE(LR0)
      ONE(LR1)
      CTN(LKS, 1)
      CTN(LKL, 2)

      ONE(NEG)
      ONE(ADD)
      ONE(SUB)
      ONE(MUL)
      ONE(DIV)
      ONE(INV)
      ONE(INC)
      ONE(DEC)
      ONE(MOD)

      ONE(CEQ)
      ONE(CNE)
      ONE(CLT)
      ONE(CLE)
      ONE(CGT)
      ONE(CGE)

      ONE(NOT)
      ONE(AND)
      ONE(IOR)
      ONE(XOR)

      ONE(CAN)
      ONE(CAZ)
      ONE(CAR)

      ONE(AMN)
      ONE(APE)
      ONE(AGE)
      ONE(ASE)

      case OP_TMN: one_ins("TMN"); break;
      case OP_TSF: idf_ins("TSF", 2); break;
      case OP_TGF: idf_ins("TGF", 2); break;

#define NUM(XXX, size) case OP_ ## XXX: num_ins(#XXX, size); break;

      NUM(PMN, 2)
      NUM(PCL, 1)
      NUM(FMN, 2)
      NUM(FCL, 1)
      NUM(LUV, 1)

      NUM(LLS, 1)
      NUM(SLS, 1)
      NUM(ULS, 1)
      NUM(LLL, 2)
      NUM(SLL, 2)

#undef NUM

#define JMP(XXX, size) case OP_ ## XXX: jmp_ins(#XXX, size); break;

      JMP(JJS, 1)
      JMP(JJL, 2)
      JMP(JBT, 1)
      JMP(JBF, 1)
      JMP(JTS, 1)
      JMP(JTL, 2)
      JMP(JFS, 1)
      JMP(JFL, 2)
      JMP(JES, 1)
      JMP(JEL, 2)
      JMP(JNS, 1)
      JMP(JNL, 2)
      JMP(JLT, 2)
      JMP(JLE, 2)
      JMP(JGT, 2)
      JMP(JGE, 2)
#undef JMP

      ONE(RET)
      ONE(END)
      ONE(DUP)
      ONE(SWP)
      ONE(ROT)
      ONE(POP)
      ONE(HLT)

#undef ONE

      default:
        printf("unknown opcode 0x%02x\n", instru);
    }
}

static void set_vmdata(VmData *vmd)
{
    dat = vmd;
    nor = &dat->pag[0];
    ip  = nor->cod;
}

static void set_all(VmData *d, Norris *n, cbyte_p p)
{
    dat = d;
    nor = n;
    ip  = p;
}

static void one_ins(const char *name)
{
    printf("%s\n", name);
}

static void ctn_ins(uint argsize)
{
    uint c;
    const char *name;
    switch (argsize) {
      case 1:
        c = read_u8(&ip);
        name = "LKS";
        break;
      case 2:
        c = read_u16(&ip);
        name = "LKL";
        break;
      default:
        panic("size in ctn_ins is not 1 or 2");
    }
    printf("%-8s %4d (", name, c);
    dat->ctn[c].print();
    puts(")");
}

static void idf_ins(const char *name, uint argsize)
{
    uint c;
    switch (argsize) {
      case 1: c = read_u8 (&ip); break;
      case 2: c = read_u16(&ip); break;
      default: panic("something went rrong in idf_ins");
    }
    printf("%-8s %4d (", name, c);
    dat->idf[c].print();
    puts(")");
}

static void num_ins(const char *name, uint argsize)
{
    uint c;
    switch (argsize) {
      case 1: c = read_u8 (&ip); break;
      case 2: c = read_u16(&ip); break;
      default: panic("something went rrong in num_ins");
    }
    printf("%-8s %4u\n", name, c);
}

static void jmp_ins(const char *name, uint argsize)
{
    int c;
    switch (argsize) {
      case 1: c = read_i8 (&ip); break;
      case 2: c = read_i16(&ip); break;
      default: panic("something went rrong in num_ins");
    }
    printf("%-8s %+4d\n", name, c);
}
