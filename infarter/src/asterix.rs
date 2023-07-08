/* ASTerix */

use std::collections::HashMap;

pub struct SemAnal
{
    vars: HashMap<String, Val>,
}

impl SemAnal
{
    pub fn new() -> Self
    {
        return Self { vars: HashMap::<String, Val>::new() };
    }

    // semantic analysis check
    pub fn anal_check(&mut self, prog: &Block)
    {
        self.do_block(prog);
        self.print_vars();
    }

    fn do_block(&mut self, b: &Block)
    {
        for s in b {
            self.do_stmt(&s);
        }
    }

    fn do_stmt(&mut self, s: &Stmt)
    {
        match s {
            Stmt::Declar(i, t)    => self.do_declar(i, t),
            Stmt::Assign(i, e)    => self.do_assign(i, e),
            Stmt::IfStmt(c, b, e) => self.do_ifstmt(c, b, e),
            Stmt::LoopIf(c, b)    => self.do_loopif(c, b),
            Stmt::FnDecl(f)       => print!("read function"),
            Stmt::Return(e)       => todo!(),
            _ => todo!(),
        }
    }

    fn do_declar(&mut self, id: &str, ty: &Type)
    {
        if !self.vars.contains_key(id) {
            self.vars.insert(id.to_string(), (*ty).default_val());
        } else {
            panic!("variable {id} already made");
        }
    }

    fn do_assign(&mut self, id: &str, value: &Expr)
    {
        if !self.vars.contains_key(id) {
            panic!("aaa dunno what is {id} variable");
        }
        let new_val: Val = self.eval_expr(value);
        if  new_val.as_type() == self.vars.get(id).unwrap().as_type() {
            self.vars.insert(id.to_string(), new_val);
        } else {
            panic!("assigning different types");
        }
    }

    fn do_ifstmt(&mut self, cond: &Expr, bloq: &Block, else_b: &Option<Block>)
    {
        let cond_bool = match self.eval_expr(cond) {
            Val::B(b) => b,
            _ => panic!("condition is not B%"),
        };
        if cond_bool {
            self.do_block(bloq);
        } else {
            match else_b {
                Some(b) => self.do_block(b),
                None => {},
            };
        }
    }

    fn do_loopif(&mut self, cond: &Expr, b: &Block)
    {
        loop {
            let eval_cond = match self.eval_expr(cond) {
                Val::B(b) => b,
                _ => panic!("condition is not B%"),
            };
            if ! eval_cond {
                break;
            }
            self.do_block(b);
        }
    }

    fn eval_expr(&self, e: &Expr) -> Val
    {
        match e {
            Expr::Const(c) => *c,
            Expr::Ident(i) => *self.vars.get(i).unwrap(),
            Expr::Tcast(t, e) => do_cast(t, &self.eval_expr(e)),
            Expr::BinOp(l, o, r) => eval_binop(
                &self.eval_expr(l),
                o,
                &self.eval_expr(r)
            ),
            _ => todo!(),
        }
    }

    fn print_vars(&self)
    {
        for (i, v) in &self.vars {
            println!("{}% {} = {:?}.", v.as_type().to_string(), i, v);
        }
    }
}

