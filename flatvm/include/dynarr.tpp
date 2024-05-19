/* dynarr.tpp */

#ifndef FLATVM_DYNARR_TPP
#define FLATVM_DYNARR_TPP

#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <utility>
#include <type_traits>
#include "common.h"

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
    this->_arr = static_cast<T *>(new_buff);
    this->_cap = new_cap;
}

/* public stuff */

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
    if (c != 0) {
        // when c is in [1, 8], set c = 8; else c stays
        this->set_cap(at_least_8(c));
    }
}

template <typename T>
DynArr<T>::~DynArr()
{
    free(this->_arr); // even if null
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
    // assign elem
    auto &new_elem = this->_arr[this->_len];
    if constexpr (std::is_scalar<T>()) {
        new_elem = elem;
    } else {
        static_assert(
            std::is_move_constructible<T>(),
            "T in DynArr<T> must be move-constructible"
        );
        new (&new_elem) T(std::move(elem));
    }
    this->_len += 1;
}

template <typename T>
T && DynArr<T>::pop()
{
    if (this->is_empty())
        panic("popping empty DynArr");
    this->_len--;
    return std::move((*this)[this->_len]);
}

template <typename T>
DynArr<T> & DynArr<T>::operator=(DynArr &&that)
{
    this->_arr = that._arr;
    this->_len = that._len;
    this->_cap = that._cap;
    that._arr = nullptr;
    return *this;
}

#endif /* FLATVM_DYNARR_TPP */
