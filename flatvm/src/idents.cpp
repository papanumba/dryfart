/* idents.cpp */

#include <cstdio>
#include <cstring>
#include "idents.h"

static uint32_t hash_buff(cbyte_p, size_t);

/* str mayn't be NUL-term'd, but idf.str will */
DfIdf::DfIdf(const uint8_t *str, size_t len)
{
    this->str = new uint8_t[len+1];
    std::memcpy(this->str, str, len);
    this->str[len] = 0; // NUL-term for printing
    this->len = len;
    this->hsh = hash_buff(this->str, len);
}

DfIdf::DfIdf(DfIdf &&that)
{
    this->str = that.str;
    this->len = that.len;
    that.str = nullptr;
}

DfIdf::~DfIdf()
{
    if (this->str != nullptr)
        delete [] this->str;
}

uint32_t DfIdf::get_hash() const
{
    return this->hsh;
}

size_t DfIdf::get_len() const
{
    return this->len;
}

void DfIdf::print() const
{
    printf("%s", (char *) this->str);
}

void DfIdf::eprint() const
{
    fprintf(stderr, "%s", (char *) this->str);
}

bool DfIdf::eq(const char *str) const
{
    return bytearr_cmp(this->str, (cbyte_p) str, this->len);
}

DfIdf & DfIdf::operator=(DfIdf &&that)
{
    this->~DfIdf();
    new (this) DfIdf(std::move(that));
    return *this;
}

// FNV-1a (Fowler-Noll-Vo) hash function for 32 bit
static uint32_t hash_buff(cbyte_p buf, size_t len)
{
    uint32_t hash = 2166136261;
    TIL(i, len) {
        hash ^= buf[i];
        hash *= 16777619;
    }
    return hash;
}
