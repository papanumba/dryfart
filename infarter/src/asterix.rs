/* src/asterix.rs */

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Type
{
    B, // bool
    C, // char
    N, // natural
    Z, // zahl
    R, // real
    F, // func
    A(ArrType, usize),
    // array: elem type, dim (>0)
}

// arrayable types
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ArrType { B, C, N, Z, R, F }

impl std::fmt::Display for ArrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Self::B => write!(f, "B%"),
            Self::C => write!(f, "C%"),
            Self::N => write!(f, "N%"),
            Self::Z => write!(f, "Z%"),
            Self::R => write!(f, "R%"),
            Self::F => write!(f, "#%"),
        }
    }
}

impl Type
{
    pub fn is_num(&self) -> bool
    {
        return match self {
            Self::N | Self::Z | Self::R => true,
            _ => false,
        }
    }

    pub fn default_val(&self) -> Val
    {
        return match self {
            Self::B => Val::B(false),
            Self::C => Val::C('\0'),
            Self::N => Val::N(0),
            Self::Z => Val::Z(0),
            Self::R => Val::R(0.0),
            _ => todo!(),
        }
    }
}

impl std::convert::From<&Val> for Type
{
    fn from(v: &Val) -> Self
    {
        return match v {
            Val::V    => todo!(),
            Val::B(_) => Type::B,
            Val::C(_) => Type::C,
            Val::N(_) => Type::N,
            Val::Z(_) => Type::Z,
            Val::R(_) => Type::R,
            Val::F(_) => Type::F,
            Val::A(a) => Type::A(a.get_type(), a.dim()),
        };
    }
}

