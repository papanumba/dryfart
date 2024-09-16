// bitarr.cpp

#include "bitarr.h"

BitArr::BitArr(const BitArr &that)
{
    new (&this->_buf) DynArr<uint8_t>(that._buf);
    this->_len = that._len;
}

das_t BitArr::len() const
{
    return this->_len;
}

bool BitArr::is_empty() const
{
    return 0 == this->_len;
}

void BitArr::push(bool b)
{
    if (this->_len % 8 == 0)
        this->_buf.push(0);
    this->_len++;
    this->set(this->_len-1, b);
}

bool BitArr::pop()
{
    if (this->is_empty())
        panic("popping empty BitArr");
    this->_len--;
    return (*this)[this->_len];
}

void BitArr::set(das_t i, bool b)
{
    if (i >= this->_len)
        panic("BitArr index out of bounds");
    uint8_t &set_byte = this->_buf[i / 8];
    uint8_t bit_mask = 1 << (i % 8);
    set_byte = (set_byte & ~bit_mask) | (b ? bit_mask : 0);
}

void BitArr::extend(const BitArr &that)
{
    // TODO optimize
    TIL (i, that._len)
        this->push(that[i]);
}

bool BitArr::operator[](das_t i) const
{
    return (this->_buf[i / 8] >> (i % 8)) & 1;
}
