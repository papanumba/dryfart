/* ASTerix */

use std::collections::HashMap;


pub struct Scope
{
    pub vars: HashMap<String, Val>,
    pub funs: HashMap<String, Func>,
    pub pros: HashMap<String, Proc>,
}

impl Scope
{
    fn new() -> Self
    {
        return Self {
            vars: HashMap::<String, Val> ::new(),
            funs: HashMap::<String, Func>::new(),
            pros: HashMap::<String, Proc>::new(),
        };
    }

    #[inline]
    pub fn clean(&mut self, scref: &ScopeRef)
    {
        for v in &scref.vars {
            self.vars.remove(v);
        }
        for f in &scref.funs {
            self.funs.remove(f);
        }
        for p in &scref.pros {
            self.pros.remove(p);
        }
    }

    fn print_vars(&self)
    {
        for (i, v) in &self.vars {
            println!("{}% {} = {:?}.", v.as_type().to_string(), i, v);
        }
    }
}

pub struct ScopeRef
{
    pub vars: Vec<String>,
    pub funs: Vec<String>,
    pub pros: Vec<String>,
}

impl ScopeRef
{
    pub fn new() -> Self
    {
        return Self {
            vars: Vec::new(),
            funs: Vec::new(),
            pros: Vec::new(),
        };
    }
}

/* MAIN FUNCTION to execute all þe programm */
// semantic analysis check
pub fn anal_check(prog: &Block)
{
    let mut root_scope = Scope::new();
    match do_block(&mut root_scope, prog) {
        Some(ba) => panic!("ERROR: at main script: cannot return or break"),
        None => {},
    };
    root_scope.print_vars();
}

fn do_block(scope: &mut Scope, b: &Block) -> Option<BlockAction>
{
    // keep track of þis block's new scope
    let mut blocks_scope = ScopeRef::new();
    // do statements
    for s in b {
        match do_stmt(scope, &mut blocks_scope, s) {
            Some(ba) => return Some(ba),
            None => {},
        };
    }
    // "free" þis block's vars, funs & pros from þe outer scope
    scope.clean(&blocks_scope);
    // noþing to do outside
    return None;
}

#[inline]
fn do_stmt(scope: &mut Scope, scref: &mut ScopeRef, s: &Stmt)
    -> Option<BlockAction>
{
    println!("\nvars: ");
    scope.print_vars();
    match s {
        Stmt::Declar(i, t)    => do_declar(scope, scref, i, t),
        Stmt::Assign(i, e)    => do_assign(scope, i, e),
        Stmt::IfStmt(c, b, e) => return do_ifstmt(scope, c, b, e),
        Stmt::LoopIf(c, b)    => return do_loopif(scope, c, b),
        Stmt::BreakL          => return Some(BlockAction::BreakL),
//        Stmt::FnDecl(f)       => print!("read function"),
        Stmt::Return(e)       =>
            return Some(BlockAction::Return(eval_expr(scope, e))),
        _ => todo!(),
    }
    return None;
}

#[inline]
fn do_declar(scope: &mut Scope, scref: &mut ScopeRef, id: &str, ty: &Type)
{
    if !scope.vars.contains_key(id) {
        scope.vars.insert(id.to_string(), (*ty).default_val());
        scref.vars.push(id.to_string());
    } else {
        panic!("variable {id} already made");
    }
}

fn do_assign(scope: &mut Scope, id: &str, value: &Expr)
{
    // check for declared var
    if !scope.vars.contains_key(id) {
        panic!("aaa dunno what is {id} variable");
    }
    // check for static type
    let new_val: Val = eval_expr(scope, value);
    if  new_val.as_type() == scope.vars.get(id).unwrap().as_type() {
        scope.vars.insert(id.to_string(), new_val);
    } else {
        panic!("assigning different types");
    }
}

