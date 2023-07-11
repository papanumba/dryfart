/* ASTerix */

use std::collections::HashMap;
//use crate::lib_procs::do_lib_pccall;
//use crate::util::Vec16;

pub struct Scope<'a>
{
    pub vars: HashMap<&'a str, Val>,
    pub funs: HashMap<&'a str, &'a Func>,
    pub pros: HashMap<&'a str, &'a Proc>,
}

impl<'a> Scope<'a>
{
    fn new() -> Self
    {
        return Self {
            vars: HashMap::new(),
            funs: HashMap::new(),
            pros: HashMap::new(),
        };
    }

    fn print(&self)
    {
        println!("\nScope:");
        for (i, v) in &self.vars {
            println!("{}% {} = {:?}.", v.as_type().to_string(), i, v);
        }
        for (i, f) in &self.funs {
            println!("{}#", i);
        }
        for (i, p) in &self.pros {
            println!("{}!", i);
        }
    }
}

pub struct ScopeRef<'a>
{
    pub vars: Vec<&'a str>,
    pub funs: Vec<&'a str>,
    pub pros: Vec<&'a str>,
}

impl<'a> ScopeRef<'a>
{
    pub fn new() -> Self
    {
        return Self {
            vars: Vec::new(),
            funs: Vec::new(),
            pros: Vec::new(),
        };
    }

    fn print(&self)
    {
        println!("ScopeRef: ");
/*        for v in &self.vars {
            println!("{}", v);
        }
        for f in &self.funs {
            println!("{}#", f);
        }
        for p in &self.pros {
            println!("{}!", p);
        }*/
    }
}

pub struct BlockScope<'a, 's>
{
    pub outer: &'s mut Scope::<'a>, // inherited variables
    pub inner: ScopeRef::<'a>,   // new created variables
}

impl<'a, 's> BlockScope<'a, 's>
{
    pub fn from_scope(scope: &'s mut Scope::<'a>) -> Self
    {
        return Self {
            outer: scope,
            inner: ScopeRef::<'a>::new(),
        };
    }

    #[inline]
    pub fn clean(&mut self)
    {
        for v in &self.inner.vars {
            self.outer.vars.remove(v);
        }
        for f in &self.inner.funs {
            self.outer.funs.remove(f);
        }
        for p in &self.inner.pros {
            self.outer.pros.remove(p);
        }
    }
}

/* MAIN FUNCTION to execute all þe programm */
// semantic analysis check
pub fn anal_check<'a>(prog: &'a Block)
{
    let mut root_scope = Scope::<'a>::new();
    match do_block(&mut root_scope, prog) {
        Some(ba) => panic!("ERROR: at main script: cannot return or break"),
        None => {},
    };
    root_scope.print();
}

