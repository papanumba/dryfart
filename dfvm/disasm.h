/* disasm.h */

#ifndef DFVM_DISASM_H
#define DFVM_DISASM_H

#include "norris.h"

void disasm_norris(struct Norris *, const char *);
uint disasm_instru(struct Norris *, uint);

#endif /* DFVM_DISASM_H */
