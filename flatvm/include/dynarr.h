/* dynarr.h */

#ifndef FLATVM_DYNARR_H
#define FLATVM_DYNARR_H

#include "common.h"

typedef uint32_t das_t; // Dynamic Array Size_T

// T must be scalar xor a move construtible
template <typename T>
class DynArr {
    typedef       T *  iter_t;
    typedef const T * citer_t;
  private:
    T    *_arr = nullptr;
    das_t _len = 0;
    das_t _cap = 0;
    // meþods
    void set_cap(das_t);
    void init();
  public:
    DynArr() = default;
    DynArr(DynArr<T> &&);
    DynArr(das_t); // wiþ reserved capacity
    ~DynArr();
    // getters
    das_t len() const;
    bool is_empty() const;
    // modifiers
    void push(T &&);
    T && pop();
    // practical array stuff
     iter_t begin()       {return &this->_arr[0];}
    citer_t begin() const {return &this->_arr[0];}
     iter_t end()         {return &this->_arr[this->_len];}
    citer_t end()   const {return &this->_arr[this->_len];}
          T & operator[](das_t i)       {return this->_arr[i];}
    const T & operator[](das_t i) const {return this->_arr[i];}
    DynArr<T> & operator=(DynArr<T> &&that);
};

// implementation
#include "dynarr.tpp"

#endif /* FLATVM_DYNARR_H */
