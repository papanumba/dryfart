/* src/asterix.rs */

use std::rc::Rc;
use std::cell::RefCell;
use crate::util;

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
    A, // array
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
            Self::A |
            Self::F => false,
        }
    }

    pub fn default_val(&self) -> Val
    {
        match self {
            Self::V => Val::V,
            Self::B => Val::B(false),
            Self::C => Val::C('\0'),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
            Self::F => panic!("cannot default function"),
            Self::A => Val::from_array(Array::new()),
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
            Val::A(_) => Type::A,
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
            Self::A => write!(f, "{{}}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array
{
    E,          // Empty array: unknown type until a val is pushed
    B(Vec<bool>),
    C(Vec<char>),
    N(Vec<u32>),
    Z(Vec<i32>),
    R(Vec<f32>),
}

impl Array
{
    #[inline]
    pub fn new() -> Self
    {
        Self::E
    }

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
            Type::C => Self::C(Vec::<char>::with_capacity(c)),
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

    // helper for Self::from<&str> kinda
    // replace escape sequences: N$, T$, $$, "$
    // TODO: is þere some way of not allocating new Strings?
    fn replace_esc_seq(s: &str) -> String
    {
        return s
            .replace("N$",  "\n")
            .replace("T$",  "\t")
            .replace("\"$", "\"")
            .replace("$$",  "$");
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
}

// TryInto is automatically implemented
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

// TryInto is automatically implemented
impl std::convert::TryFrom<&str> for Array
{
    type Error = &'static str;
    // s is already stript, ie it has no `"` arround
    fn try_from(s: &str) -> Result<Self, Self::Error>
    {
        return Ok(Self::C(
            Self::replace_esc_seq(s)
                .as_str()
                .chars()
                .collect(),
        ));
    }
}

impl std::fmt::Display for Array
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // special case for strings (C)
        if let Self::C(a) = self {
            write!(f, "\"")?;
            for c in a {
                write!(f, "{c}")?;
            };
            write!(f, "\"")?;
            return Ok(());
        }
        write!(f, "{{")?;
        // TODO: do not print tailing comma?
        match self {
            Self::E => {}, // empty
            Self::B(a) => for b in a {
                if *b {write!(f, "T, ",)?;}
                else  {write!(f, "F, ",)?;}
            },
            Self::C(_) => {}, // done
            Self::N(a) => for n in a { write!(f, "{n}, ")?; },
            Self::Z(a) => for z in a { write!(f, "{z}, ")?; },
            Self::R(a) => for r in a { write!(f, "{r}, ")?; },
        }
        write!(f, "}}")?;
        return Ok(());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val
{
    V,
    B(bool),
    C(char),
    N(u32),
    Z(i32),
    R(f32),
    A(Rc<RefCell<Array>>),
    F(Func),
}

/*
** Note: Val clone is always "shallow":
**  - for primitives (VBCNZR) it's just a Copy
**  - for heap objects (AF) it's an Rc::clone
*/

impl Val
{
    pub fn from_str_to_c(s: &str) -> Self
    {
        match s.chars().nth(3) {
            Some(c) => return Self::C(c),
            None => panic!("not valid char"),
        }
    }

    pub fn from_array(a: Array) -> Self
    {
        Self::A(Rc::new(RefCell::new(a)))
    }
}

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

#[derive(Clone)]
pub struct Func
{
    pars: Vec<String>,
    body: Block,
}

impl Func
{
    pub fn new(p: &Vec<String>, b: &Block) -> Self
    {
        // check uniques in p
        let mut p2: Vec<String> = p.clone();
        p2.sort();
        p2.dedup();
        if p2.len() != p.len() {
            panic!("duplicate parameters in decl of a func");
        }
        return Self {
            pars: (*p).clone(),
            body: (*b).clone(),
        };
    }

    pub fn parc(&self) -> usize
    {
        return self.pars.len();
    }

    pub fn pars(&self) -> &[String]
    {
        return self.pars.as_slice();
    }

    pub fn body(&self) -> &Block
    {
        return &self.body;
    }
}

impl PartialEq for Func
{
    // Required method
    fn eq(&self, _other: &Self) -> bool
    {
        return false;
    }
}

impl std::fmt::Debug for Func
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "#%...")
    }
}

#[derive(Debug, Clone)]
pub struct Proc
{
    name: String,
    pars: Vec<(Type, String)>,
    body: Block,
}

impl Proc
{
    pub fn new(n: &str, p: &Vec<(Type, String)>, b: &Block) -> Self
    {
        return Self {
            name: String::from(n),
            pars: p.clone(),
            body: (*b).clone(),
        };
    }

    pub fn name(&self) -> &str
    {
        return &self.name;
    }

    pub fn pars(&self) -> &[(Type, String)]
    {
        return self.pars.as_slice();
    }

    pub fn parc(&self) -> usize
    {
        return self.pars.len();
    }

    pub fn part(&self) -> Vec<Type>
    {
        return self.pars
            .iter()
            .map(|arg| (*arg).0.clone())
            .collect();
    }

    pub fn body(&self) -> &Block
    {
        return &self.body;
    }
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
    IfStmt(Expr, Block, Option<Block>), // cond, main block, else block
    LoopIf(Loop),
    BreakL(u32),
    Return(Expr),
    PcDecl(Proc),
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
    Fdefn(Func),
    Fcall(Box<Expr>, Vec<Expr>),
    Array(Vec<Expr>),
}
