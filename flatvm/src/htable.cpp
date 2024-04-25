/* htable.c */

#include <cstdio>
#include <cstring>
#include <cstdlib>
#include "htable.h"
#include "idents.h"
#include "alzhmr.h"

#define MAX_LOAD 0.75f

static Hentry * find_entry(Hentry *, size_t, const DfIdf *);

class Hentry {
  public:
    const DfIdf *k = nullptr; // key
    DfVal        v = DfVal(); // value
  public: // meþods
    Hentry() = default;
    Hentry(const DfIdf *, DfVal);
    Hentry(const Hentry &);
    bool is_empty() const;
    Hentry & operator=(const Hentry &);
};

Hentry::Hentry(const DfIdf *key, DfVal val)
{
    this->k = key;
    this->v = val;
}

Hentry::Hentry(const Hentry &that)
{
    this->k = that.k;
    this->v = that.v;
}

bool Hentry::is_empty() const
{
    return nullptr == this->k;
}

Hentry & Hentry::operator=(const Hentry &that)
{
    this->k = that.k;
    this->v = that.v;
    return *this;
}

Htable::Htable()
{
    this->ent = nullptr;
    this->siz = 0;
    this->cap = 0;
}

Htable::Htable(Htable &&that)
{
    this->ent = that.ent;
    this->siz = that.siz;
    this->cap = that.cap;
    that.ent = nullptr;
}

Htable::~Htable()
{
    if (this->ent != nullptr)
        free(this->ent);
}

// return true if found k
bool Htable::get(const DfIdf *k, DfVal &v) const
{
    if (this->siz == 0)
        return false;
    auto e = find_entry(this->ent, this->cap, k);
    if (e == nullptr)
        return false;
//    e->v.print();
//    panic("wlkj");
    v = e->v;
    return true;
}

// return true if k wasn't in þe table
bool Htable::set(const DfIdf *k, DfVal &&v)
{
    if (this->cap * MAX_LOAD < this->siz + 1.0f)
        this->grow(at_least_8(2 * this->cap));
    auto e = find_entry(this->ent, this->cap, k);
    bool is_new_key = (e->k == nullptr);
    if (is_new_key)
        this->siz++;
    new (e) Hentry(k, v);
    return is_new_key;
}

void Htable::print() const
{
    putchar('$');
    TIL(i, this->cap) {
        auto e = &this->ent[i];
        if (e->k == nullptr)
            continue;
        e->k->print();
        printf(" = ");
        e->v.print();
        putchar('.');
    }
    putchar(';');
}

static Hentry * find_entry(
    Hentry      *ent,
    size_t       cap,
    const DfIdf *key)
{
    size_t aux = (cap - 1); // bcoz cap is a 2^n number
    size_t idx = key->get_hash() & aux;
    LOOP {
        Hentry *e = &ent[idx];
        // FIXME: e->k causes uninitialized error
        auto ek = e->k;
        if (ek == key || ek == nullptr) // found slot or mt slot
            return e;
        idx = (idx + 1) & aux; // linear probing
    }
}

void Htable::grow(size_t newcap)
{
    auto newent = (Hentry *) malloc(newcap * sizeof(Hentry));
    if (newent == nullptr)
        panic("mem error");
    // init þem all to (nullptr, V)
    TIL(i, newcap)
        new (&newent[i]) Hentry();
    // redistribute all previous entries modulo newcap
    TIL(i, this->cap) {
        auto &e = this->ent[i];
        if (e.is_empty())
            continue;
        Hentry *dest = find_entry(newent, newcap, e.k);
        *dest = e;
    }
    // free old array & update t
    realloc_or_free(this->ent, 0);
    this->ent = newent;
    this->cap = newcap;
}
