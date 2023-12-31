/* virmac.h */

#ifndef FLATVM_VIRMAC_H
#define FLATVM_VIRMAC_H

#include "common.h"
#include "norris.h"
#include "values.h"
#include "htable.h"

#define STACK_MAX   0x100

struct VirMac {
    struct Norris *norris;
    uchar *ip;
    struct DfVal stack[STACK_MAX];
    struct DfVal *sp;
    struct Htable globals;
};

enum ItpRes {
    ITP_OK,
    ITP_COMPILE_ERR,
    ITP_RUNTIME_ERR,
    ITP_NULLPTR_ERR
};

void          virmac_init(struct VirMac *);
void          virmac_free(struct VirMac *);
enum ItpRes   virmac_run (struct VirMac *, struct Norris *);
void          virmac_push(struct VirMac *, struct DfVal *);
struct DfVal  virmac_pop (struct VirMac *);
struct DfVal *virmac_peek(struct VirMac *);

#endif /* FLATVM_VIRMAC_H */
