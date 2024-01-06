/* garcol.h */

#ifndef FLATVM_GARCOL_H
#define FLATVM_GARCOL_H

#include "common.h"

struct VirMac;

void garcol_init(void);
void garcol_exit(void);
void garcol_do(struct VirMac *);

#endif /* FLATVM_GARCOL_H */
