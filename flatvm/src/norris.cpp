/* norris.cpp */

#include <cstring>
#include "norris.h"

Norris::Norris(
    const Slice<uint8_t> &cod,
    uint32_t              lne,
    uint8_t               ari,
    const DfIdf          *nam)
{
    this->lne = lne;
    this->ari = ari;
    this->nam = nam;
    size_t len = cod.len;
    if (len >= UINT32_MAX)
        panic("Buffer for Norris too big");
    this->len = (uint32_t) len;
    if (len == 0) {
        this->cod = nullptr;
        return;
    }
    this->cod = new uint8_t[len];
    std::memcpy(this->cod, cod.buf, len);
}

Norris::Norris(Norris &&that)
{
    std::memcpy(this, &that, sizeof(Norris));
    that.cod = nullptr;
}

Norris & Norris::operator=(Norris &&that)
{
    if (this->cod != nullptr)
        delete [] this->cod;
    std::memcpy(this, &that, sizeof(Norris));
    std::memset(&that, 0, sizeof(Norris));
    return *this;
}

Norris::~Norris()
{
    if (this->cod != nullptr)
        delete [] this->cod;
}
