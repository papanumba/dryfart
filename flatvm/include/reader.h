/* reader.h */

#ifndef FLATVM_READER_H
#define FLATVM_READER_H

#ifdef __cplusplus
#include "common.h"
#else
#include "common.h"
#endif // C++

#ifdef __cplusplus
extern "C" {
#endif // C++

struct Reader {
    cbyte_p buf;
    size_t  len;
};

enum ReadRes {
    READRES_OK,
    READRES_ENULL,
    READRES_EOPEN,
    READRES_EMMAP,
};

enum ReadRes reader_open(const char *, struct Reader *);
enum ReadRes reader_free(struct Reader *);
const char * readres_what(enum ReadRes);

#ifdef __cplusplus
} // extern C
#endif // C++

#endif // FLATVM_READER_H
