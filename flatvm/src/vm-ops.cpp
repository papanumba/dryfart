/* vm-ops.c */

//#include <cassert>

#define VM_OP(XXX, block) \
case (uint8_t) Op::XXX: { block break; }

/*#define SIMPLE_ERR(msg) do { \
    eputln(msg);            \
    return ITP_RUNTIME_ERR; \
} while (false)*/

#define READ_U8()   read_u8 (&this->ip)
#define READ_U16()  read_u16(&this->ip)

/*****************************************************************/

VM_OP(NOP,
    {}
)

// load ---------------------

VM_OP(LKS,
    auto idx = READ_U8();
    this->push(this->dat->ctn[idx]);
)

VM_OP(LKL,
    auto idx = READ_U16();
    this->push(this->dat->ctn[idx]);
)

VM_OP(LN1,
    this->push(DfVal(1u));
)

VM_OP(LR1,
    this->push(DfVal(1.0));
)

// ariþmetic ------------------

#define VM_UNIOP(name, op, t) \
VM_OP(name,                   \
    auto x = this->pop();     \
    this->push(DfVal(op x.t));\
)

VM_UNIOP(NGZ, -, z)
VM_UNIOP(NER, -, r)
VM_UNIOP(INR, 1.0 /, r)
VM_UNIOP(NOB, !, b)
VM_UNIOP(NOC, ~, c)
VM_UNIOP(NON, ~, n)

#undef VM_UNIOP

// used in binops and jcx
// stmt does sþ w/ res
#define VM_BINOP_AUX(name, op, t, stmt) \
VM_OP(name,                             \
    auto rhs = this->pop();             \
    auto lhs = this->pop();             \
    auto res = lhs.t op rhs.t;          \
    stmt \
)

#define VM_BINOP(name, op, t) \
VM_BINOP_AUX(name, op, t, this->push(DfVal(res));)

// NUM
VM_BINOP(ADC, +, c)
VM_BINOP(ADN, +, n)
VM_BINOP(ADZ, +, z)
VM_BINOP(ADR, +, r)
VM_BINOP(SUZ, -, z)
VM_BINOP(SUR, -, r)
VM_BINOP(MUC, *, c)
VM_BINOP(MUN, *, n)
VM_BINOP(MUZ, *, z)
VM_BINOP(MUR, *, r)
VM_BINOP(DIN, /, n)
VM_BINOP(DIR, /, r)

