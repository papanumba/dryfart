/* dynarr.h */

#ifndef FLATVM_DYNARR_H
#define FLATVM_DYNARR_H

#include "common.hpp"

typedef uint32_t das_t; // Dynamic Array Size_t

template <typename T> // T must be scalar xor a move construcable class
class DynArr {
    typedef       T *  iter_t;
    typedef const T * citer_t;
  private:
    T    *_arr = nullptr;
    das_t _len = 0;
    das_t _cap = 0;
    // meþods
    void set_cap(das_t);
  public:
    void init();
    DynArr() = default;
//    DynArr(const DynArr<T> &);  // copy
    DynArr(DynArr<T> &&);       // move
    DynArr(das_t); // with reserved capacity
    ~DynArr();
    das_t len() const;
    bool is_empty() const;
    void push(T&&);
    // array stuff
     iter_t begin()       {return &this->_arr[0];}
    citer_t begin() const {return &this->_arr[0];}
     iter_t end()         {return &this->_arr[this->_len];}
    citer_t end()   const {return &this->_arr[this->_len];}
          T & operator[](das_t i)       {return this->_arr[i];}
    const T & operator[](das_t i) const {return this->_arr[i];}
//    DynArr<T> & operator=(DynArr<T> &);
    DynArr<T> & operator=(DynArr<T> &&that); // move
};

#include "dynarr.tpp"

#endif /* FLATVM_DYNARR_H */
