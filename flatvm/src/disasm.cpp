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
static void simple_ins(const char *);
static void    ctn_ins(uint);
static void    idf_ins(const char *, uint);
static void    num_ins(const char *, uint);
static void    jmp_ins(const char *, uint);


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

#define SIMPLE(XXX) case OP_ ## XXX: simple_ins(#XXX); break;

      /* 0x0_ */
      SIMPLE(NOP)
      case OP_LVV: simple_ins("LVV"); break;
      case OP_LBT: simple_ins("LBT"); break;
      case OP_LBF: simple_ins("LBF"); break;
      case OP_LN0: simple_ins("LN0"); break;
      case OP_LN1: simple_ins("LN1"); break;
      case OP_LN2: simple_ins("LN2"); break;
      case OP_LN3: simple_ins("LN3"); break;
      case OP_LM1: simple_ins("LM1"); break;
      case OP_LZ0: simple_ins("LZ0"); break;
      case OP_LZ1: simple_ins("LZ1"); break;
      case OP_LZ2: simple_ins("LZ2"); break;
      case OP_LR0: simple_ins("LR0"); break;
      case OP_LR1: simple_ins("LR1"); break;
      case OP_LKS: ctn_ins(1); break;
      case OP_LKL: ctn_ins(2); break;

      case OP_NEG: simple_ins("NEG"); break;
      case OP_ADD: simple_ins("ADD"); break;
      case OP_SUB: simple_ins("SUB"); break;
      case OP_MUL: simple_ins("MUL"); break;
      case OP_DIV: simple_ins("DIV"); break;
      case OP_INV: simple_ins("INV"); break;
      case OP_INC: simple_ins("INC"); break;
      case OP_DEC: simple_ins("DEC"); break;
      SIMPLE(MOD)

      case OP_CEQ: simple_ins("CEQ"); break;
      case OP_CNE: simple_ins("CNE"); break;
      case OP_CLT: simple_ins("CLT"); break;
      case OP_CLE: simple_ins("CLE"); break;
      case OP_CGT: simple_ins("CGT"); break;
      case OP_CGE: simple_ins("CGE"); break;

      case OP_NOT: simple_ins("NOT"); break;
      case OP_AND: simple_ins("AND"); break;
      case OP_IOR: simple_ins("IOR"); break;

      case OP_CAN: simple_ins("CAN"); break;
      case OP_CAZ: simple_ins("CAZ"); break;
      case OP_CAR: simple_ins("CAR"); break;

      case OP_AMN: simple_ins("AMN"); break;
      case OP_APE: simple_ins("APE"); break;
      case OP_AGE: simple_ins("AGE"); break;
      case OP_ASE: simple_ins("ASE"); break;

      case OP_TMN: simple_ins("TMN"); break;
      case OP_TSF: idf_ins("TSF", 2); break;
      case OP_TGF: idf_ins("TGF", 2); break;

      case OP_PMN: num_ins("PMN", 2); break;
      case OP_PCL: num_ins("PCL", 1); break;
      case OP_FMN: num_ins("FMN", 2); break;
      case OP_FCL: num_ins("FCL", 1); break;
      case OP_LUV: num_ins("LUV", 1); break;

      case OP_LLS: num_ins("LLS", 1); break;
      case OP_SLS: num_ins("SLS", 1); break;
      case OP_ULS: num_ins("ULS", 1); break;
      case OP_LLL: num_ins("LLL", 2); break;
      case OP_SLL: num_ins("SLL", 2); break;

      /* 0x5_ */
#define JMP(op, name, size) \
    case op: jmp_ins(name, size); break;
      JMP(OP_JJS, "JJS", 1)
      JMP(OP_JJL, "JJL", 2)
      JMP(OP_JBT, "JBT", 1)
      JMP(OP_JBF, "JBF", 1)
      JMP(OP_JTS, "JTS", 1)
      JMP(OP_JTL, "JTL", 2)
      JMP(OP_JFS, "JFS", 1)
      JMP(OP_JFL, "JFL", 2)
      JMP(OP_JES, "JES", 1)
      JMP(OP_JEL, "JEL", 2)
      JMP(OP_JNS, "JNS", 1)
      JMP(OP_JNL, "JNL", 2)
      JMP(OP_JLT, "JLT", 2)
      JMP(OP_JLE, "JLE", 2)
      JMP(OP_JGT, "JGT", 2)
      JMP(OP_JGE, "JGE", 2)
#undef JMP

      SIMPLE(RET)
      case OP_END: simple_ins("END"); break;
      case OP_DUP: simple_ins("DUP"); break;
      case OP_SWP: simple_ins("SWP"); break;
      case OP_ROT: simple_ins("ROT"); break;
      case OP_POP: simple_ins("POP"); break;
      case OP_HLT: simple_ins("HLT"); break;

#undef SIMPLE

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

static void simple_ins(const char *name)
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
