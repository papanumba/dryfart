/* asterix.rs */

use std::{
    rc::Rc,
    cell::RefCell,
    fmt,
};
use crate::{util, dflib, util::MutRc};

#[derive(Debug, Clone)]
pub struct DfStr
{
    s: Vec<u8>, // ascii string
    h: u32,
}

impl DfStr
{
    pub fn as_u8s(&self) -> &[u8]
    {
        return &self.s;
    }

    pub fn as_str(&self) -> &str
    {
        unsafe {
            return std::str::from_utf8_unchecked(&self.s);
        }
    }

    fn check_ascii(s: &[u8]) -> bool
    {
        for c in s {
            if c >> 7 == 1 {
                return false;
            }
        }
        return true;
    }

    fn hash(s: &[u8]) -> u32
    {
        let mut hash: u32 = 2166136261;
        for c in s {
            hash ^= *c as u32;
            hash = hash.wrapping_mul(16777619);
        }
        return hash;
    }
}

impl TryFrom<Vec<u8>> for DfStr
{
    type Error = (); // indicates þat þe vector is not ascii
    fn try_from(s: Vec<u8>) -> Result<Self, ()>
    {
        if !Self::check_ascii(&s) {
            return Err(());
        }
        let h = Self::hash(&s);
        return Ok(Self {h:h, s:s});
    }
}

impl TryFrom<&[u8]> for DfStr
{
    type Error = (); // indicates þat þe vector is not ascii
    fn try_from(s: &[u8]) -> Result<Self, ()>
    {
        if !Self::check_ascii(s) {
            return Err(());
        }
        let h = Self::hash(s);
        return Ok(Self {h:h, s:s.to_owned()});
    }
}

impl TryFrom<&&[u8]> for DfStr
{
    type Error = (); // indicates þat þe vector is not ascii
    fn try_from(s: &&[u8]) -> Result<Self, ()>
    {
        if !Self::check_ascii(s) {
            return Err(());
        }
        let h = Self::hash(s);
        return Ok(Self {h:h, s:(*s).to_owned()});
    }
}

impl std::hash::Hash for DfStr
{
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher
    {
        self.h.hash(state);
    }
}

impl PartialEq for DfStr
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.h == other.h // faster
            && self.s == other.s;
    }
}

impl Eq for DfStr {}

