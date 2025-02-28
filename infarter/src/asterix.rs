/* asterix.rs */

use std::{rc::Rc, cell::RefCell, fmt};
use strum::EnumCount;
use strum_macros::EnumCount;
use num_enum::TryFromPrimitive;
use crate::{util, /*dflib,*/ util::{MutRc, DfStr}};

pub type IdfIdx = usize;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, TryFromPrimitive)]
pub enum FundTy
{
    B = b'B', // bool
    C = b'C', // char
    N = b'N', // natural
    Z = b'Z', // zahl
    R = b'R', // real
}

impl FundTy
{
    pub fn is_num(&self) -> bool
    {
        matches!(self, Self::C | Self::N | Self::Z | Self::R)
    }

    pub fn default_val(&self) -> Val
    {
        match self {
            Self::B => Val::B(false),
            Self::C => Val::C(0),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
        }
    }
}

impl fmt::Display for FundTy
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", char::from(*self as u8))
    }
}

impl From<FundTy> for Type
{
    fn from(ft: FundTy) -> Type
    {
        return Type::Fund(ft);
    }
}

pub struct SubrTy
{
    arg: Vec<Type>,
    ret: Option<Type>, // Some => func, None => proc
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type
{
    Fund(    FundTy),
//    Subr(Box<SubrTy>),
    Arrr(Box<Type>),
}

impl Type
{
    // aux fn
    pub fn is_b(&self) -> bool
    {
        return *self == Type::Fund(FundTy::B);
    }
}

impl std::convert::From<&Val> for Type
{
    fn from(v: &Val) -> Self
    {
        match v {
            Val::B(_) => Type::Fund(FundTy::B),
            Val::C(_) => Type::Fund(FundTy::C),
            Val::N(_) => Type::Fund(FundTy::N),
            Val::Z(_) => Type::Fund(FundTy::Z),
            Val::R(_) => Type::Fund(FundTy::R),
        }
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::Fund(t) => write!(f, "{t}"),
            Self::Arrr(t) => write!(f, "_{}", *t),
        }
    }
}

pub const ESC_CH: u8 = b'?';

#[derive(Debug, Clone)]
pub enum Val
{
    B(bool),
    C(u8),
    N(u32),
    Z(i32),
    R(f64),
    // TODO: to incl refs
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
            Val::B(b) => write!(f, "{}", if *b {'T'} else {'F'}),
            Val::C(c) => write!(f, "{}", char::from(*c)),
            Val::N(n) => write!(f, "{n}u"),
            Val::Z(z) => write!(f, "{z}"),
            Val::R(r) => write!(f, "{r}"),
        }
    }
}

// AST stuff ------------------------------------

