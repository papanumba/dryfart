/* latin1.h */

#ifndef FLATVM_LATIN1_H
#define FLATVM_LATIN1_H

#include "common.h"

#ifdef __cplusplus
extern "C" {
#endif

int  latin1_is_ascii       (uint8_t);
int  latin1_is_ascii_string(cbyte_p, size_t);
void latin1_putchar        (uint8_t);
void latin1_print          (cbyte_p, size_t);

#ifdef __cplusplus
} // extern C
#endif

#endif // FLATVM_LATIN1_H