impl fmt::Display for DfStr
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        unsafe {
            write!(f, "{}", std::str::from_utf8_unchecked(&self.s))
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type
{
    #[default]
    V = b'V', // void
    B = b'B', // bool
    C = b'C', // char
    N = b'N', // natural
    Z = b'Z', // zahl
    R = b'R', // real
    A = b'_', // array
    T = b'$', // table
    F = b'#', // func
    P = b'!', // proc
}

impl Type
{
    pub fn default_val(&self) -> Val
    {
        match self {
            Self::V => Val::V,
            Self::B => Val::B(false),
            Self::C => Val::C(0),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
            Self::A => Val::from_array(Array::default()),
            Self::T => Val::T(Table::new_empty()),
            Self::F |
            Self::P => panic!("cannot default subroutine"),
        }
    }
}

impl std::convert::From<&Val> for Type
{
    fn from(v: &Val) -> Self
    {
        match v {
            Val::V    => Type::V,
            Val::B(_) => Type::B,
            Val::C(_) => Type::C,
            Val::N(_) => Type::N,
            Val::Z(_) => Type::Z,
            Val::R(_) => Type::R,
            Val::A(_) => Type::A,
            Val::T(_) => Type::T,
            Val::F(_) => Type::F,
            Val::P(_) => Type::P,
        }
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}%", char::from(*self as u8))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Array
{
    #[default]
    E,          // Empty array: unknown type until a val is pushed
    B(Vec<bool>),
    C(Vec<u8>),
    N(Vec<u32>),
    Z(Vec<i32>),
    R(Vec<f32>),
}

impl Array
{
    pub fn singleton(v: &Val) -> Self
    {
        match v {
            Val::B(b) => Self::B(vec![*b]),
            Val::C(c) => Self::C(vec![*c]),
            Val::N(n) => Self::N(vec![*n]),
            Val::Z(z) => Self::Z(vec![*z]),
            Val::R(r) => Self::R(vec![*r]),
            _ => panic!("cannot create array from {:?}", v),
        }
    }

    #[inline]
    pub fn with_type(t: &Type) -> Self
    {
        return Self::with_capacity(t, 0);
    }

    // þis can be empty, but will have a concrete Type & expected cap
    #[inline]
    fn with_capacity(t: &Type, c: usize) -> Self
    {
        match t {
            Type::B => Self::B(Vec::<bool>::with_capacity(c)),
            Type::C => Self::C(Vec::<u8>  ::with_capacity(c)),
            Type::N => Self::N(Vec::<u32> ::with_capacity(c)),
            Type::Z => Self::Z(Vec::<i32> ::with_capacity(c)),
            Type::R => Self::R(Vec::<f32> ::with_capacity(c)),
            _ => todo!(),
        }
    }

    pub fn try_push(&mut self, v: &Val) -> Result<(), String>
    {
        match (&mut *self, v) {
            (Self::E, _) => *self = Self::singleton(v),
            (Self::B(a), Val::B(b)) => a.push(*b),
            (Self::C(a), Val::C(c)) => a.push(*c),
            (Self::N(a), Val::N(n)) => a.push(*n),
            (Self::Z(a), Val::Z(z)) => a.push(*z),
            (Self::R(a), Val::R(r)) => a.push(*r),
            _ => return util::format_err!(
                "cannot push {} value into {} array",
                Type::from(v), self.get_type().unwrap()
            ),
        }
        Ok(())
    }

    pub fn get_type(&self) -> Option<Type>
    {
        match self {
            Self::E => None,
            Self::B(_) => Some(Type::B),
            Self::C(_) => Some(Type::C),
            Self::N(_) => Some(Type::N),
            Self::Z(_) => Some(Type::Z),
            Self::R(_) => Some(Type::R),
        }
    }

    pub fn get(&self, i: usize) -> Option<Val>
    {
        if i >= self.len() {
            return None;
        }
        Some(match self {
            Self::E => unreachable!(),
            Self::B(a) => Val::B(a[i]),
            Self::C(a) => Val::C(a[i]),
            Self::N(a) => Val::N(a[i]),
            Self::Z(a) => Val::Z(a[i]),
            Self::R(a) => Val::R(a[i]),
        })
    }

    pub fn try_set(&mut self, i: usize, v: Val) -> Result<(), String>
    {
        if i >= self.len() {
            return util::format_err!(
                "{} out of bounds (len = {})", i, self.len()
            );
        }
        match (&mut *self, &v) {
            (Self::E, _) => unreachable!(),
            (Self::B(a), Val::B(b)) => a[i] = *b,
            (Self::C(a), Val::C(c)) => a[i] = *c,
            (Self::N(a), Val::N(n)) => a[i] = *n,
            (Self::Z(a), Val::Z(z)) => a[i] = *z,
            (Self::R(a), Val::R(r)) => a[i] = *r,
            _ => return util::format_err!(
                "{:?} is not of array's type {:?}",
                v, self.get_type().unwrap()
            )
        }
        Ok(())
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        match self {
            Self::E => 0,
            Self::B(a) => a.len(),
            Self::C(a) => a.len(),
            Self::N(a) => a.len(),
            Self::Z(a) => a.len(),
            Self::R(a) => a.len(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        self.len() == 0
    }

    pub fn len_val_n(&self) -> Val
    {
        match u32::try_from(self.len()) {
            Ok(u) => Val::N(u),
            _ => panic!("array too long to fit in u32"),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String>
    {
        if self.is_empty() {
            return Ok(other.clone());
        }
        let mut res = self.clone();
        let len = other.len();
        for i in 0..len {
            res.try_push(&other.get(i).unwrap())?;
        }
        return Ok(res);
    }
}

impl std::convert::TryFrom<&[Val]> for Array
{
    type Error = String;
    fn try_from(vals: &[Val]) -> Result<Self, Self::Error>
    {
        if vals.is_empty() {
            return Ok(Array::E);
        }
        // set array's type as þe type of þe 0st element,
        // þen try to push þe oþers & see if þey're þe same type
        let arr_type = Type::from(&vals[0]);
        let mut res = Self::with_capacity(&arr_type, vals.len());
        for v in vals {
            res.try_push(v)?;
        }
        return Ok(res);
    }
}

pub const ESC_CH: u8 = b'?';

impl std::convert::TryFrom<&[u8]> for Array
{
    type Error = &'static str;
    // s is already stript, ie it has no " arround
    fn try_from(s: &[u8]) -> Result<Self, Self::Error>
    {
        let mut res = Self::default();
        let mut i = 0;
        while i < s.len() {
            if s[i] == ESC_CH {
                // from þe lexer, we know it's followed by a char
                match s[i+1] {
                    ESC_CH => res.try_push(&Val::C(ESC_CH)),
                    b'\'' => res.try_push(&Val::C(b'\'')),
                    b'"' => res.try_push(&Val::C(b'"')),
                    b'0' => res.try_push(&Val::C(b'\0')),
                    b'N' => res.try_push(&Val::C(b'\n')),
                    b'R' => res.try_push(&Val::C(b'\r')),
                    b'T' => res.try_push(&Val::C(b'\t')),
                    _ => return Err("unknown escape char"),
                }.unwrap();
                i += 1;
            } else {
                res.try_push(&Val::C(s[i])).unwrap();
            }
            i += 1;
        }
        return Ok(res);
    }
}

impl fmt::Display for Array
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // special case for strings (C)
        if let Self::C(a) = self {
            write!(f, "{}", std::str::from_utf8(a).unwrap())?;
            return Ok(());
        }
        write!(f, "_")?;
        // 1st elem
        match self.get(0) {
            None => return write!(f, ";"),
            Some(v) => write!(f, "{v}")?,
        }
        // now all þe oþers
        match self {
            Self::E => unreachable!(),
            Self::B(a) => for b in &a[1..] {
                write!(f, ", {}", if *b {'T'} else {'F'})?;
            },
            Self::C(_) => unreachable!(),
            Self::N(a) => for n in &a[1..] { write!(f, ", {n}")?; },
            Self::Z(a) => for z in &a[1..] { write!(f, ", {z}")?; },
            Self::R(a) => for r in &a[1..] { write!(f, ", {r}")?; },
        }
        write!(f, ";")?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub enum Table
{
    Nat(dflib::tables::NatTb),
    Usr(MutRc<Vec<(Rc<DfStr>, Val)>>),
}

impl Table
{
    pub fn new_empty() -> Self
    {
        Self::Usr(Rc::new(RefCell::new(vec![])))
    }

    pub fn get(&self, k: &DfStr) -> Option<Val>
    {
        match &self {
            Self::Nat(n) => n.get(k.as_str()),
            Self::Usr(u) => {
                for p in u.borrow().iter() {
                    if &*p.0 == k {
                        return Some(p.1.clone());
                    }
                }
                return None;
            },
        }
    }

    pub fn set(&mut self, k: &Rc<DfStr>, v: Val)
    {
        match &mut *self {
            Self::Nat(_) => unreachable!("cannot set a native table"),
            Self::Usr(u) => {
                for p in &mut u.borrow_mut().iter_mut() {
                    if &p.0 == k {
                        p.1 = v;
                        return;
                    }
                }
            },
        };
    }

    pub fn has(&self, k: &DfStr) -> bool
    {
        match &self {
            Self::Nat(n) => n.get(k.as_str()).is_some(),
            Self::Usr(u) => {
                for p in u.borrow().iter() {
                    if &*p.0 == k {
                        return true;
                    }
                }
                return false;
            },
        }
    }
}

impl PartialEq for Table
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other) {
            (Self::Nat(n), Self::Nat(m)) => n == m,
            (Self::Usr(u), Self::Usr(v)) => Rc::ptr_eq(u, v),
            _ => false,
        }
    }
}

impl Eq for Table {}

impl fmt::Display for Table
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match &self {
            Self::Nat(n) => write!(f, "{}", n.name()),
            Self::Usr(u) => {
                write!(f, "$")?;
                for p in u.borrow().iter() {
                    write!(f, "{}={}.", p.0, p.1)?;
                }
                write!(f, ";") // return it
            },
        }
    }
}

// used for procs & funcs
macro_rules! impl_eq_nat_usr {
    ($tname:ident) => {

impl PartialEq for $tname {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nat(n), Self::Nat(m)) => n == m,
            (Self::Usr(u, _), Self::Usr(v, _)) => Rc::ptr_eq(u, v),
            _ => false,
        }
    }
}
impl Eq for $tname {}

    };
}

