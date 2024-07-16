/* latin1.h */

#ifndef FLATVM_LATIN1_H
#define FLATVM_LATIN1_H

#include "common.h"

#ifdef __cplusplus
extern "C" {
#endif

void latin1_puchar(uint8_t);
void latin1_print(cbyte_p, size_t);

#ifdef __cplusplus
} // extern C
#endif

#endif // FLATVM_LATIN1_H


