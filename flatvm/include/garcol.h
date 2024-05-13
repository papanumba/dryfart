/* garcol.h */

#ifndef FLATVM_GARCOL_H
#define FLATVM_GARCOL_H

class VirMac;
class DfVal;
class ObjRef;

namespace garcol
{
    void do_it(VirMac *);
};

void mark_dfval(DfVal &);
void mark_object(ObjRef &);

#endif /* FLATVM_GARCOL_H */
