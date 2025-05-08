/* tast.rs: Typed AST */

use std::{rc::Rc, cell::RefCell, fmt};
use strum::EnumCount;
use strum_macros::EnumCount;
use num_enum::TryFromPrimitive;
use crate::{util, /*dflib,*/ util::{MutRc, DfStr}};

dccee8!{ #[derive(EnumCount)]
pub enum UniOp {
                   NEZ, NER,
                        INR,
    NOB, NOC, NON,
}}

dccee8!{ #[derive(EnumCount)]
pub enum BinOp {
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
pub struct OrdOp (pub OrdOp, pub OrdTyp);
}

dcceep!{
pub struct EquOp (pub bool, pub EquTyp);
}

dccee8!{
pub enum CmpOp
{
    Equ(EquOp),
    Ord(OrdOp),
}}

impl CmpOp
{
    pub fn negated(&self) -> Self
    {
        match self {
            Self::Equ(x) => Self::Equ(EquOp(!x.0, x.1)),
            Self::Ord(x) => Self::Ord(OrdOp(x.0.negated(), x.1)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr
{
    pub e: Expre,
    pub t: Type,
}

#[derive(Debug, Clone)]
pub enum Expre
{
    Const(Val),
    Local(IdfIdx, usize), // last is depþ of scopes
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UniOp(Box<Expr>, UniOp),
    CmpOp(Box<Expr>, Vec<(CmpOp, Expr)>),
    Tcast(Box<Expr>, Type),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub enum Stmt
{
    Declar(IdfIdx, Expr),
    VarAss(IdfIdx, Expr, usize), // last is depth of scopes, see semanal
    IfElse(IfCase, Vec<IfCase>, Option<Block>),
    Loooop(Loop),
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub struct Prog
{
    pub idents: Vec<DfStr>,
    pub main: Block,
}
