/* vm-ops.c */

#include <cassert>

#define SIMPLE_ERR(msg) do { \
    eputln(msg);            \
    return ITP_RUNTIME_ERR; \
} while (false)

#define ERR_BINOP(msg) do { \
    err_dif_types(msg, lhs.as_type(), rhs.as_type()); \
    return ITP_RUNTIME_ERR; \
} while (false)

#define ERR_OP_TYPE(msg, valref) do { \
    err_cant_op(msg, valref); \
    return ITP_RUNTIME_ERR;   \
} while (false)

#define READ_U8()   read_u8 (&this->ip)
#define READ_U16()  read_u16(&this->ip)

/*****************************************************************/

case OP_NOP:
    break;

// load ---------------------

case OP_LVV:
    this->push(DfVal());
    break;

#define OP_LXX(xx, val) \
case OP_L ## xx: this->push(DfVal((val))); break;
OP_LXX(BT, true)
OP_LXX(BF, false)

#define OP_LNX(n) OP_LXX(N ## n, (uint32_t)(n))
OP_LNX(0)
OP_LNX(1)
OP_LNX(2)
OP_LNX(3)
#undef OP_LNX

OP_LXX(M1, (int32_t)(-1))

#define OP_LZX(z) OP_LXX(Z ## z, (int32_t)(z))
OP_LZX(0)
OP_LZX(1)
OP_LZX(2)
#undef OP_LZX

#define OP_LRX(r) OP_LXX(R ## r, (float)(r))
OP_LRX(0)
OP_LRX(1)
#undef OP_LRX

case OP_LKS: {
    uint idx = READ_U8();
    this->push(DfVal(this->dat->ctn[idx]));
    break;
}

case OP_LKL: {
    uint idx = READ_U16();
    this->push(this->dat->ctn[idx]);
    break;
}

// ariþmetic ------------------

case OP_NEG: {
    DfVal val = this->pop(); // fastest (checked with peek)
    switch (val.type) {
      case VAL_Z: this->fpush(DfVal(-val.as.z)); break;
      case VAL_R: this->fpush(DfVal(-val.as.r)); break;
      default: ERR_OP_TYPE("unary -", &val);
    }
    break;
}

// only used in OP_ADD
#define ADD_O do { \
    auto lo = lhs.as.o;                     \
    auto ro = rhs.as.o;                     \
    if (lo.get_type() != ro.get_type())     \
        ERR_BINOP("+");                     \
    switch (lo.get_type()) {                \
      case OBJ_ARR: {                       \
        auto a = maitre::alloc(OBJ_ARR);  \
        a.as_arr()->typ = DfType::V;        \
        a.as_arr()->is_nat = false;         \
        auto r = lo.as_arr()->concat(       \
            *ro.as_arr(), *a.as_arr());     \
        if (AccRes::OK != r)                \
            SIMPLE_ERR(accres_what(r));          \
        this->push(DfVal(a));               \
        break;                              \
      }                                     \
      default: todo("add other objects");   \
    } \
} while (false)

#define DO_BINOP(x, op) this->push(DfVal(lhs.as.x op rhs.as.x)); break

case OP_ADD: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("+");
    switch (lhs.type) {
      case VAL_N: DO_BINOP(n, +);
      case VAL_Z: DO_BINOP(z, +);
      case VAL_R: DO_BINOP(r, +);
      case VAL_O: ADD_O; break;
      default: ERR_OP_TYPE("+", &lhs);
    }
    break;
}

#undef ADD_O

case OP_SUB: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("-");
    switch (lhs.type) {
      case VAL_Z: DO_BINOP(z, -);
      case VAL_R: DO_BINOP(r, -);
      default: ERR_OP_TYPE("-", &lhs);
    }
    break;
}

case OP_MUL: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("*");
    switch (lhs.type) {
      case VAL_N: DO_BINOP(n, *);
      case VAL_Z: DO_BINOP(z, *);
      case VAL_R: DO_BINOP(r, *);
      default: ERR_OP_TYPE("*", &lhs);
    }
    break;
}

case OP_DIV: { // fastest: checked with {pop, peek, ->} & {2pop, fpush}
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (lhs.type != rhs.type)
        ERR_BINOP("/");
    switch (lhs.type) {
//      case VAL_N: this->push(DfVal(lhs.as.n * rhs.as.n)); break;
//      case VAL_Z: this->push(DfVal(lhs.as.z * rhs.as.z)); break;
      case VAL_R: DO_BINOP(r, /);
      default: ERR_OP_TYPE("/", &lhs);
    }
    break;
}

case OP_INV: {
/* TODO: why is þis slower þan LR1 [expr] DIV ? */
    DfVal &val = this->peek();
    if (val.type != VAL_R)
        ERR_OP_TYPE("unary /", &val);
    val.as.r = 1.0f / val.as.r;
    break;
}

case OP_INC: {
    DfVal &val = this->peek();
    switch (val.type) {
        case VAL_N: val.as.n += 1; break;
        case VAL_Z: val.as.z += 1; break;
        default: ERR_OP_TYPE("1 +", &val);
    }
    break;
}

case OP_DEC: {
    DfVal &val = this->peek();
    if (val.type != VAL_Z)
        ERR_OP_TYPE("- 1", &val);
    val.as.z -= 1;
    break;
}

