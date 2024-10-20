/* disasm.c */

#include <cstdio>
#include <cstdlib>
#include "disasm.h"
#include "idents.h"
#include "ser-de.h"

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
//    printf("size of Norris = %u\n", (uint) sizeof(Norris));
    printf("=== %s ===\n", name);
    size_t pag_len = dat->pag.len();
    TIL (p, pag_len) {
        nor = &dat->pag[p];
/*        if (nor->nam != nullptr) {
            printf("-------- %u-ary @ line: %u \"",
                nor->ari, nor->lne);
            nor->nam->print();
            puts("\" --------");
        } else {
            printf("-------- %u-ary @ line: %u --------\n",
                nor->ari, nor->lne);
        }*/
        puts("--------------");
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

#define ONE(XXX)       case (uint8_t) Op::XXX: one_ins(#XXX      ); break;
#define CTN(XXX, size) case (uint8_t) Op::XXX: ctn_ins(      size); break;
#define NUM(XXX, size) case (uint8_t) Op::XXX: num_ins(#XXX, size); break;
#define JMP(XXX, size) case (uint8_t) Op::XXX: jmp_ins(#XXX, size); break;

      ONE(NOP)

      CTN(LKS, 1)
      CTN(LKL, 2)

      ONE(NGZ) ONE(NER) ONE(INR) ONE(NOB) ONE(NOC) ONE(NON)

      ONE(ADC) ONE(ADN) ONE(ADZ) ONE(ADR) ONE(SUZ) ONE(SUR) ONE(MUC) ONE(MUN)
      ONE(MUZ) ONE(MUR) ONE(DIN) ONE(DIR) ONE(MOC) ONE(MON) ONE(MOZ) ONE(ANB)
      ONE(ANC) ONE(ANN) ONE(IOB) ONE(IOC) ONE(ION) ONE(XOB) ONE(XOC) ONE(XON)

      ONE(EQB) ONE(NEB) ONE(EQC) ONE(NEC) ONE(EQN) ONE(NEN) ONE(EQZ) ONE(NEZ)
      ONE(LTC) ONE(LEC) ONE(GTC) ONE(GEC) ONE(LTN) ONE(LEN) ONE(GTN) ONE(GEN)
      ONE(LTZ) ONE(LEZ) ONE(GTZ) ONE(GEZ) ONE(LTR) ONE(LER) ONE(GTR) ONE(GER)

      NUM(LLS, 1) NUM(LLL, 2)
      NUM(SLS, 1) NUM(SLL, 2)
      NUM(ULS, 1) NUM(ULL, 2)

      JMP(JJS, 1) JMP(JJL, 2)
      JMP(JBT, 1)             JMP(JBF, 1)
      JMP(JTS, 1) JMP(JTL, 2) JMP(JFS, 1) JMP(JFL, 2)

      ONE(DUP)
      ONE(SWP)
      ONE(ROT)
      ONE(POP)
      ONE(HLT)

#undef JMP
#undef NUM
#undef CTN
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
    printf("todo: print val");
//    dat->ctn[c].print();
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