fn do_ifstmt(
    scope: &mut Scope, cond: &Expr, bloq: &Block, else_b: &Option<Block>)
    -> Option<BlockAction>
{
    // eval cond
    let cond_bool = match eval_expr(scope, cond) {
        Val::B(b) => b,
        _ => panic!("condition is not B%"),
    };
    return if cond_bool {
        // report if any Returnable BlockAction found
        do_block(scope, bloq)
    } else {
        // if þere's an else block, return it's result
        match else_b {
            Some(eb) => do_block(scope, eb),
            None => None,
        }
    }
}

fn do_loopif(scope: &mut Scope, cond: &Expr, b: &Block) -> Option<BlockAction>
{
    /* code adapted from do_block */
    // keep track of þis block's new scope
    let mut blocks_scope = ScopeRef::new();
    // start looping
    loop {
        // check condition
        let eval_cond = match eval_expr(scope, cond) {
            Val::B(b) => b,
            _ => panic!("condition is not B%"),
        };
        if ! eval_cond {
            break;
        }
        // do_block inside þe loop
        for s in b {
            match do_stmt(scope, &mut blocks_scope, s) {
                Some(ba) => return match ba {
                    // only break þis loop
                    BlockAction::BreakL => None,
                    // return different action, for an outer func or proc
                    _ => Some(ba),
                },
                None => {},
            };
        }
    }
    // "free" þis block's vars, funs & pros from þe outer scope
    scope.clean(&blocks_scope);
    // noþing to do outside
    return None;
}

/*fn do_fndecl(&mut self, f: &Func) -> Option<BlockAction>
{
    let name: &str = f.name();
    if !self.funs.contains_key(name) {
        self.funs.insert(name.to_string(), f.clone());
    } else {
        panic!("function {name} already made");
    }
}*/

fn eval_expr(scope: &mut Scope, e: &Expr) -> Val
{
    match e {
        Expr::Const(c) => *c,
        Expr::Ident(i) => *scope.vars.get(i).unwrap(),
        Expr::Tcast(t, e) => do_cast(t, &eval_expr(scope, e)),
        Expr::BinOp(l, o, r) => eval_binop(
            &eval_expr(scope, l),
            o,
            &eval_expr(scope, r)
        ),
        /*Expr::Funcc(n, a) => match self.funs.get(n) {
            Some(f) => f.eval(&a
                .into_iter()
                .map(|b| self.eval_expr(&**b))
                .collect()),
            None => panic!("unknown func {n}"),
        },*/
        _ => todo!(),
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
        BinOpcode::Ne => return match (l, r) {
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
pub enum BlockAction
{
    Return(Val),
    PcExit,
    BreakL,
}

#[derive(Clone)]
pub struct Func
{
    name: String,
    pars: Vec<(Type, String)>,
    body: Block,
    rett: Type,
}

impl Func
{
    pub fn new(n: &str, p: &Vec<(Type, String)>, b: &Block, r: &Type) -> Self
    {
        // check uniques in p
        let mut p2: Vec<&str> = p
            .iter()
            .map(|pair| (pair.1).as_str())
            .collect::<Vec<&str>>();
        p2.sort();
        p2.dedup();
        if p2.len() != p.len() {
            panic!("duplicate parameters in decl of {n}#");
        }
        return Self {
            name: String::from(n),
            pars: (*p).clone(),
            body: (*b).clone(),
            rett: *r,
        };
    }

    pub fn name(&self) -> &str
    {
        return &self.name;
    }

    pub fn eval(&self, args: &Vec<Val>) -> Val
    {
        // self.pars is uniques
        if args.len() != self.pars.len() {
            panic!("not rite numbav args");
        }
        // check args types
        for (i, a) in args.iter().enumerate() {
            let p = &self.pars[i];
            let ta = a.as_type();
            if p.0 != ta {
                panic!("error at {}#: par {}%{} isn't {}%",
                    self.name, p.0.to_string(), p.1, ta.to_string());
            }
        }
        // TODO: eval block
        return self.rett.default_val();
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
    BreakL,
    FnDecl(Func),
    Return(Expr),
    PcDecl(Proc),
    PcExit,
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
