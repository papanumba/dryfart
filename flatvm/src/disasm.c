/* disasm.c */

#include <stdio.h>
#include <stdlib.h>
#include "values.h"
#include "disasm.h"
#include "alzhmr.h"

/* static functions */
static uint simple_instru(const char *, uint);
static uint    lk_instru (size_t, struct Norris *, uint);
static uint    glo_instru(const char *, struct Norris *, uint, uint);
static uint    loc_instru(const char *, struct Norris *, uint, uint);
static uint    jmp_instru(const char *, struct Norris *, uint, uint);

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
      /* 0x0_ */
      case OP_NOP: return simple_instru("NOP", offset);
      case OP_LVV: return simple_instru("LVV", offset);
      case OP_LBT: return simple_instru("LBT", offset);
      case OP_LBF: return simple_instru("LBF", offset);
      case OP_LN0: return simple_instru("LN0", offset);
      case OP_LN1: return simple_instru("LN1", offset);
      case OP_LN2: return simple_instru("LN2", offset);
      case OP_LN3: return simple_instru("LN3", offset);
      case OP_LM1: return simple_instru("LM1", offset);
      case OP_LZ0: return simple_instru("LZ0", offset);
      case OP_LZ1: return simple_instru("LZ1", offset);
      case OP_LZ2: return simple_instru("LZ2", offset);
      case OP_LR0: return simple_instru("LR0", offset);
      case OP_LR1: return simple_instru("LR1", offset);
      case OP_LKS: return lk_instru(1, code, offset);
      case OP_LKL: return lk_instru(2, code, offset);

      case OP_NEG: return simple_instru("NEG", offset);
      case OP_ADD: return simple_instru("ADD", offset);
      case OP_SUB: return simple_instru("SUB", offset);
      case OP_MUL: return simple_instru("MUL", offset);
      case OP_DIV: return simple_instru("DIV", offset);
      case OP_INV: return simple_instru("INV", offset);
      case OP_INC: return simple_instru("INC", offset);
      case OP_DEC: return simple_instru("DEC", offset);

      case OP_CEQ: return simple_instru("CEQ", offset);
      case OP_CNE: return simple_instru("CNE", offset);
      case OP_CLT: return simple_instru("CLT", offset);
      case OP_CLE: return simple_instru("CLE", offset);
      case OP_CGT: return simple_instru("CGT", offset);
      case OP_CGE: return simple_instru("CGE", offset);

      case OP_NOT: return simple_instru("NOT", offset);
      case OP_AND: return simple_instru("AND", offset);
      case OP_IOR: return simple_instru("IOR", offset);

      case OP_CAZ: return simple_instru("CAZ", offset);
      case OP_CAR: return simple_instru("CAR", offset);

      case OP_MEA: return simple_instru("MEA", offset);
      case OP_TPE: return simple_instru("TPE", offset);
      case OP_TGE: return simple_instru("TGE", offset);

      case OP_LGL: return glo_instru("LGL", code, offset, 2);
      case OP_SGL: return glo_instru("SGL", code, offset, 2);
      case OP_LLS: return loc_instru("LLS", code, offset, 1);
      case OP_SLS: return loc_instru("SLS", code, offset, 1);
      case OP_LLL: return loc_instru("LLL", code, offset, 2);
      case OP_SLL: return loc_instru("SLL", code, offset, 2);

      /* 0x5_ */
#define JMP(op, name, size) \
    case op: return jmp_instru(name, code, offset, size);
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

      case OP_RET: return simple_instru("RET", offset);
      case OP_DUP: return simple_instru("DUP", offset);
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

static uint lk_instru(size_t len, struct Norris *n, uint offset)
{
    uchar c;
    const char *name;
    offset++;
    switch (len) {
      case 1:
        c = n->cod[offset];
        name = "LKS";
        break;
      case 2:
        c = b2tohu(&n->cod[offset]);
        name = "LKL";
        break;
      default:
        panic("size in lk_instru is not 1 or 2");
    }
    printf("%-8s %4d (", name, c);
    values_print(&n->ctn.arr[c]);
    printf(")\n");
    return offset + len;
}

static uint glo_instru(
    const char *name,
    struct Norris *n,
    uint offset,
    uint argsize)
{
    uint c;
    offset++;
    switch (argsize) {
      case 1: c = n->cod[offset]; break;
      case 2: c = b2tohu(&n->cod[offset]); break;
      default:
        fputs("something went rrong in glo_instru\n", stderr);
        exit(1);
        break;
    }
    printf("%-8s %4d (", name, c);
    values_print(&n->idf.arr[c]);
    printf(")\n");
    return offset + argsize;
}

static uint loc_instru(
    const char *name,
    struct Norris *n,
    uint offset,
    uint argsize)
{
    uint c;
    offset++;
    switch (argsize) {
      case 1: c = n->cod[offset]; break;
      case 2: c = b2tohu(&n->cod[offset]); break;
      default:
        fputs("something went rrong in loc_instru\n", stderr);
        exit(1);
        break;
    }
    printf("%-8s %4d\n", name, c);
    return offset + argsize;
}

static uint jmp_instru(
    const char *name,
    struct Norris *n,
    uint offset,
    uint argsize)
{
    int c;
    offset++;
    switch (argsize) {
      case 1: c = b1toc(&n->cod[offset]); break;
      case 2: c = b2tohi(&n->cod[offset]); break;
      default:
        fputs("something went rrong in loc_instru\n", stderr);
        exit(1);
        break;
    }
    printf("%-8s %+4hi\n", name, c);
    return offset + argsize;
}