impl std::fmt::Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Self::B => write!(f, "B%"),
            Self::C => write!(f, "C%"),
            Self::N => write!(f, "N%"),
            Self::Z => write!(f, "Z%"),
            Self::R => write!(f, "R%"),
            Self::F => write!(f, "#%"),
            Self::A(a, d) => {
                for _ in 0..*d { write!(f, "{{")?; }
                write!(f, "{}", a)?;
                for _ in 0..*d { write!(f, "}}")?; }
                Ok(())
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Array
{
    B(Vec<bool>),
    C(Vec<char>),
    N(Vec<u32>),
    Z(Vec<i32>),
    R(Vec<f32>),
}

impl Array
{
    pub fn new(t: &Type) -> Self
    {
        return Self::with_capacity(t, 0);
    }

    #[inline]
    fn with_capacity(t: &Type, c: usize) -> Self
    {
        return match t {
            Type::B => Self::B(Vec::<bool>::with_capacity(c)),
            Type::C => Self::C(Vec::<char>::with_capacity(c)),
            Type::N => Self::N(Vec::<u32> ::with_capacity(c)),
            Type::Z => Self::Z(Vec::<i32> ::with_capacity(c)),
            Type::R => Self::R(Vec::<f32> ::with_capacity(c)),
            _ => todo!(),
        };
    }

    fn try_push(&mut self, v: &Val)
    {
        match (self, v) {
            (Self::B(a), Val::B(b)) => a.push(*b),
            (Self::C(a), Val::C(c)) => a.push(*c),
            (Self::N(a), Val::N(n)) => a.push(*n),
            (Self::Z(a), Val::Z(n)) => a.push(*n),
            (Self::R(a), Val::R(n)) => a.push(*n),
            _ => todo!(),
        }
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

    pub fn get_type(&self) -> ArrType
    {
        return match self {
            Self::B(_) => ArrType::B,
            Self::C(_) => ArrType::C,
            Self::N(_) => ArrType::N,
            Self::Z(_) => ArrType::Z,
            Self::R(_) => ArrType::R,
        };
    }

    pub fn dim(&self) -> usize
    {
        return 1; // TODO: multidim arrs
    }

    pub fn get(&self, i: u32) -> Val
    {
        return match self {
            Self::B(a) => Val::B(a[i as usize]),
            Self::C(a) => Val::C(a[i as usize]),
            Self::N(a) => Val::N(a[i as usize]),
            Self::Z(a) => Val::Z(a[i as usize]),
            Self::R(a) => Val::R(a[i as usize]),
        };
    }

    pub fn len(&self) -> Val
    {
        return match self {
            Self::B(a) => Val::N(a.len() as u32),
            Self::C(a) => Val::N(a.len() as u32),
            Self::N(a) => Val::N(a.len() as u32),
            Self::Z(a) => Val::N(a.len() as u32),
            Self::R(a) => Val::N(a.len() as u32),
        };
    }
}

// TryInto is automatically implemented
impl std::convert::TryFrom<&[Val]> for Array
{
    type Error = &'static str;
    fn try_from(vals: &[Val]) -> Result<Self, Self::Error>
    {
        if vals.is_empty() {
            return Err("empty array");
        }
        // set self.typ as þe type of þe 1st element, þen try to push þe oþers
        let mut res = Self::with_capacity(&Type::from(&vals[0]), vals.len());
        for v in vals {
            res.try_push(&v);
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
        // special case for strings {C%}
        if let Self::C(a) = self {
            for c in a {
                write!(f, "{c}")?;
            };
            return Ok(());
        }
        write!(f, "{{")?;
        match self {
            Self::B(a) => for b in a { write!(f, "{}, ", *b)?; },
            Self::N(a) => for n in a { write!(f, "{n}, ")?; },
            Self::Z(a) => for z in a { write!(f, "{z}, ")?; },
            Self::R(a) => for r in a { write!(f, "{r}, ")?; },
            Self::C(_) => {}, // done
        }
        write!(f, "}}")?;
        return Ok(());
    }
}

pub fn try_arr_el(a: &Val, i: &Val) -> Val
{
    let arr: &Array = match a {
        Val::A(arr_val) => arr_val,
        _ => panic!("not indexable"),
    };
    let idx: u32 = match i {
        Val::N(idx_val) => *idx_val,
        _ => panic!("not an index"),
    };
    return arr.get(idx);
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
    A(Array),
    F(Func),
}

impl Val
{
    pub fn from_str_to_c(s: &str) -> Self
    {
        match s.chars().nth(3) {
            Some(c) => return Self::C(c),
            None => panic!("not valid char"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinOpcode { Add, Sub, Mul, Div, Eq, Ne, Lt, Gt, Le, Ge, And, Or }

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
            _ => panic!("unknown binop"),
        }
    }

    pub fn is_num(&self) -> bool
    {
        return match self {
            Self::Add |
            Self::Sub |
            Self::Mul |
            Self::Div => true,
            _ => false,
        };
    }

    pub fn is_bool(&self) -> bool
    {
        return match self {
            Self::And |
            Self::Or => true,
            _ => false,
        };
    }

    pub fn is_cmp(&self) -> bool
    {
        return match self {
            Self::Eq |
            Self::Ne |
            Self::Lt |
            Self::Gt |
            Self::Le |
            Self::Ge => true,
            _ => false,
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UniOpcode {
    Neg, // additive negative
    Inv, // multiplicative inverse
    Not, // boolean negation
}

#[derive(Debug, Clone)]
pub enum BlockAction
{
    Return(Val),
    PcExit,
    BreakL,
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
    fn eq(&self, other: &Self) -> bool
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
    Ini(       Expr, Block),
    Mid(Block, Expr, Block),
    Fin(Block, Expr       ),
}

pub type Block = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum Stmt
{
    Assign(String, Expr),
    OperOn(String, BinOpcode, Expr),
    IfStmt(Expr, Block, Option<Block>), // cond, main block, else block
    LoopIf(Loop),
    BreakL(u32),
    Return(Expr),
    PcDecl(Proc),
    PcExit,
    PcCall(String, Vec<Expr>),
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
    ArrEl(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
}
