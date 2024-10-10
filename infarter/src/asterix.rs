/* asterix.rs */

use std::{rc::Rc, cell::RefCell, fmt};
use crate::{util, /*dflib,*/ util::{MutRc, DfStr}};

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

pub const ESC_CH: u8 = b'?';

#[derive(Debug, Clone)]
pub enum Val
{
    V,
    B(bool),
    C(u8),
    N(u32),
    Z(i32),
    R(f64),
}

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
            _ => false, // R
        }
    }
}

impl Eq for Val {}

impl fmt::Display for Val
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Val::V    => write!(f, "V"),
            Val::B(b) => write!(f, "{}", if *b {'T'} else {'F'}),
            Val::C(c) => write!(f, "{}", char::from(*c)),
            Val::N(n) => write!(f, "{n}u"),
            Val::Z(z) => write!(f, "{z}"),
            Val::R(r) => write!(f, "{r}"),
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

/*#[derive(Debug, Clone)]
pub enum Loop
{
    Inf(Block),
    Cdt(Block, Expr, Block),
}*/

pub type Block = Vec<Stmt>;

/*#[derive(Debug, Clone)]
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
pub struct SwCase
{
    pub comp: Expr, // þis may be expanded in þe futur
    pub blok: Block,
}*/

#[derive(Debug, Clone)]
pub enum Stmt
{
    Assign(Expr, Expr),
//    OperOn(Expr, BinOpcode, Expr),
//    IfElse(IfCase, Vec<IfCase>, Option<Block>),
//    Switch(Expr,   Vec<SwCase>, Block),
//    LoopIf(Loop),
//    AgainL(u32),
//    BreakL(u32),
//    Return(Expr),
//    PcExit,
//    PcCall(Expr, Vec<Expr>),
//    TbPCal(Expr, Rc<DfStr>, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr
{
    Const(Val),
    Ident(Rc<DfStr>),
//    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UniOp(Box<Expr>, UniOpcode),
    CmpOp(Box<Expr>, Vec<(BinOpcode, Expr)>),
//    IfExp(Vec<(Expr, Expr)>, Box<Expr>),
}
