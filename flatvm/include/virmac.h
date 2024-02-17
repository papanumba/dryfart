/* virmac.h */

#ifndef FLATVM_VIRMAC_H
#define FLATVM_VIRMAC_H

#include "common.h"
#include "loader.h"
#include "norris.h"
#include "values.h"

#define STACK_MAX   0x200
#define CALLS_MAX   0x100

struct VirMac {
    struct VmData *dat;         /* not ownt */
    struct Norris *nor;         /* curr exec norris */
    const uint8_t *ip;          /* ip to þe nor */
    struct DfVal   stack[STACK_MAX];
    struct DfVal  *sp;          /* stack pointer */
    struct DfVal  *calls[CALLS_MAX];
    struct Norris *norrs[CALLS_MAX];
    const uint8_t *ips  [CALLS_MAX];
    int            callnum;     /* call top index */
    struct DfVal  *bp;          /* base pointer = calls[call_num] */
};

enum ItpRes {
    ITP_OK = 0,
    ITP_COMPILE_ERR,
    ITP_RUNTIME_ERR,
    ITP_NULLPTR_ERR
};

void          virmac_init(struct VirMac *);
void          virmac_free(struct VirMac *);
enum ItpRes   virmac_run (struct VirMac *, struct VmData *);
void          virmac_push(struct VirMac *, struct DfVal *);
struct DfVal  virmac_pop (struct VirMac *);
struct DfVal *virmac_peek(struct VirMac *);

#endif /* FLATVM_VIRMAC_H */
