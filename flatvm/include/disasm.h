/* disasm.h */

#ifndef FLATVM_DISASM_H
#define FLATVM_DISASM_H

#include "loader.h"

void disasm_vmdata(VmData *, const char *);
void disasm_instru(VmData *, Norris *, cbyte_p);

#endif /* FLATVM_DISASM_H */