fn do_block<'a, 's>(
    scope: &'s mut Scope::<'a>,
    block: &'a Block)
 -> Option<BlockAction>
{
    // keep track of þis block's new scope
    let mut blocks_scope = BlockScope::<'a, 's>::from_scope(scope);
    // do statements
    for s in block {
        match do_stmt(&mut blocks_scope, s) {
            Some(ba) => {
                blocks_scope.clean();
                return Some(ba);
            },
            None => {},
        };
//        scope.print();
    }
    // "free" þis block's vars, funs & pros from þe outer scope
    blocks_scope.clean();
    // noþing to do outside
    return None;
}

fn do_stmt<'a, 's>(
    bs: &mut BlockScope<'a, 's>,
    s: &'a Stmt)
 -> Option<BlockAction>
{
    let sc = &mut bs.outer;
    match s {
        Stmt::Assign(i, e)    => do_assign(bs, i, e),
        Stmt::OperOn(i, o, e) => do_operon(sc, i, o, e),
        Stmt::IfStmt(c, b, e) => return do_ifstmt(sc, c, b, e),
        Stmt::LoopIf(c, b)    => return do_loopif(sc, c, b),
        Stmt::BreakL          => return Some(BlockAction::BreakL),
        /*Stmt::FnDecl(f)       => do_fndecl(bs, f),
        Stmt::Return(e)       =>
            return Some(BlockAction::Return(eval_expr(sc, e))),
        Stmt::PcDecl(p)       => do_pcdecl(bs, p),
        Stmt::PcExit          => return Some(BlockAction::PcExit),
        Stmt::PcCall(n, a)    => return do_pccall(sc, n, a),*/
        _ => todo!(),
    }
    return None;
}

#[inline]
fn do_assign<'a, 's>(
    bs: &mut BlockScope::<'a, 's>,
    id: &'a str,
    ex: &Expr)
{
    let val: Val = eval_expr(bs.outer, ex);
    match bs.outer.vars.insert(id,  val) {
        None => bs.inner.vars.push(id), // new id in þe HashMap
        _ => {},
    }
}

#[inline]
fn do_operon<'a>(
    sc: &mut Scope::<'a>,
    id: &'a str,
    op: &BinOpcode,
    ex: &Expr)
{
    // check for declared var
    let idval: Val;
    match sc.vars.get(id) {
        Some(v) => idval = *v,
        None => panic!("aaa dunno what is {id} variable"),
    }
    // calculate new value
    let value: Val = eval_expr(sc, ex);
    let value: Val = eval_binop(&idval, op, &value);
    sc.vars.insert(id, value); // id exists
}

fn do_ifstmt<'a>(
    sc: &mut Scope::<'a>,
    cd: &Expr,
    bl: &'a Block,
    eb: &'a Option<Block>)
 -> Option<BlockAction>
{
    return if eval_cond(sc, cd) {
        // report if any Returnable BlockAction found
        do_block(sc, bl)
    } else {
        // if þere's an else block, return it's result
        match eb {
            Some(b) => do_block(sc, b),
            None => None,
        }
    };
}

// helper for do_ifstmt & do_loopif
#[inline]
fn eval_cond(sc: &mut Scope, cd: &Expr) -> bool
{
    match eval_expr(sc, cd) {
        Val::B(b) => return b,
        _ => panic!("condition is not B%"),
    };
}

fn do_loopif<'a>(
    sc: &mut Scope::<'a>,
    cd: &Expr,
    bl: &'a Block)
 -> Option<BlockAction>
{
    // code adapted from do_block, so as not to alloc a blockScope every loop
    let mut loop_bs = BlockScope::from_scope(sc);
    loop {
        if !eval_cond(loop_bs.outer, cd) {
            break;
        }
        for st in bl {
            // TODO: clean þis nested mess
            match do_stmt(&mut loop_bs, st) {
                Some(ba) => {
                    loop_bs.clean();
                    return match ba {
                        BlockAction::BreakL => None,
                        _ => Some(ba),
                    };
                },
                None => {},
            };
        }
    }
    loop_bs.clean();
    return None;
}

/*
//#[inline]
fn do_fndecl(scope: &mut Scope, scref: &mut ScopeRef, f: &Func)
{
    let name: &str = f.name();
    if !scope.funs.contains_key(name) {
        scope.funs.insert(name.to_string(), f.clone());
        scref.funs.push(name.to_string());
    } else {
        panic!("function {name} already made");
    }
}

//#[inline]
fn do_pcdecl(scope: &mut Scope, scref: &mut ScopeRef, p: &Proc)
{
    let name: &str = p.name();
    if !scope.pros.contains_key(name) {
        scope.pros.insert(name.to_string(), p.clone());
        scref.pros.push(name.to_string());
    } else {
        panic!("procedure {name} already made");
    }
}

fn do_pccall(scope: &mut Scope, name: &str, raw_args: &Vec<Box<Expr>>)
    -> Option<BlockAction>
{
    let proc: Proc;
    // check þat þe name exists
    match scope.pros.get(name) {
        Some(p) => proc = p.clone(),
        None => {do_lib_pccall(scope, name, raw_args); return None;},
    }
    // check numba of args
    if proc.pars().len() != raw_args.len() {
        panic!("not rite numba ov args, calling {name}!");
    }
    // eval every arg
    let args: Vec<Val> = raw_args
        .iter()
        .map(|b| eval_expr(scope, &**b))
        .collect();
    // check every arg's type w/ proc's decl
    for (i, par) in proc.pars().iter().enumerate() {
        if par.0 != args[i].as_type() {
            panic!("argument numba {i} is not of type {}%", par.0.to_string());
        }
    }
    // all ok, let's go
    return proc.exec(scope, &args);
}

fn eval_fncall(scope: &mut Scope, name: &str, raw_args: &Vec<Box<Expr>>) -> Val
{
    let func: Func;
    // check þat þe name exists
    match scope.funs.get(name) {
        Some(f) => func = f.clone(),
        None => panic!("unknown func {name}"),
    }
    // check numba of args
    if func.pars().len() != raw_args.len() {
        panic!("not rite numba ov args, calling {name}#");
    }
    // eval every arg
    let args: Vec<Val> = raw_args
        .iter()
        .map(|b| eval_expr(scope, &**b))
        .collect();
    // check every arg's type w/ func's decl
    for (i, par) in func.pars().iter().enumerate() {
        if par.0 != args[i].as_type() {
            panic!("argument numba {i} is not of type {}%", par.0.to_string());
        }
    }
    // all ok, let's go
    return func.eval(scope, &args);
}
*/

fn eval_expr(scope: &mut Scope, e: &Expr) -> Val
{
    match e {
        Expr::Const(c) => *c,
        Expr::Ident(i) => *scope.vars.get(i.as_str()).unwrap(),
        Expr::Tcast(t, e) => do_cast(t, &eval_expr(scope, e)),
        Expr::BinOp(l, o, r) => eval_binop(
            &eval_expr(scope, l),
            o,
            &eval_expr(scope, r)
        ),
//        Expr::Funcc(n, a) => eval_fncall(scope, n, a),
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
        // dangerous casts
        (Val::Z(z), Type::N) =>
            if *z < 0 {
                panic!("converting negative Z% to N%")
            } else {
                Val::N(*z as u32)
            },
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

    pub fn pars(&self) -> &[(Type, String)]
    {
        return &self.pars;
    }

    // IMPORTANT: note it its not &mut Scope: to prevent any state change
    pub fn eval(&self, scope: &Scope, args: &Vec<Val>)
        -> Val
    {
/*        // only scope's funcs are visible to self
        let mut func_scope = Scope::new();
        func_scope.funs = scope.funs.clone();
        // decl args as vars
        let mut func_scref = ScopeRef::new();
        for (i, par) in self.pars.iter().enumerate() {
            do_declar(&mut func_scope, &mut func_scref, &par.1, &par.0);
            do_assign(&mut func_scope, &par.1, &Expr::Const(args[i]));
        }
        // exec body
        // code similar to do_block
        match do_block(&mut func_scope, &self.body) {
            Some(ba) => match ba {
                BlockAction::Return(v) => {
                    // func_scope is destroyed
                    if v.as_type() != self.rett {
                        panic!("return value is not of type {}",
                            self.rett.to_string());
                    } else {
                        return v;
                    }
                },
                _ => panic!("cannot break or exit from func"),
            },
            None => {},
        }*/
        // func_scope is destroyed
        panic!("EOF func w/o a return value");
    }
}

#[derive(Clone)]
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
        return &self.pars;
    }

    pub fn exec(&self, scope: &mut Scope, args: &Vec<Val>)
        -> Option<BlockAction>
    {
/*        // decl args as vars
        let mut proc_scref = ScopeRef::new();
        for (i, par) in self.pars.iter().enumerate() {
            do_declar(scope, &mut proc_scref, &par.1, &par.0);
            do_assign(scope, &par.1, &Expr::Const(args[i]));
        }
        // exec body
        // code similar to do_block
        match do_block(scope, &self.body) {
            Some(ba) => match ba {
                BlockAction::PcExit => {
                    scope.clean(&proc_scref);
                    return None;
                },
                _ => return Some(ba), // break or return
            },
            None => {},
        }
        scope.clean(&proc_scref);*/
        return None;
    }
}

pub type Block = Vec<Stmt>;

#[derive(Clone)]
pub enum Stmt
{
    Assign(String, Expr),
    Declar(String, Type),
    DecAss(String, Type, Expr),
    OperOn(String, BinOpcode, Expr),
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

    pub fn from_str_to_c(s: &str) -> Self
    {
        match s.chars().nth(3) {
            Some(c) => return Self::C(c),
            None => panic!("not valid char"),
        }
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
