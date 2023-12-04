/* disasm.c */

#include <stdio.h>
#include "values.h"
#include "disasm.h"
#include "alzhmr.h"

/* static functions */
static uint simple_instru(const char *, uint);
static uint    ctn_instru(const char *, struct Norris *, uint);
static uint    ctl_instru(const char *, struct Norris *, uint);
static uint    glo_instru(const char *, struct Norris *, uint);
static uint    jmp_instru(const char *, struct Norris *, uint);

void disasm_norris(struct Norris *code, const char *name)
{
    uint offset = 0;
    printf("=== %s ===\n", name);
    /*fputs("Idents: ", stdout);*/
    /*values_print(values_print(&code->idf);*/
    printf("len = %ld\n", code->len);
    while (offset < code->len)
        offset = disasm_instru(code, offset);
}

uint disasm_instru(struct Norris *code, uint offset)
{
    uchar instru;
    printf("%04d ", offset);
    instru = code->cod[offset];
    switch (instru) {
      case OP_CTN: return ctn_instru("CTN", code, offset);
      case OP_CTL: return ctl_instru("CTL", code, offset);

      case OP_LVV: return simple_instru("LVV", offset);
      case OP_LBT: return simple_instru("LBT", offset);
      case OP_LBF: return simple_instru("LBF", offset);
      case OP_LN0: return simple_instru("LN0", offset);
      case OP_LN1: return simple_instru("LN1", offset);
      case OP_LM1: return simple_instru("LM1", offset);
      case OP_LZ0: return simple_instru("LZ0", offset);
      case OP_LZ1: return simple_instru("LZ1", offset);
      case OP_LR0: return simple_instru("LR0", offset);
      case OP_LR1: return simple_instru("LR1", offset);

      case OP_NEG: return simple_instru("NEG", offset);
      case OP_ADD: return simple_instru("ADD", offset);
      case OP_SUB: return simple_instru("SUB", offset);
      case OP_MUL: return simple_instru("MUL", offset);
      case OP_DIV: return simple_instru("DIV", offset);
      case OP_INV: return simple_instru("INV", offset);

      case OP_CEQ: return simple_instru("CEQ", offset);
      case OP_CNE: return simple_instru("CNE", offset);
      case OP_CLT: return simple_instru("CLT", offset);
      case OP_CLE: return simple_instru("CLE", offset);
      case OP_CGT: return simple_instru("CGT", offset);
      case OP_CGE: return simple_instru("CGE", offset);

      case OP_NOT: return simple_instru("NOT", offset);
      case OP_AND: return simple_instru("AND", offset);
      case OP_IOR: return simple_instru("IOR", offset);

      case OP_CAT: return simple_instru("CAT", offset);

      case OP_GGL: return glo_instru("GGL", code, offset);
      case OP_SGL: return glo_instru("SGL", code, offset);

      case OP_JMP: return jmp_instru("JMP", code, offset);
      case OP_JBF: return jmp_instru("JBF", code, offset);

      case OP_RET: return simple_instru("RET", offset);
      case OP_POP: return simple_instru("POP", offset);
      case OP_HLT: return simple_instru("HLT", offset);

      default:
        printf("unknown opcode 0x%02x\n", instru);
        return offset + 1;
    }
}

static uint simple_instru(const char *name, uint offset)
{
    printf("%s\n", name);
    return offset + 1;
}

static uint ctn_instru(const char *name, struct Norris *n, uint offset)
{
    uchar c = n->cod[offset+1];
    printf("%-16s %4d (", name, c);
    values_print(&n->ctn.arr[c]);
    printf(")\n");
    return offset + 2;
}

static uint ctl_instru(const char *name, struct Norris *n, uint offset)
{
    uint c = b2tohu(&n->cod[offset+1]);
    printf("%-16s %4d (", name, c);
    values_print(&n->ctn.arr[c]);
    printf(")\n");
    return offset + 3;
}

static uint glo_instru(const char *name, struct Norris *n, uint offset)
{
    uint c = b2tohu(&n->cod[offset+1]);
    printf("%-16s %4d (", name, c);
    values_print(&n->idf.arr[c]);
    printf(")\n");
    return offset + 3;
}

static uint jmp_instru(const char *name, struct Norris *n, uint offset)
{
    short c = b2tohi(&n->cod[offset+1]);
    printf("%-16s %+4hi\n", name, c);
    return offset + 3;
}