fn eval_binop(l: &Val, o: &BinOpcode, r: &Val) -> Val
{
    let lt = l.as_type();
    let rt = r.as_type();

    if lt != rt {
        panic!("operating different types");
    }

    // first, check cmp operations
    match o {
        // equivalence cmp only btwin þe same type
        BinOpcode::Eq => return match (l, r) {
            (Val::R(_), Val::R(_)) => panic!("use an epsilon wiþ R% u idiot"),
            _ => Val::B(l == r),
        },
        BinOpcode::Eq => return match (l, r) {
            (Val::R(_), Val::R(_)) => panic!("use an epsilon wiþ R% u idiot"),
            _ => Val::B(l != r),
        },
        // order cmp only btwin numerical types
        // TODO: impl PartialOrd for Val { if is_num }
        BinOpcode::Lt => match (l, r) {
            (Val::N(vl), Val::N(vr)) => return Val::B(vl < vr),
            (Val::Z(vl), Val::Z(vr)) => return Val::B(vl < vr),
            (Val::R(vl), Val::R(vr)) => return Val::B(vl < vr),
            _ => panic!("comparing different types"),
        }
        BinOpcode::Gt => match (l, r) {
            (Val::N(vl), Val::N(vr)) => return Val::B(vl > vr),
            (Val::Z(vl), Val::Z(vr)) => return Val::B(vl > vr),
            (Val::R(vl), Val::R(vr)) => return Val::B(vl > vr),
            _ => panic!("comparing different types"),
        }
        BinOpcode::Le => match (l, r) {
            (Val::N(vl), Val::N(vr)) => return Val::B(vl <= vr),
            (Val::Z(vl), Val::Z(vr)) => return Val::B(vl <= vr),
            (Val::R(vl), Val::R(vr)) => return Val::B(vl <= vr),
            _ => panic!("comparing different types"),
        }
        BinOpcode::Ge => match (l, r) {
            (Val::N(vl), Val::N(vr)) => return Val::B(vl >= vr),
            (Val::Z(vl), Val::Z(vr)) => return Val::B(vl >= vr),
            (Val::R(vl), Val::R(vr)) => return Val::B(vl >= vr),
            _ => panic!("comparing different types"),
        }
        _ => {}, // continue
    }

    // then, check num & bool operations
    return match (l, r) {
        (Val::N(vl), Val::N(vr)) => match o {
            BinOpcode::Add => Val::N(vl + vr),
            BinOpcode::Mul => Val::N(vl * vr),
            _ => panic!("not valid operation"),
        },
        (Val::Z(vl), Val::Z(vr)) => match o {
            BinOpcode::Add => Val::Z(vl + vr),
            BinOpcode::Sub => Val::Z(vl - vr),
            BinOpcode::Mul => Val::Z(vl * vr),
            _ => panic!("not valid operation"),
        },
        (Val::R(vl), Val::R(vr)) => match o {
            BinOpcode::Add => Val::R(vl + vr),
            BinOpcode::Sub => Val::R(vl - vr),
            BinOpcode::Mul => Val::R(vl * vr),
            BinOpcode::Div => Val::R(vl / vr),
            _ => panic!("not valid operation"),
        },
        (Val::B(vl), Val::B(vr)) => match o {
            BinOpcode::And => Val::B(*vl && *vr),
            BinOpcode::Or  => Val::B(*vl || *vr),
            _ => panic!("unknown op btwin B%"),
        },
        _ => panic!("not valid operation"),
    }
}

// return v casted into t
fn do_cast(t: &Type, v: &Val) -> Val
{
    if v.as_type() == *t {
        return *v;
    }
    return match (v, t) {
        (Val::N(n), Type::Z) => Val::Z(*n as i32),
        (Val::Z(z), Type::R) => Val::R(*z as f32),
        _ => panic!("converting types"),
    }
}

#[derive(Clone)]
pub struct Func
{
    name: String,
    args: Vec<(Type, String)>,
    body: Block,
    rett: Type,
}

impl Func
{
    pub fn new(n: &str, a: &Vec<(Type, String)>, b: &Block, r: &Type) -> Self
    {
        return Self {
            name: String::from(n),
            args: a.clone(),
            body: (*b).clone(),
            rett: *r,
        };
    }
}

#[derive(Clone)]
pub struct Proc
{
    name: String,
    args: Vec<(Type, String)>,
    body: Block,
}

impl Proc
{
    pub fn new(n: &str, a: &Vec<(Type, String)>, b: &Block) -> Self
    {
        return Self {
            name: String::from(n),
            args: a.clone(),
            body: (*b).clone(),
        };
    }
}

pub type Block = Vec<Stmt>;

#[derive(Clone)]
pub enum Stmt
{
    Assign(String, Expr),
    Declar(String, Type),
    IfStmt(Expr, Block, Option<Block>), // cond, main block, else block
    LoopIf(Expr, Block),
    FnDecl(Func),
    Return(Expr),
    PcDecl(Proc),
    PcExit(),
    PcCall(String, Vec<Box<Expr>>),
}

#[derive(Clone)]
pub enum Expr
{
    Const(Val),
    Ident(String),
    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UniOp(Box<Expr>, UniOpcode),
    Funcc(String, Vec<Box<Expr>>),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Type { B, C, N, Z, R }

impl Type
{
    pub fn from_str(s: &str) -> Self
    {
        return match s {
            "B" => Self::B,
            "C" => Self::C,
            "N" => Self::N,
            "Z" => Self::Z,
            "R" => Self::R,
            _ => panic!("unknown type"),
        };
    }

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
        }
    }
}

impl std::string::ToString for Type
{
    fn to_string(&self) -> String
    {
        return String::from(match self {
            Self::B => "B",
            Self::C => "C",
            Self::N => "N",
            Self::Z => "Z",
            Self::R => "R",
        });
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Val
{
    B(bool),
    C(char),
    N(u32),
    Z(i32),
    R(f32),
}

impl Val
{
    pub fn as_type(&self) -> Type
    {
        return match self {
            Self::B(_) => Type::B,
            Self::C(_) => Type::C,
            Self::N(_) => Type::N,
            Self::Z(_) => Type::Z,
            Self::R(_) => Type::R,
        };
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
            "/=" => Self::Ne,
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

#[derive(Clone)]
pub enum UniOpcode { Sub }
