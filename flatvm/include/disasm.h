/* disasm.h */

#ifndef FLATVM_DISASM_H
#define FLATVM_DISASM_H

#include "loader.h"

void disasm_vmdata(struct VmData *, const char *);
void disasm_instru(struct VmData *, struct Norris *, const uint8_t *);

#endif /* FLATVM_DISASM_H */
