/* src/asterix.rs */

use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};
use crate::{util, dflib, util::MutRc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type
{
    V, // void
    B, // bool
    C, // char
    N, // natural
    Z, // zahl
    R, // real
    F, // func
    P, // proc
    A, // array
    T, // table
}

impl Type
{
    pub fn is_num(&self) -> bool
    {
        match self {
            Self::N | Self::Z | Self::R => true,
            _ => false,
        }
    }

    pub fn is_copy(&self) -> bool
    {
        match self {
            Self::V |
            Self::B |
            Self::C |
            Self::N |
            Self::Z |
            Self::R => true,
            _ => false,
        }
    }

    pub fn default_val(&self) -> Val
    {
        match self {
            Self::V => Val::V,
            Self::B => Val::B(false),
            Self::C => Val::C(0),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
            Self::F => panic!("cannot default function"),
            Self::P => panic!("cannot default procedure"),
            Self::A => Val::from_array(Array::default()),
            Self::T => Val::T(Table::new()),
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
            Val::F(_) => Type::F,
            Val::P(_) => Type::P,
            Val::A(_) => Type::A,
            Val::T(_) => Type::T,
        }
    }
}

impl std::fmt::Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Self::V => write!(f, "V%"),
            Self::B => write!(f, "B%"),
            Self::C => write!(f, "C%"),
            Self::N => write!(f, "N%"),
            Self::Z => write!(f, "Z%"),
            Self::R => write!(f, "R%"),
            Self::F => write!(f, "#%"),
            Self::P => write!(f, "!%"),
            Self::A => write!(f, "_%"),
            Self::T => write!(f, "$%"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array
{
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

    fn try_push(&mut self, v: &Val) -> Result<(), String>
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

    pub fn dim(&self) -> usize
    {
        return 1; // TODO: multidim arrs
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

    pub fn len_val_n(&self) -> Val
    {
        if let Ok(u) = u32::try_from(self.len()) {
            return Val::N(u);
        } else {
            panic!("array too long to fit in u32");
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String>
    {
        if self.len() == 0 {
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

impl Default for Array {
    fn default() -> Self { Self::E }
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
            res.try_push(&v)?;
        }
        return Ok(res);
    }
}

impl std::convert::TryFrom<&[u8]> for Array
{
    type Error = &'static str;
    // s is already stript, ie it has no " arround
    fn try_from(s: &[u8]) -> Result<Self, Self::Error>
    {
        let mut res = Self::default();
        let mut i = 0;
        while i < s.len() {
            if s[i] == b'`' {
                // from þe lexer, we know it's followed by a char
                match s[i+1] {
                    b'`' => res.try_push(&Val::C(b'`')),
                    b'"' => res.try_push(&Val::C(b'"')),
                    b'0' => res.try_push(&Val::C(b'\0')),
                    b'N' => res.try_push(&Val::C(b'\n')),
                    b'R' => res.try_push(&Val::C(b'\r')),
                    b'T' => res.try_push(&Val::C(b'\t')),
                    b'x' => todo!("hex escapes"),
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

impl std::fmt::Display for Array
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // special case for strings (C)
        if let Self::C(a) = self {
            write!(f, "{}", std::str::from_utf8(a).unwrap())?;
            return Ok(());
        }
        write!(f, "_")?;
        // TODO: do not print tailing comma?
        match self {
            Self::E => {}, // empty
            Self::B(a) => for b in a {
                if *b {write!(f, "T, ",)?;}
                else  {write!(f, "F, ",)?;}
            },
            Self::C(_) => unreachable!(),
            Self::N(a) => for n in a { write!(f, "{n}, ")?; },
            Self::Z(a) => for z in a { write!(f, "{z}, ")?; },
            Self::R(a) => for r in a { write!(f, "{r}, ")?; },
        }
        write!(f, ";")?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub enum Table
{
    Nat(dflib::tables::NatTb),
    Usr(Rc<RefCell<HashMap<String, Val>>>),
}

impl Table
{
    pub fn new() -> Self
    {
        Self::Usr(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn get(&self, k: &str) -> Option<Val>
    {
        match &self {
            Self::Nat(n) => n.get(k),
            Self::Usr(u) => u.borrow().get(k).cloned(),
        }
    }

    pub fn set(&mut self, k: String, v: Val)
    {
        match &mut *self {
            Self::Nat(_) => unreachable!("cannot set a native table"),
            Self::Usr(u) => u.borrow_mut().insert(k, v),
        };
    }

    pub fn has(&self, k: &str) -> bool
    {
        match &self {
            Self::Nat(n) => n.get(k).is_some(),
            Self::Usr(u) => u.borrow().contains_key(k),
        }
    }
}

// used for tables, procs, funcs
macro_rules! impl_eq_nat_usr {
    ($tname:ident) => {

impl PartialEq for $tname {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nat(n), Self::Nat(m)) => n == m,
            (Self::Usr(u), Self::Usr(v)) => Rc::ptr_eq(u, v),
            _ => false,
        }
    }
}
impl Eq for $tname {}

    };
}

impl_eq_nat_usr!(Table);

#[derive(Debug)]
pub struct SubrMeta
{
    pub line: usize, // line where it started (# xor !)
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Subr
{
    pub meta: SubrMeta,
    pub pars: Vec<String>,
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

#[derive(Debug, Clone)]
pub enum Proc {
    Nat(dflib::procs::NatPc),
    Usr(Rc<Subr>),
}

impl Proc
{
    pub fn arity(&self) -> usize
    {
        match &self {
            Self::Nat(n) => n.arity(),
            Self::Usr(u) => u.arity(),
        }
    }
}

impl_eq_nat_usr!(Proc);

#[derive(Debug, Clone)]
pub enum Func {
//    Nat(dflib::NatFn),
    Usr(Rc<Subr>),
}

impl Func
{
    pub fn arity(&self) -> usize
    {
        match &self {
//            Self::Nat(n) => n.arity(),
            Self::Usr(u) => u.arity(),
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
    F(Func), // TODO: add upvalues
    P(Proc), // TODO: add upvalues
    A(MutRc<Array>),
    T(Table),
}

/*
** Note: Val clone is always "shallow":
**  - for primitives (VBCNZR) it's just Copy
**  - for heap objects (FPAT) it's Rc::clone
*/

impl Val
{
    pub fn from_array(a: Array) -> Self
    {
        Self::A(Rc::new(RefCell::new(a)))
    }

    pub fn new_nat_tb(n: &'static str) -> Self
    {
        Self::T(Table::Nat(dflib::tables::NatTb::new(n)))
    }

    pub fn new_usr_fn(s: Rc<Subr>) -> Self
    {
        Self::F(Func::Usr(s))
    }

    pub fn new_usr_pc(s: Rc<Subr>) -> Self
    {
        Self::P(Proc::Usr(s))
    }

    pub fn new_nat_proc(n: &'static str) -> Self
    {
        Self::P(Proc::Nat(dflib::procs::NatPc::new(n)))
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
            (Val::F(_), Val::F(_)) => false,//Rc::ptr_eq(f, g),
            (Val::P(p), Val::P(q)) => p == q,
            (Val::A(a), Val::A(b)) => *a.borrow() == *b.borrow(),
            (Val::T(t), Val::T(r)) => t == r,
            _ => false,
        }
    }
}

impl Eq for Val {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOpcode {
    Add, Sub, Mul, Div,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or, Xor, Cand, Cor,
    Idx
}

impl BinOpcode
{
    pub fn from_str(s: &str) -> Self
    {
        return match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "==" => Self::Eq,
            "~=" => Self::Ne,
            "<"  => Self::Lt,
            ">"  => Self::Gt,
            "<=" => Self::Le,
            ">=" => Self::Ge,
            "&"  => Self::And,
            "|"  => Self::Or,
            "&&" => Self::Cand,
            "||" => Self::Cor,
            _ => panic!("unknown binop"),
        }
    }

    pub fn is_num(&self) -> bool
    {
        match self {
            Self::Add |
            Self::Sub |
            Self::Mul |
            Self::Div => true,
            _ => false,
        }
    }

    pub fn is_bit(&self) -> bool
    {
        match self {
            Self::And |
            Self::Or  |
            Self::Xor => true,
            _ => false,
        }
    }

    pub fn is_sce(&self) -> bool
    {
        match self {
            Self::Cand | Self::Cor => true,
            _ => false,
        }
    }

    pub fn is_cmp(&self) -> bool
    {
        match self {
            Self::Eq |
            Self::Ne |
            Self::Lt |
            Self::Gt |
            Self::Le |
            Self::Ge => true,
            _ => false,
        }
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
pub enum Stmt
{
    Assign(Expr, Expr),
    OperOn(Expr, BinOpcode, Expr),
    IfStmt(Expr, Block, Option<Block>), // last is else block
    LoopIf(Loop),
    BreakL(u32),
    Return(Expr),
    PcExit,
    PcCall(Expr, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr
{
    Const(Val),
    Ident(String),
    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UniOp(Box<Expr>, UniOpcode),
    CmpOp(Box<Expr>, Vec<(BinOpcode, Expr)>),
    FnDef(Rc<Subr>), // TODO upvalues
    Fcall(Box<Expr>, Vec<Expr>),
    RecFn,
    PcDef(Rc<Subr>),
    RecPc,
    Array(Vec<Expr>),
    Table(Vec<(String, Expr)>),
    TblFd(Box<Expr>, String),
    RecsT(u32),
}
