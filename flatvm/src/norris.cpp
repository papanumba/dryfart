/* norris.cpp */

#include <cstring>
#include "norris.h"

Norris::Norris(
    cbyte_p      cod,
    size_t       len,
    uint32_t     lne,
    uint8_t      ari,
    uint8_t      uvs,
    const DfIdf *nam)
{
    this->lne = lne;
    this->ari = ari;
    this->uvs = uvs;
    this->nam = nam;
    if (len >= UINT32_MAX)
        panic("Buffer for Norris too big");
    this->len = (uint32_t) len;
    if (len == 0) {
        this->cod = nullptr;
        return;
    }
    this->cod = new uint8_t[len];
    std::memcpy(this->cod, cod, len);
}

Norris::Norris(Norris &&that)
{
    std::memcpy(this, &that, sizeof(Norris));
    that.cod = nullptr;
}

Norris::~Norris()
{
    if (this->cod != nullptr)
        delete [] this->cod;
}