macro_rules! dccee {
    ($t:item) => { #[derive(Copy, Clone, Debug, Eq, PartialEq)] $t }
}

macro_rules! dccee8 { // usefull for small enums
    ($t:item) => { #[repr(u8)] dccee!{ $t } }
}

macro_rules! dcceep { // for structs
    ($t:item) => { #[repr(packed)] dccee!{ $t } }
}

pub(crate) use dccee;
pub(crate) use dccee8;

dccee8!{
pub enum UniOp { Neg, Inv, Not }
}

dccee8!{
pub enum BinOp { Add, Sub, Mul, Div, Mod, And, Ior, Xor, Typ }
}

macro_rules! is_sth_fn {
    ($name:ident, $($member:ident),+) => {
        pub fn $name(&self) -> bool
        {
            matches!(self, $(Self::$member)|+)
        }
    }
}

impl BinOp
{
    is_sth_fn!(is_num, Add, Sub, Mul, Div, Mod);
    is_sth_fn!(is_bit, And, Ior, Xor);
//    is_sth_fn!(is_sce, Cand, Cor);
}

dccee8!{
pub enum OrdOp { Lt, Le, Gt, Ge }
}

impl OrdOp
{
    pub fn negated(&self) -> Self
    {
        match self {
            Self::Lt => Self::Ge,
            Self::Le => Self::Gt,
            Self::Gt => Self::Le,
            Self::Ge => Self::Lt,
        }
    }
}

dccee8!{
pub enum CmpOp
{
    Equ(bool),
    Ord(OrdOp),
}}

// TODO Cand &?, Cor |?

#[derive(Debug, Clone)]
pub struct IfCase
{
    pub cond: Expr,
    pub blok: Block,
}

/*#[derive(Debug, Clone)]
pub struct SwCase
{
    pub comp: Expr, // þis may be expanded in þe futur
    pub blok: Block,
}*/

#[derive(Debug, Clone)]
pub enum Expr
{
//    DfLib,
    Const(Val),
    Ident(IdfIdx),
//    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UniOp(Box<Expr>, UniOp),
    CmpOp(Box<Expr>, Vec<(CmpOp, Expr)>),
//    IfExp(Vec<(Expr, Expr)>, Box<Expr>),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Loop
{
    Inf(Block),
    Cdt(Block, Expr, Block),
}

#[derive(Debug, Clone)]
pub enum Stmt
{
    Assign(Expr, Expr),
//    OperOn(Expr, BinOpcode, Expr),
    IfElse(IfCase, Vec<IfCase>, Option<Block>),
//    Switch(Expr,   Vec<SwCase>, Block),
    Loooop(Loop),
//    AgainL(u32),
//    BreakL(u32),
//    Return(Expr),
//    PcExit,
//    PcCall(Expr, Vec<Expr>),
//    TbPCal(Expr, Rc<DfStr>, Vec<Expr>),
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Prog
{
    pub idents: Vec<DfStr>,
    pub main: Block,
}

// TODO: move all after this to semanal
// AST wiþ types, after SemAnal

dccee8!{ #[derive(EnumCount)]
pub enum UniOpWt {
                   NEZ, NER,
                        INR,
    NOB, NOC, NON,
}}

dccee8!{ #[derive(EnumCount)]
pub enum BinOpWt {
         ADC, ADN, ADZ, ADR,
                   SUZ, SUR,
         MUC, MUN, MUZ, MUR,
              DIN,      DIR,
         MOC, MON, MOZ,      // MOZ is %Z \ %N
    ANB, ANC, ANN,
    IOB, IOC, ION,
    XOB, XOC, XON,
}}

// EQU CMP

dccee8!{ #[derive(EnumCount)]
pub enum EquTyp { B, C, N, Z }
}

// ORD CMP

// Types which can be compared using OrdOps
dccee8!{ #[derive(EnumCount)]
pub enum OrdTyp { C, N, Z, R }
}

dcceep!{
pub struct OrdOpWt (pub OrdOp, pub OrdTyp);
}

dcceep!{
pub struct EquOpWt (pub bool, pub EquTyp);
}

dccee8!{
pub enum CmpOpWt
{
    Equ(EquOpWt),
    Ord(OrdOpWt),
}}

impl CmpOpWt
{
    pub fn negated(&self) -> Self
    {
        match self {
            Self::Equ(x) => Self::Equ(EquOpWt(!x.0, x.1)),
            Self::Ord(x) => Self::Ord(OrdOpWt(x.0.negated(), x.1)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprWt
{
    pub e: ExprWte,
    pub t: Type,
}

#[derive(Debug, Clone)]
pub enum ExprWte
{
    Const(Val),
    Local(IdfIdx, usize), // last is depþ of scopes
    BinOp(Box<ExprWt>, BinOpWt, Box<ExprWt>),
    UniOp(Box<ExprWt>, UniOpWt),
    CmpOp(Box<ExprWt>, Vec<(CmpOpWt, ExprWt)>),
    Tcast(Box<ExprWt>, Type),
    Array(Vec<ExprWt>),
}

#[derive(Debug, Clone)]
pub struct IfCaseWt
{
    pub cond: ExprWt,
    pub blok: BlockWt,
}

#[derive(Debug, Clone)]
pub enum LoopWt
{
    Inf(BlockWt),
    Cdt(BlockWt, ExprWt, BlockWt),
}

#[derive(Debug, Clone)]
pub enum StmtWt
{
    Declar(IdfIdx, ExprWt),
    VarAss(IdfIdx, ExprWt, usize), // last is depth of scopes, see semanal
    IfElse(IfCaseWt, Vec<IfCaseWt>, Option<BlockWt>),
    Loooop(LoopWt),
}

pub type BlockWt = Vec<StmtWt>;

#[derive(Debug, Clone)]
pub struct ProgWt
{
    pub idents: Vec<DfStr>,
    pub main: BlockWt,
}
