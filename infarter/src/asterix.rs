/* ASTerix */

use std::collections::HashMap;
use crate::lib_procs::*;
use crate::lib_funcs::*;
use crate::util;

pub struct Scope<'a>
{
    pub vars: HashMap<&'a str, Val>,
    pub pros: HashMap<&'a str, &'a Proc>,
}

impl<'a> Scope<'a>
{
    fn new() -> Self
    {
        return Self {
            vars: HashMap::new(),
            pros: HashMap::new(),
        };
    }

    fn print(&self)
    {
        println!("\nScope:");
        for (i, v) in &self.vars {
            println!("{}% {} = {:?}.", v.as_type().to_string(), i, v);
        }
        for (i, p) in &self.pros {
            println!("{}!", i);
        }
    }
}

pub struct ScopeRef<'a>
{
    pub vars: util::StaVec<8, &'a str>,
    pub pros: util::StaVec<8, &'a str>,
}

impl<'a> ScopeRef<'a>
{
    pub fn new() -> Self
    {
        return Self {
            vars: util::StaVec::new(),
            pros: util::StaVec::new(),
        };
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
//        self.outer.print();
        for v in self.inner.vars.as_slice() {
            self.outer.vars.remove(v);
        }
        for p in self.inner.pros.as_slice() {
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
//    root_scope.print();
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
        if let Some(ba) = do_stmt(&mut blocks_scope, s) {
            blocks_scope.clean();
            return Some(ba);
        }
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
        Stmt::Return(e)       =>
            return Some(BlockAction::Return(eval_expr(sc, e))),
        Stmt::PcDecl(p)       => do_pcdecl(bs, p),
        Stmt::PcExit          => return Some(BlockAction::PcExit),
        Stmt::PcCall(n, a)    => return do_pccall(sc, n, a),
//        _ => todo!(),
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
//    println!("assigning {id} = {:?}", val);
    match bs.outer.vars.insert(id,  val) {
        None => bs.inner.vars.push(&id), // new id in þe HashMap
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
        Some(v) => idval = (*v).clone(),
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
            if let Some(ba) = do_stmt(&mut loop_bs, st) {
                loop_bs.clean();
                return match ba {
                    BlockAction::BreakL => None,
                    _ => Some(ba),
                };
            }
        }
    }
    loop_bs.clean();
    return None;
}

#[inline]
fn do_pcdecl<'a, 's>(
    bs: &mut BlockScope::<'a, 's>,
    pc: &'a Proc)
{
    let name: &str = pc.name();
    if !bs.outer.pros.contains_key(name) {
        bs.outer.pros.insert(name, pc);
        bs.inner.pros.push(&name);
    } else {
        panic!("procedure {name} already made");
    }
}

fn do_pccall<'a>(
    scope: &mut Scope::<'a>,
    name: &'a str,
    raw_args: &Vec<Expr>)
 -> Option<BlockAction>
{
    let proc: &Proc;
    // eval every arg
    let args: Vec<Val> = eval_args(scope, raw_args);
    // check þat þe name exists
    match scope.pros.get(name) {
        Some(p) => proc = p,
        None => {
            do_lib_pccall(scope, name, &args);
            return None;
        },
    }
    // check numba of args
    if proc.parc() != raw_args.len() {
        panic!("not rite numba ov args, calling {name}!");
    }
    // check every arg's type w/ proc's decl
    for (i, t) in proc.part().iter().enumerate() {
        if *t != args[i].as_type() {
            panic!("argument numba {i} is not of type {}%", t.to_string());
        }
    }
    // all ok, let's go
    return proc.exec(scope, &args);
}

fn eval_fncall(
    scope:&Scope,
    call: &Expr,
    args: &Vec<Expr>)
 -> Val
{
    // check if call is simply an Ident(&str)
    return if let Expr::Ident(i) = call {
        eval_named_fncall(scope, i, args)
    } else {
        // eval þe more complex call Expr
        if let Val::F(f) = eval_expr(scope, call) {
            f.eval(scope, &args)
        } else {
            panic!("call expr is not a func")
        }
    }

}

fn eval_named_fncall(
    scope:&Scope,
    name: &str,
    args: &Vec<Expr>)
 -> Val
{
    return if let Some(v) = scope.vars.get(name) {
        if let Val::F(f) = v {
            f.eval(scope, args)
        } else {
            panic!("{name}# is not a function")
        }
    } else { // couldn't find a local variable, þen try from the lib
        do_lib_fncall(name, &eval_args(scope, &args))
    };
}

#[inline]
fn eval_args(scope: &Scope, a: &Vec<Expr>) -> Vec<Val>
{
    return a
        .iter()
        .map(|b| eval_expr(scope, b))
        .collect();
}

fn eval_expr(scope: &Scope, e: &Expr) -> Val
{
    match e {
        Expr::Const(c) => (*c).clone(),
        Expr::Ident(i) => (*scope.vars.get(i.as_str()).unwrap()).clone(),
        Expr::Tcast(t, e) => do_cast(t, &eval_expr(scope, e)),
        Expr::BinOp(l, o, r) => eval_binop(
            &eval_expr(scope, l),
            o,
            &eval_expr(scope, r)
        ),
        Expr::UniOp(t, o) => eval_uniop(&eval_expr(scope, t), o),
        Expr::Fdefn(f) => Val::F((*f).clone()),
        Expr::Fcall(c, a) => eval_fncall(scope, &**c, a),
        Expr::ArrEl(a, i) => try_arr_el(
            &eval_expr(scope, a),
            &eval_expr(scope, i)
        ),
        Expr::Array(a) => Val::A(Array::try_new(&a
            .iter()
            .map(|e| eval_expr(scope, e))
            .collect::<Vec<Val>>()
        )),
        //_ => todo!(),
    }
}

fn eval_uniop(t: &Val, o: &UniOpcode) -> Val
{
    match o {
        UniOpcode::Sub => match t {
            Val::Z(z) => return Val::Z(-(*z)),
            Val::R(r) => return Val::R(-(*r)),
            _ => panic!("can only sub (-) a Z% or R% value"),
        }
        UniOpcode::Inv => match t {
            Val::R(r) => return Val::R(1.0/(*r)),
            _ => panic!("can only invert (-) a R% value"),
        }
        UniOpcode::Neg => match t {
            Val::B(b) => return Val::B(!(*b)),
            _ => panic!("cannot negate a non B% value"),
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
            _ => panic!("not valid operation btwin N%"),
        },
        (Val::Z(vl), Val::Z(vr)) => match o {
            BinOpcode::Add => Val::Z(vl + vr),
            BinOpcode::Sub => Val::Z(vl - vr),
            BinOpcode::Mul => Val::Z(vl * vr),
            _ => panic!("not valid operation btwin Z%"),
        },
        (Val::R(vl), Val::R(vr)) => match o {
            BinOpcode::Add => Val::R(vl + vr),
            BinOpcode::Sub => Val::R(vl - vr),
            BinOpcode::Mul => Val::R(vl * vr),
            BinOpcode::Div => Val::R(vl / vr),
            _ => panic!("not valid operation btwin R%"),
        },
        (Val::B(vl), Val::B(vr)) => match o {
            BinOpcode::And => Val::B(*vl && *vr),
            BinOpcode::Or  => Val::B(*vl || *vr),
            _ => panic!("unknown op btwin B%"),
        },
        _ => panic!("not valid operation btwin {:?} and {:?}", l, r),
    }
}

// return v casted into t
fn do_cast(t: &Type, v: &Val) -> Val
{
    if v.as_type() == *t {
        return (*v).clone();
    }
    return match (v, t) {
        (Val::N(n), Type::Z) => Val::Z(*n as i32),
        (Val::Z(z), Type::R) => Val::R(*z as f32),
        (Val::N(n), Type::R) => Val::R(*n as f32),
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
    pars: Vec<(Type, String)>,
    body: Block,
    rett: Type,
}

impl Func
{
    pub fn new(
        p: &Vec<(Type, String)>,
        b: &Block,
        r: &Type)
     -> Self
    {
        // check uniques in p
        let mut p2: Vec<&str> = p
            .iter()
            .map(|pair| (pair.1).as_str())
            .collect();
        p2.sort();
        p2.dedup();
        if p2.len() != p.len() {
            panic!("duplicate parameters in decl of a func");
        }
        return Self {
            pars: (*p).clone(),
            body: (*b).clone(),
            rett: (*r).clone(),
        };
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

    pub fn get_type(&self) -> Type
    {
        return Type::F(
            Box::new(self.rett.clone()),
            self.pars
                .iter()
                .map(|pair| pair.0.clone())
                .collect(),
        );
    }

    pub fn eval(&self, scope: &Scope, raw_args: &Vec<Expr>) -> Val
    {
        // check numba'v args
        if self.parc() != raw_args.len() {
            panic!("not rite numba ov args, calling func");
        }
        // eval every arg
        let args: Vec<Val> = eval_args(scope, raw_args);
        // check every arg's type w/ func's decl
        for (i, t) in self.part().iter().enumerate() {
            if *t != args[i].as_type() {
                panic!("argument numba {i} is not of type {}%", t.to_string());
            }
        }
        // all checked ok, let's go
        return self.eval_ok(&args);
    }

    fn eval_ok<'a>(&'a self, args: &'a Vec<Val>) -> Val
    {
        let mut func_sc = Scope::<'a>::new();
        // decl args as vars
        for (i, par) in self.pars.iter().enumerate() {
            func_sc.vars.insert(&par.1, args[i].clone());
            //do_assign(&mut func_bs, &par.1, &Expr::Const(args[i]));
        }
        // add idself to be recursive
        func_sc.vars.insert("@", Val::F(self.clone()));
        // --------------------------------
        // exec body
        // code similar to do_block
        if let Some(ba) = do_block(&mut func_sc, &self.body) {
            match ba {
                BlockAction::Return(v) => {
                    // func_scope is destroyed
                    if v.as_type() != self.rett {
                        panic!("return value is not of type {}",
                            self.rett.to_string());
                    } else {
                        return v.clone();
                    }
                },
                _ => panic!("cannot break or exit from func"),
            }
        }
        // func_scope is destroyed
        panic!("EOF func w/o a return value");
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
        write!(f, "{}#...", self.rett.to_string())
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

    pub fn exec<'a, 's>(
        &'a self,
        sc: &'s mut Scope::<'a>,
        args: &Vec<Val>)
     -> Option<BlockAction>
    {
        // number and types of args already checked
        // proc scope = new BlockScope
        let mut ps = BlockScope::<'a, 's>::from_scope(sc);
        // decl args as vars
        for (i, par) in self.pars.iter().enumerate() {
            let name = par.1.as_str();
            if !ps.outer.vars.contains_key(name) {
                ps.outer.vars.insert(name, args[i].clone());
                ps.inner.vars.push(&name);
            } else {
                panic!("arg {name} already made");
            }
        }
        // exec body
        // code similar to do_loopif
        for st in &self.body {
            if let Some(ba) = do_stmt(&mut ps, st) {
                ps.clean();
                return match ba {
                    BlockAction::PcExit => None,
                    _ => Some(ba),
                };
            }
        }
        ps.clean();
        return None;
    }
}

pub type Block = Vec<Stmt>;

#[derive(Clone)]
pub enum Stmt
{
    Assign(String, Expr),
    OperOn(String, BinOpcode, Expr),
    IfStmt(Expr, Block, Option<Block>), // cond, main block, else block
    LoopIf(Expr, Block),
    BreakL,
    Return(Expr),
    PcDecl(Proc),
    PcExit,
    PcCall(String, Vec<Expr>),
}

#[derive(Clone)]
pub enum Expr
{
    Const(Val),
    Ident(String),
    Tcast(Type, Box<Expr>),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UniOp(Box<Expr>, UniOpcode),
    Fdefn(Func),
    Fcall(Box<Expr>, Vec<Expr>),
    ArrEl(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Type
{
    B,  // bool
    C,  // char
    N,  // natural
    Z,  // zahl
    R,  // real
    A(Box<Type>),  // array
    F(Box<Type>, Vec<Type>), // func
}

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
            _ => todo!(),
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
            Self::A(_) => "A",
            Self::F(..) => "F",
            //_ => todo!(),
        });
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
    pub fn try_new(vals: &[Val]) -> Self
    {
        if vals.is_empty() {
            panic!("empty array");
        }
        // set self.typ as þe type of þe 1st element
        // þen compare to it while appending new elems
        let val0_type: Type = vals[0].as_type();
        let vals_len = vals.len();
        let mut res = match val0_type {
            Type::B => Self::B(Vec::<bool>::with_capacity(vals_len)),
            Type::C => Self::C(Vec::<char>::with_capacity(vals_len)),
            Type::N => Self::N(Vec::<u32>::with_capacity(vals_len)),
            Type::Z => Self::Z(Vec::<i32>::with_capacity(vals_len)),
            Type::R => Self::R(Vec::<f32>::with_capacity(vals_len)),
            _ => todo!(),
        };
        for v in vals {
            res.try_push(&v);
        }
        return res;
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

    pub fn from_str(s: &str) -> Self
    {
        if s.len() < 3 {
            panic!("trying to make string from str too short");
        }
        let last: usize = (s.len() as isize - 1) as usize;
        return Self::C(
            Self::replace_esc_seq(&s[1..last])
                .as_str()
                .chars()
                .collect(),
        );
    }

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

    pub fn get_type(&self) -> Type
    {
        return match self {
            Self::B(_) => Type::B,
            Self::C(_) => Type::C,
            Self::N(_) => Type::N,
            Self::Z(_) => Type::Z,
            Self::R(_) => Type::R,
        };
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

impl std::fmt::Display for Array
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // special case for strings {C%}
        match self {
            Self::C(a) => {
                for c in a { write!(f, "{c}")?; }; return Ok(());},
            _ => {},
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

fn try_arr_el(a: &Val, i: &Val) -> Val
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
    pub fn as_type(&self) -> Type
    {
        return match self {
            Self::B(_) => Type::B,
            Self::C(_) => Type::C,
            Self::N(_) => Type::N,
            Self::Z(_) => Type::Z,
            Self::R(_) => Type::R,
            Self::A(a) => Type::A(Box::new(a.get_type())),
            Self::F(f) => f.get_type(),
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UniOpcode { Sub, Inv, Neg }