case OP_MOD: {
    DfVal rhs = this->pop();
    DfVal lhs = this->pop();
    if (rhs.type != VAL_N)
        ERR_BINOP("\\");
    switch (lhs.type) {
      case VAL_N: DO_BINOP(n, %);
      case VAL_Z: {
        auto rn = rhs.as.n;
        auto res = ((lhs.as.z % rn) + rn) % rn;
        this->push(DfVal((uint32_t) res));
        break;
      }
      default: ERR_BINOP("\\");
    }
    break;
}

// bool/bits ----------------------

case OP_NOT: {
    DfVal &val = this->peek();
    if (val.type != VAL_B)
        ERR_OP_TYPE("unary ~", &val);
    val.as.b = !val.as.b;
    break;
}

// applicable to B% and N%
#define BIT_BINOP(name, b_op, n_op, msg) \
case OP_ ## name: {             \
    DfVal rhs = this->pop();    \
    DfVal lhs = this->pop();    \
    if (lhs.type != rhs.type)   \
        ERR_BINOP(msg);         \
    switch (lhs.type) {         \
      case VAL_B: DO_BINOP(b, b_op);    \
      case VAL_N: DO_BINOP(n, n_op);    \
      default: ERR_OP_TYPE(msg, &lhs);  \
    }                           \
    break;                      \
}

BIT_BINOP(AND, &&, &, "&")
BIT_BINOP(IOR, ||, |, "|")
BIT_BINOP(XOR,  ^, ^, "^")

#undef BIT_BINOP

// compare ----------------------

#define OP_CEX(XX, cmpop) \
case OP_C ## XX: {                 \
    DfVal rhs = this->pop();       \
    DfVal lhs = this->pop();       \
    this->push(DfVal(lhs cmpop rhs)); \
    break;                         \
}

// overloaded operators
OP_CEX(EQ, ==)
OP_CEX(NE, !=)

#undef OP_CEX

// orderings

#define OP_CXX(XX, xx, msg) \
case OP_C ## XX: {           \
    int cmp;                 \
    DfVal rhs = this->pop(), \
          lhs = this->pop(); \
    switch ((cmp = dfval_ ## xx(&lhs, &rhs))) { \
      case CMP_ERR:          \
        ERR_BINOP(msg);      \
      default:               \
        this->push(DfVal((bool)cmp)); \
    }                        \
    break;                   \
}

OP_CXX(LT, lt, "<")
OP_CXX(LE, le, "<=")
OP_CXX(GT, gt, ">")
OP_CXX(GE, ge, ">=")

#undef OP_CXX

// locals -----------------------

case OP_LLS: {
    uint index = READ_U8();
    this->push(this->bp[index]);
    break;
}

case OP_SLS: {
    uint index = READ_U8();
    this->bp[index] = this->pop();
    break;
}

case OP_ULS: {
    uint index = READ_U8();
    this->bp[index] = this->peek(); // fastest, checked with sp[-1]
    break;
}

// jumps -----------------------------

#define CHECK_IS_B(var) do { \
    if ((var).type != VAL_B) \
        SIMPLE_ERR("condition is not B%"); \
} while (false)

// JJ[SL]

#define OP_JJX(X, size) \
case OP_JJ ## X: {      \
    auto dist = read_i ## size(&this->ip); \
    this->ip += dist;   \
    break;              \
}

OP_JJX(S, 8)
OP_JJX(L, 16)

#undef OP_JJX

// JB[TF]

#define OP_JBX(TF, op) \
case OP_JB##TF: {           \
    DfVal &b = this->peek();\
    CHECK_IS_B(b);          \
    this->js_if(op b.as.b); \
    break;                  \
}

OP_JBX(T, !!) // þer'sn't an Id bool op
OP_JBX(F, !)

#undef OP_JBX

// J[TF][SL]

#define OP_JXY(X, op, Y, y) \
case OP_J##X##Y: {              \
    DfVal b = this->pop();      \
    CHECK_IS_B(b);              \
    this->j##y##_if(op b.as.b); \
    break;                      \
}

OP_JXY(T, !!, S, s)
OP_JXY(T, !!, L, l)
OP_JXY(F,  !, S, s)
OP_JXY(F,  !, L, l)

#undef OP_JXY

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

#define OP_J_CMP(CMP, cmp, msg) \
case OP_J ## CMP: {          \
    DfVal rhs = this->pop(); \
    DfVal lhs = this->pop(); \
    int cmp = dfval_ ## cmp(&lhs, &rhs); \
    if (CMP_ERR == cmp) {    \
        ERR_BINOP(msg);      \
    } else {                 \
        this->jl_if(cmp);    \
    }                        \
    break;                   \
}

OP_J_CMP(LT, lt, ">=")
OP_J_CMP(LE, le, ">")
OP_J_CMP(GT, gt, "<=")
OP_J_CMP(GE, ge, "<")

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

case OP_DUP:
    this->push(DfVal(this->peek()));
    break;

case OP_SWP:
    // unchecked
    std::swap<DfVal>(this->sp[-1], this->sp[-2]);
    break;

case OP_ROT:
    // unchecked
    std::swap<DfVal>(this->sp[-1], this->sp[-2]);
    std::swap<DfVal>(this->sp[-2], this->sp[-3]);
    break;

case OP_POP:
    (void) this->pop();
    break;

case OP_HLT:
#ifdef DEBUG
    puts("VM HALTED");
    //this->print_calls();
#endif
    if (this->callnum != 0) { // not halted at main
        this->print_stack();
        this->reset_stack();
        return ITP_RUNTIME_ERR;
    }
    this->reset_stack();
//            garcol_do(vm);
    return ITP_OK;

