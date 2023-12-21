/* disasm.h */

#ifndef FLATVM_DISASM_H
#define FLATVM_DISASM_H

#include "norris.h"

void disasm_norris(struct Norris *, const char *);
uint disasm_instru(struct Norris *, uint);

#endif /* FLATVM_DISASM_H */
