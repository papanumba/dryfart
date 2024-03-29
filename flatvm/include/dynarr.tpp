/* dynarr.tpp */

#ifndef FLATVM_DYNARR_TPP
#define FLATVM_DYNARR_TPP

#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <utility>
#include "alzhmr.h"

static inline das_t at_least_8(das_t c)
{
    return (c < 8 ? 8 : c);
}

template <typename T>
void DynArr<T>::init()
{
    this->_arr = nullptr;
    this->_len = 0;
    this->_cap = 0;
}

// sets capacity to max{8, new_cap} and allocates its space
template <typename T>
void DynArr<T>::set_cap(das_t new_cap)
{
    das_t new_size = new_cap * (das_t) sizeof(T);
    void *new_buff = realloc_or_free(this->_arr, new_size);
    std::memset(new_buff, 0, new_size);
    this->_arr = static_cast<T *>(new_buff);
    this->_cap = new_cap;
}

/* public stuff */

/*template <typename T>
DynArr<T>::DynArr(const DynArr<T> &that)
{
    das_t len = that._len;
    if (len == 0) {
        this->init();
        return;
    }
    this->_len = len;
    this->set_cap(at_least_8(len));
    std::memcpy(this->_arr, that.arr, this->_len * sizeof(T));
}*/

template <typename T>
DynArr<T>::DynArr(DynArr<T> &&that) :  // move
    _len(that._len),
    _cap(that._cap),
    _arr(that._arr)
{
    that.init();
}

template <typename T>
DynArr<T>::DynArr(das_t c) // with reserved capacity
{
    this->init();
    if (c != 0)
        this->set_cap(at_least_8(c));
}

template <typename T>
DynArr<T>::~DynArr()
{
    if (this->_arr != nullptr)
        free(this->_arr);
}

template <typename T>
das_t DynArr<T>::len() const
{
    return this->_len;
}

template <typename T>
bool DynArr<T>::is_empty() const
{
    return 0 == this->_len;
}

template <typename T>
void DynArr<T>::push(T &&elem)
{
    if (this->_cap < this->_len + 1)
        this->set_cap(at_least_8(2 * this->_cap));
    this->_arr[this->_len] = std::move(elem);
    this->_len += 1;
}

template <typename T>
DynArr<T> & DynArr<T>::operator=(DynArr &&that)
{
    if (this->_arr != nullptr)
        free(this->_arr);
    std::memcpy(this, &that, sizeof(DynArr<T>));
    that.init();
    return *this;
}

#endif /* FLATVM_DYNARR_TPP */