// BIT
#define VM_BITOP(T, t) \
    VM_BINOP(AN##T, &, t) \
    VM_BINOP(IO##T, |, t) \
    VM_BINOP(XO##T, ^, t)

VM_BITOP(B, b)
VM_BITOP(C, c)
VM_BITOP(N, n)

#undef VM_BITOP

// CMP
#define VM_EQUOP(T, t) \
    VM_BINOP(EQ ## T, ==, t) \
    VM_BINOP(NE ## T, !=, t)

VM_EQUOP(B, b)
VM_EQUOP(C, c)
VM_EQUOP(N, n)
VM_EQUOP(Z, z)

#undef VM_EQUOP

#define VM_ORDOP(T, t) \
    VM_BINOP(LT ## T, < , t) \
    VM_BINOP(LE ## T, <=, t) \
    VM_BINOP(GT ## T, > , t) \
    VM_BINOP(GE ## T, >=, t)

VM_ORDOP(C, c)
VM_ORDOP(N, n)
VM_ORDOP(Z, z)
VM_ORDOP(R, r)

#undef VM_ORDOP

#undef VM_BINOP

// loop dummy

VM_OP(DUM,
    this->push(DfVal());
)

// locals

VM_OP(LLS,
    uint index = READ_U8();
    this->push(this->bp[index]);
)

VM_OP(LLL,
    uint index = READ_U16();
    this->push(this->bp[index]);
)

VM_OP(SLS,
    uint index = READ_U8();
    this->bp[index] = this->pop();
)

VM_OP(SLL,
    uint index = READ_U16();
    this->bp[index] = this->pop();
)

VM_OP(ULS,
    uint index = READ_U8();
    this->bp[index] = this->peek(); // fastest, checked with sp[-1]
)

VM_OP(ULL,
    uint index = READ_U8();
    this->bp[index] = this->peek(); // fastest, checked with sp[-1]
)

// jumps

// JJ[SL]
#define OP_JJX(X, size) \
VM_OP(JJ ## X,          \
    auto dist = read_i ## size(&this->ip); \
    this->ip += dist;   \
)

OP_JJX(S, 8)
OP_JJX(L, 16)

#undef OP_JJX

// JB[TF]
#define OP_JBX(X, op) \
VM_OP(JB ## X,                 \
    auto &cond = this->peek(); \
    this->js_if(op cond.b);    \
)

OP_JBX(T, !!) // þer'sn't an Id bool op
OP_JBX(F, !)

#undef OP_JBX

// J[TF][SL]
#define OP_JXY(X, op, Y, y) \
VM_OP(J##X##Y,                  \
    auto cond = this->pop();    \
    this->j##y##_if(op cond.b); \
)

OP_JXY(T, !!, S, s)
OP_JXY(T, !!, L, l)
OP_JXY(F,  !, S, s)
OP_JXY(F,  !, L, l)

#undef OP_JXY

#define VM_EQUOP_AUX(T, t, stmt) \
    VM_BINOP_AUX(EQ ## T, ==, t, stmt) \
    VM_BINOP_AUX(NE ## T, !=, t, stmt)

#define OP_JCX(X, x, size) \
VM_OP(JC##X,                                        \
    auto cmpop = READ_U8();                         \
    switch (cmpop) {                                \
        VM_EQUOP_AUX(B, b, this->j##x##_if(res);)   \
        VM_EQUOP_AUX(C, c, this->j##x##_if(res);)   \
        VM_EQUOP_AUX(N, n, this->j##x##_if(res);)   \
        VM_EQUOP_AUX(Z, z, this->j##x##_if(res);)   \
        default: panic("unknown CMP");              \
    }                                               \
)

OP_JCX(S, s, 8)
OP_JCX(L, l, 8)

#undef VM_EQUOP

#undef OP_JCX

// stack stuff

VM_OP(DUP,
    this->push(DfVal(this->peek()));
)

VM_OP(SWP,
    // Warning: unchecked
    std::swap<DfVal>(this->sp[-1], this->sp[-2]);
)

VM_OP(ROT,
    // Warning: unchecked
    std::swap<DfVal>(this->sp[-1], this->sp[-2]);
    std::swap<DfVal>(this->sp[-2], this->sp[-3]);
)

VM_OP(POP,
    (void) this->pop();
)

VM_OP(HLT,
/*    if (this->callnum != 0) { // not halted at main
        this->print_stack();
        this->reset_stack();
        return ITP_RUNTIME_ERR;
    }*/
    auto last = this->pop();
    printf("top of stack as\nN = %d\nR = %lf\n", last.n, last.r);
    this->reset_stack();
//            garcol_do(vm);
    return ITP_OK;
)


// final
#undef VM_OP


#if 0 // mega

// J[EN][SL]
#define OP_JXY(X, op, Y, y) \
case OP_J##X##Y: {               \
    DfVal rhs = this->pop();     \
    DfVal lhs = this->pop();     \
    this->j##y##_if(rhs op lhs); \
    break;                       \
}

OP_JXY(E, ==, S, s)
OP_JXY(E, ==, L, l)
OP_JXY(N, !=, S, s)
OP_JXY(N, !=, L, l)

#undef OP_JXY

// J[LG][TE]

#define OP_J_CMP(XX, op) \
case OP_J##XX: {               \
    DfVal rhs = this->pop();   \
    DfVal lhs = this->pop();   \
    int cmp = lhs op rhs;      \
    if (cmp == DfVal::CMP_ERR) \
        ERR_BINOP(#op);        \
    else                       \
        this->jl_if(cmp);      \
    break;                     \
}

OP_J_CMP(LT, < )
OP_J_CMP(LE, <=)
OP_J_CMP(GT, > )
OP_J_CMP(GE, >=)

#undef OP_J_CMP

// array ---------------------------

case OP_AMN: {
    auto a = maitre::alloc(OBJ_ARR);
    a.as_arr()->typ = DfType::V;
    a.as_arr()->is_nat = false;
    this->push(DfVal(a));
    break;
}

#define CHECK_IS_A(x) do { \
    if (!(x).is_arr()) {            \
        fprintf(stderr, "ERROR: value of type %c%% is not an array\n", \
            (char) (x).as_type());  \
        return ITP_RUNTIME_ERR;     \
    }                               \
} while (false)

case OP_APE: {
    DfVal elem = this->pop();
    DfVal arr  = this->pop();
#ifdef SAFE
    CHECK_IS_A(arr);
    if (!arr.as.o.mut()) // must be mut
        SIMPLE_ERR("trying to push into of immutable array");
#endif
    ArrObj *a = arr.as.o.as_arr();
    if (AccRes::OK != a->push(std::move(elem)))
        SIMPLE_ERR("ERROR: some error pushing into array");
    this->push(std::move(arr));
    break;
}

case OP_AGE: {
    DfVal idx = this->pop();
    DfVal arr = this->pop();
#ifdef SAFE
    CHECK_IS_A(arr);
#endif
    uint32_t idx_n = 0;
    switch (idx.type) {
      case VAL_N: idx_n = idx.as.n; break;
      case VAL_Z:
        if (idx.as.z < 0)
            SIMPLE_ERR("ERROR: Z% index is negative");
        idx_n = (uint32_t) idx.as.z;
        break;
      default: SIMPLE_ERR("ERROR: index is not N% or Z%");
    }
    auto a = arr.as.o.as_arr();
    DfVal val;
    auto res = a->get(idx_n, val);
    if (res != AccRes::OK) {
        printf("len is %u, idx is %u\n", a->len(), idx_n);
        SIMPLE_ERR(accres_what(res));
    }
#ifdef SAFE
    if (!arr.as.o.mut()) // propagate mutability
        val.set_mut(false);
#endif
    this->push(std::move(val));
    break;
}

case OP_ASE: {
    DfVal val = this->pop(),
          idx = this->pop(),
          arr = this->pop();
#ifdef SAFE
    CHECK_IS_A(arr);
    if (!arr.as.o.mut()) // must be mut
        SIMPLE_ERR("trying to push into an immutable array");
#endif
    uint32_t idx_n = 0;
    switch (idx.type) {
      case VAL_N: idx_n = idx.as.n; break;
      case VAL_Z:
        if (idx.as.z < 0)
            SIMPLE_ERR("ERROR: Z% index is negative");
        idx_n = (uint32_t) idx.as.z;
        break;
      default: SIMPLE_ERR("ERROR: index is not N% or Z%");
    }
    auto a = arr.as.o.as_arr();
    auto res = a->set(idx_n, std::move(val));
    if (res != AccRes::OK)
        SIMPLE_ERR(accres_what(res));
    break;
}

#undef CHECK_IS_A

// table ---------------------------

case OP_TMN: {
    auto t = maitre::alloc(OBJ_TBL);
    t.as_tbl()->set(Htable());
    this->push(DfVal(t));
    break;
}

#define CHECK_IS_T(x) do { \
    if (!(x).is_tbl()) {            \
        fprintf(stderr, "ERROR: value of type %c%% is not a table\n", \
            (char) (x).as_type());  \
        return ITP_RUNTIME_ERR;     \
    }                               \
} while (false)

#define NOT_FOUND(x) do { \
    eput("field '"); idf->eprint(); eput("' not found in table\n"); \
    return ITP_RUNTIME_ERR; \
} while (false)

case OP_TSF: {
    DfVal val = this->pop();
    DfVal tbl = this->pop();
#ifdef SAFE
    CHECK_IS_T(tbl);
    if (!tbl.as.o.mut()) // must be mut
        SIMPLE_ERR("trying to set field of immutable table");
#endif
    const DfIdf *idf = &this->dat->idf[READ_U16()];
    (void) tbl.as.o.as_tbl()->set(idf, std::move(val));
    this->push(std::move(tbl));
    break;
}

case OP_TGF: {
    DfVal tbl = this->pop();
#ifdef SAFE
    CHECK_IS_T(tbl);
#endif
    const DfIdf *idf = &this->dat->idf[READ_U16()];
    DfVal val;
    if (!tbl.as.o.as_tbl()->get(idf, val))
        NOT_FOUND(idf);
#ifdef SAFE
    if (!tbl.as.o.mut()) // propagate mutability
        val.set_mut(false);
#endif
    this->push(std::move(val));
    break;
}

#undef CHECK_IS_T

// subrts ---------------------------

case OP_FMN: {
    auto f = maitre::alloc(OBJ_FUN);
    uint nor_idx = READ_U16();
    auto nor = &this->dat->pag[nor_idx];
    auto uvs = nor->uvs;
    auto uvp = this->sp - uvs; // base uv pointer
#ifdef SAFE
    TIL(i, uvs) {
        if (uvp[i].type == VAL_O)
            uvp[i].as.o.set_mut(false);
    }
#endif
    f.as_fun()->set(UsrSrt(nor, uvp));
    TIL(i, uvs) this->pop();
    this->push(DfVal(f));
    break;
}

case OP_FCL: {
    uint8_t arity = READ_U8();
    DfVal *cle = this->sp - (arity + 1); // args + callee
#ifdef SAFE
    if (!cle->is_fun())
        SIMPLE_ERR("cannot #call a not #");
#endif
    auto *fun = cle->as.o.as_fun();
    if (fun->is_nat) {
        DfVal ret;
        int res = fun->as.nat.eval(*this, this->sp - arity, arity, ret);
        if (res == 0)
            SIMPLE_ERR("some nat func err");
        this->sp -= arity;
        this->sp[-1] = ret;
        break;
    }
#ifdef SAFE
    // user funs
    if (fun->as.usr.nrs->ari != arity) {
        printf("ERROR: wrong arity calling ");
        fun->print();
        puts("");
        return ITP_RUNTIME_ERR;
    }
    // set immutable arguments
    TIL(i, arity)
        cle[1+arity].set_mut(false);
#endif // SAFE
    this->push_call(cle, fun->as.usr.nrs);
    break;
}

case OP_PMN: {
    auto p = maitre::alloc(OBJ_PRO);
    auto nor_idx = READ_U16();
    auto nor = &this->dat->pag[nor_idx];
    auto uvs = nor->uvs;
    p.as_pro()->set(UsrSrt(nor, this->sp - uvs));
    TIL(i, uvs) this->pop();
    this->push(DfVal(p));
    break;
}

case OP_PCL: {
    uint8_t arity = READ_U8();
    DfVal *cle = this->sp - (arity + 1); // args + callee
#ifdef SAFE
    if (!cle->is_pro())
        SIMPLE_ERR("cannot !call a not !");
#endif
    auto *pro = cle->as.o.as_pro();
    if (pro->is_nat) {
        int res = pro->as.nat.exec(*this, this->sp - arity, arity);
        if (res == 0)
            SIMPLE_ERR("some nat proc err");
        this->sp -= arity + 1;
        break;
    }
#ifdef SAFE
    // user procs
    if (pro->as.usr.nrs->ari != arity) {
        printf("ERROR: wrong arity calling ");
        pro->print();
        puts("");
        return ITP_RUNTIME_ERR;
    }
#endif // SAFE
    this->push_call(cle, pro->as.usr.nrs);
    break;
}

case OP_LUV: {
    assert(this->bp->type == VAL_O);
    auto bpo = this->bp->as.o;
    auto upvidx = READ_U8();
    switch (bpo.get_type()) {
      case OBJ_FUN: {
        auto bpf = bpo.as_fun();
        assert(!bpf->is_nat);
        this->push(DfVal(bpf->as.usr.upv[upvidx]));
        break;
      }
      case OBJ_PRO: {
        auto bpp = bpo.as_pro();
        assert(!bpp->is_nat);
        this->push(DfVal(bpp->as.usr.upv[upvidx]));
        break;
      }
      default:
        unreachable();
    }
    break;
}

// casts ---------------------------

case OP_CAN: {
    DfVal &val = this->peek();
    switch (val.type) {
      case VAL_N: break; // do noþing
      case VAL_Z:
        if (val.as.z < 0) // even in unsafe version
            SIMPLE_ERR("ERROR: casting negative Z% to N%");
        val = DfVal((uint32_t) val.as.z);
        break;
      default:
        SIMPLE_ERR("err cast N%");
    }
    break;
}

case OP_CAZ: {
    DfVal &val = this->peek();
    switch (val.type) {
      case VAL_N:
#ifdef SAFE
        if (val.as.n > (uint32_t) INT32_MAX)
            SIMPLE_ERR("ERROR: Overflow casting to Z");
#endif
        val = DfVal((int32_t) val.as.n); break;
      case VAL_Z: break; // do noþing
      default: SIMPLE_ERR("err cast Z%");
    }
    break;
}

case OP_CAR: {
    DfVal &val = this->peek();
    switch (val.type) {
      case VAL_N: val.as.r = (float) val.as.n; break;
      case VAL_Z: val.as.r = (float) val.as.z; break;
      case VAL_R: break; // do noþing
      default: SIMPLE_ERR("err cast R%");
    }
    val.type = VAL_R;
    break;
}

// stack stuff -----------------------

case OP_END:
    this->pop_call();
    break;

case OP_RET: {
    DfVal ret = this->pop();
    this->pop_call();
    this->push(std::move(ret));
    break;
}


#endif // mega