#[derive(Debug)]
pub struct SubrMeta
{
    pub line: usize, // line where it started (# xor !)
    pub name: Option<Rc<DfStr>>,
}

#[derive(Debug)]
pub struct Subr
{
    pub meta: SubrMeta,
    pub upvs: Vec<Rc<DfStr>>, // eval'd at definition
    pub pars: Vec<Rc<DfStr>>, // eval'd at call
    pub body: Block,
}

impl Subr
{
    #[inline]
    pub fn arity(&self) -> usize
    {
        self.pars.len()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubrType { F, P }

pub type UpVals = Option<Rc<Vec<Val>>>;

#[derive(Debug, Clone)]
pub enum Proc {
    Nat(dflib::procs::NatPc),
    Usr(MutRc<Subr>, UpVals),
}

impl Proc
{
    pub fn arity(&self) -> usize
    {
        match &self {
            Self::Nat(n) => n.arity(),
            Self::Usr(u, _) => u.borrow().arity(),
        }
    }
}

impl_eq_nat_usr!(Proc);

#[derive(Debug, Clone)]
pub enum Func {
    Nat(dflib::funcs::NatFn),
    Usr(MutRc<Subr>, UpVals),
}

impl Func
{
    pub fn arity(&self) -> usize
    {
        match &self {
            Self::Nat(n) => n.arity(),
            Self::Usr(u, _) => u.borrow().arity(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Val
{
    V,
    B(bool),
    C(u8),
    N(u32),
    Z(i32),
    R(f32),
    A(MutRc<Array>),
    T(Table),
    F(Func), // TODO: add upvalues
    P(Proc), // TODO: add upvalues
}

/*
** Note: Val::clone is always "shallow":
**  - for primitives (VBCNZR) it's just Copy
**  - for heap objects (ATFP) it's Rc::clone
*/

impl Val
{
    pub fn escape_char(e: u8) -> Result<u8, ()>
    {
        match e {
            b'\'' => Ok(b'\''),
            b'"' => Ok(b'"'),
            b'0' => Ok(b'\0'),
            b'N' => Ok(b'\n'),
            b'R' => Ok(b'\r'),
            b'T' => Ok(b'\t'),
            ESC_CH => Ok(ESC_CH),
            _ => Err(()),
        }
    }

    pub fn from_array(a: Array) -> Self
    {
        Self::A(Rc::new(RefCell::new(a)))
    }

    pub fn new_usr_fn(s: MutRc<Subr>, u: UpVals) -> Self
    {
        Self::F(Func::Usr(s, u))
    }

    pub fn new_usr_pc(s: MutRc<Subr>, u: UpVals) -> Self
    {
        Self::P(Proc::Usr(s, u))
    }

    pub fn new_nat_proc(np: dflib::procs::NatPc) -> Self
    {
        Self::P(Proc::Nat(np))
    }
}

impl PartialEq for Val
{
    fn eq(&self, other: &Val) -> bool
    {
        match (self, other) {
            (Val::V, Val::V) => true,
            (Val::B(b), Val::B(c)) => b == c,
            (Val::C(c), Val::C(d)) => c == d,
            (Val::N(n), Val::N(m)) => n == m,
            (Val::Z(z), Val::Z(a)) => z == a,
            (Val::F(_), Val::F(_)) => false, // FIXME Rc::ptr_eq(f, g),
            (Val::P(p), Val::P(q)) => p == q,
            (Val::A(a), Val::A(b)) => *a.borrow() == *b.borrow(),
            (Val::T(t), Val::T(r)) => t == r,
            _ => false,
        }
    }
}

impl Eq for Val {}

impl From<dflib::tables::NatTb> for Val
{
    fn from(nt: dflib::tables::NatTb) -> Val
    {
        Self::T(Table::Nat(nt))
    }
}

impl From<dflib::funcs::NatFn> for Val
{
    fn from(nf: dflib::funcs::NatFn) -> Val
    {
        Self::F(Func::Nat(nf))
    }
}

impl fmt::Display for Val
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Val::V => write!(f, "V"),
            Val::B(b) => write!(f, "{}", if *b {'T'} else {'F'}),
            Val::C(c) => write!(f, "{}", char::from(*c)),
            Val::N(n) => write!(f, "{n}u"),
            Val::Z(z) => write!(f, "{z}"),
            Val::R(r) => write!(f, "{r}"),
            Val::A(a) => write!(f, "{}", a.borrow()),
            Val::T(t) => write!(f, "{t}"),
            Val::F(_) => write!(f, "some #"),
            Val::P(_) => write!(f, "some !"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOpcode {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or, Xor, Cand, Cor,
    Idx
}

impl BinOpcode
{
    pub fn is_num(&self) -> bool
    {
        matches!(self,
            Self::Add |
            Self::Sub |
            Self::Mul |
            Self::Div |
            Self::Mod
        )
    }

    pub fn is_bit(&self) -> bool
    {
        matches!(self,
            Self::And |
            Self::Or  |
            Self::Xor
        )
    }

    pub fn is_sce(&self) -> bool
    {
        matches!(self, Self::Cand | Self::Cor)
    }

    pub fn is_cmp(&self) -> bool
    {
        matches!(self,
            Self::Eq |
            Self::Ne |
            Self::Lt |
            Self::Gt |
            Self::Le |
            Self::Ge
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UniOpcode {
    Neg, // additive negative
    Inv, // multiplicative inverse
    Not, // boolean negation
}

#[derive(Debug, Clone)]
pub enum Loop
{
    Inf(Block),
    Cdt(Block, Expr, Block),
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct IfCase
{
    pub cond: Expr,
    pub blok: Block,
}

impl IfCase
{
    pub fn new(c: Expr, b: Block) -> Self
    {
        return Self {cond:c, blok:b};
    }
}

#[derive(Debug, Clone)]
pub enum Stmt
{
    Assign(Expr, Expr),
    OperOn(Expr, BinOpcode, Expr),
    IfElse(IfCase, Vec<IfCase>, Option<Block>),
    LoopIf(Loop),
    BreakL(u32),
    Return(Expr),
    PcExit,
    PcCall(Expr, Vec<Expr>),
    TbPCal(Expr, Rc<DfStr>, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr
{
    Const(Val),
    Ident(Rc<DfStr>),
    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UniOp(Box<Expr>, UniOpcode),
    CmpOp(Box<Expr>, Vec<(BinOpcode, Expr)>),
    FnDef(MutRc<Subr>),
    Fcall(Box<Expr>, Vec<Expr>),
    TbFcl(Box<Expr>, Rc<DfStr>, Vec<Expr>), // expr#$ident#exprs,;
    RecFn,
    PcDef(MutRc<Subr>),
    RecPc,
    Array(Vec<Expr>),
    Table(Vec<(Rc<DfStr>, Expr)>),
    TblFd(Box<Expr>, Rc<DfStr>),
    RecsT(u32),
    IfExp(Vec<(Expr, Expr)>, Box<Expr>),
}
