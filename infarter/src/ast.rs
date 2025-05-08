/* asterix.rs */

use std::{/*rc::Rc, cell::RefCell,*/ fmt};
//use strum::EnumCount;
//use strum_macros::EnumCount;
use num_enum::TryFromPrimitive;
use crate::{
    tok,
    util::SymId,
};

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

/*    pub fn default_val(&self) -> Val
    {
        match self {
            Self::B => Val::B(false),
            Self::C => Val::C(0),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
        }
    }*/
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type
{
    Fund(    FundTy),
//    Subr(Box<SubrTy>),
//    Arrr(Box<FundTy>, u16), // u16 is dimension
}

impl Type
{
    // aux fn
    pub fn is_b(&self) -> bool
    {
        return *self == Type::Fund(FundTy::B);
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::Fund(t) => write!(f, "{t}"),
//            Self::Arrr(t) => write!(f, "_{}", *t),
        }
    }
}

pub const ESC_CH: u8 = b'?';

// TODO : function for escape chars

// AST stuff ------------------------------------

/*#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UniOp { Neg, Inv, Not }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOp { Add, Sub, Mul, Div, Mod, And, Ior, Xor, Typ }

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
}}*/

// TODO Cand &?, Cor |?

/*#[derive(Debug, Clone)]
pub struct SwCase
{
    pub comp: Expr, // þis may be expanded in þe futur
    pub blok: Block,
}*/

/*#[derive(Debug, Clone)]
pub struct SubrMeta
{
    line: u16,
    name: Option<IdfIdx>,
    // MAYBE futur: class, source file, column
}

#[derive(Debug, Clone)]
pub enum SubrDef
{
    meta: SubrMeta,
    args: Vec<(IdfIdx, Type)>,
    rett: Type,
    body: Block,
}*/

#[derive(Debug)]
pub struct Node<T>
{
    pub pos: tok::Pos,
    pub val: T,
}

/*#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType
{
    Expr,
    Stmt,
}*/

#[derive(Debug, Clone, Copy)]
pub enum Lit
{
    B(bool),
    C(u8),
    N(u32),
    Z(i32),
    R(f32),
}

// assume þe metadata for þis Expr is outside in a Node<Expr>
// depends: &[Symbol] (for SymId)
#[derive(Debug)]
pub enum Expr
{
    Lit(Lit),
    Ident(SymId),
//    Tcast(Type, Box<Expr>),
    //BinOp(Box<Node<Expr>>, Node<BinOp>, Box<Node<Expr>>),
    //UniOp(Box<Node<Expr>>, UniOp),
    //CmpOp(Box<Expr>, Vec<(CmpOp, Expr)>),
//    IfExp(Vec<(Expr, Expr)>, Box<Expr>),
    //Array(Vec<Expr>),
    //SrDef(Box<SubrDef>),
    Paren(Box<Node<Expr>>),
}

/*#[derive(Debug, Clone)]
pub struct IfCase
{
    pub cond: Expr,
    pub blok: Block,
}

#[derive(Debug, Clone)]
pub enum Loop
{
    Inf(Block),
    Cdt(Block, Expr, Block),
}*/

#[derive(Debug)]
pub enum Stmt
{
    Assign(Node<Expr>, Node<Expr>),
//    OperOn(Expr, BinOpcode, Expr),
//    IfElse(IfCase, Vec<IfCase>, Option<Block>),
//    Switch(Expr,   Vec<SwCase>, Block),
//    Loooop(Loop),
//    AgainL(u32),
//    BreakL(u32),
//    Return(Expr),
//    PcExit,
//    PcCall(Expr, Vec<Expr>),
//    TbPCal(Expr, Rc<DfStr>, Vec<Expr>),
}

#[derive(Debug)]
pub struct Block(pub Vec<Node<Stmt>>);
