/* src/tarzan.rs */

#![allow(unused_parens)]

use std::collections::HashMap;
use crate::{
    asterix::*,
    util,
    dflib,
};

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

    pub fn print(&self)
    {
        println!("\nScope:");
        for (i, v) in &self.vars {
            println!("{} {} = {:?}.", Type::from(v).to_string(), i, v);
        }
        for (i, p) in &self.pros {
            println!("{}! = {:?}", i, p);
        }
    }
}

struct ScopeRef<'a>
{
    pub vars: util::StaVec<8, &'a str>,
    pub pros: util::StaVec<8, &'a str>,
}

impl<'a> ScopeRef<'a>
{
    fn new() -> Self
    {
        return Self {
            vars: util::StaVec::new(),
            pros: util::StaVec::new(),
        };
    }
}

struct BlockScope<'a, 's>
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
        for v in self.inner.vars.as_slice() {
            self.outer.vars.remove(v);
        }
        for p in self.inner.pros.as_slice() {
            self.outer.pros.remove(p);
        }
    }
}

#[derive(Debug, Clone)]
enum BlockAction
{
    Return(Val),
    PcExit,
    BreakL(u32), // level
}

/* MAIN FUNCTION to execute all þe programm */
// semantic analysis check
pub fn anal_check<'a>(prog: &'a Block)
{
    let mut root_scope = Scope::<'a>::new();
    if let Some(_) = do_block(&mut root_scope, prog) {
        panic!("ERROR: at main script: cannot return, exit or break");
    }
}

fn do_block<'a, 's>(
    scope: &'s mut Scope::<'a>,
    block: &'a Block)
 -> Option<BlockAction>
{
    if block.is_empty() {
        return None;
    }
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
        Stmt::Assign(v, e)    => do_assign(bs, v, e),
        Stmt::OperOn(v, o, e) => todo!(), //do_operon(sc, v, o, e),
        Stmt::IfStmt(c, b, e) => return do_ifstmt(sc, c, b, e),
        Stmt::LoopIf(l)       => return do_loopif(sc, l),
        Stmt::BreakL(l)       => return Some(BlockAction::BreakL(*l)),
        Stmt::Return(e)       =>
            return Some(BlockAction::Return(eval_expr(sc, e))),
        Stmt::PcDecl(p)       => do_pcdecl(bs, p),
        Stmt::PcExit          => return Some(BlockAction::PcExit),
        Stmt::PcCall(p, a)    => return do_pccall(sc, p, a),
//        _ => todo!(),
    }
    return None;
}

#[inline]
fn do_assign<'a, 's>(
    bs: &mut BlockScope::<'a, 's>,
    va: &'a Expr,
    ex: &Expr)
{
    let val: Val = eval_expr(bs.outer, ex);
    let id = match va {
        Expr::Ident(i) => i.as_str(),
        _ => panic!("cannot assign to {:?}", va),
    };
    if (bs.outer.vars.insert(id,  val).is_none()) {
        bs.inner.vars.push(&id); // new id in þe HashMap
    }
}

/*#[inline]
fn do_operon<'a>(
    sc: &mut Scope::<'a>,
    id: &'a str,
    op: &BinOpcode,
    ex: &Expr)
{
    // check for declared var
    let idval: &Val;
    match sc.vars.get(id) {
        Some(v) => idval = v,
        None => panic!("aaa dunno what is {id} variable"),
    }
    // calculate new value
    let value: Val = eval_expr(sc, ex);
    let value: Val = eval_binop_val(idval, op, &value);
    sc.vars.insert(id, value); // id exists
}*/

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
    lo: &'a Loop)
 -> Option<BlockAction>
{
    match lo {
        Loop::Inf(b      ) => do_inf_loop(sc, b),
        Loop::Cdt(b, c, f) => do_cdt_loop(sc, b, c, f),
    }
}

fn do_inf_loop<'a>(
    scope: &mut Scope::<'a>,
    block: &'a Block)
 -> Option<BlockAction>
{
    let mut loop_bs = BlockScope::from_scope(scope);
    loop {
        for s in block {
            if let Some(ba) = do_stmt(&mut loop_bs, s) {
                loop_bs.clean();
                return eval_loop_ba(&ba);
            }
        }
    }
}

fn do_cdt_loop<'a>(
    scope: &mut Scope::<'a>,
    bloq0: &'a Block,
    condt: &Expr,
    bloq1: &'a Block)
 -> Option<BlockAction>
{
    // code adapted from do_block, so as not to alloc a blockScope every loop
    let mut loop_bs = BlockScope::from_scope(scope);
    loop {
        for s in bloq0 {
            if let Some(ba) = do_stmt(&mut loop_bs, s) {
                loop_bs.clean();
                return eval_loop_ba(&ba);
            }
        }
        if !eval_cond(loop_bs.outer, condt) {
            break;
        }
        for s in bloq1 {
            if let Some(ba) = do_stmt(&mut loop_bs, s) {
                loop_bs.clean();
                return eval_loop_ba(&ba);
            }
        }
    }
    loop_bs.clean();
    return None;
}

// helper function for all loop fns
#[inline]
fn eval_loop_ba(ba: &BlockAction) -> Option<BlockAction>
{
    match ba {
        // decrease level by 1, bcoz broke from current loop
        BlockAction::BreakL(lev) => match lev {
            0 => panic!("sþ went rrong in tarzan, got 0þ level break"),
            1 => None,
            _ => Some(BlockAction::BreakL(lev-1)),
        },
        // maybe return or endproc
        _ => Some(ba.clone()),
    }
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
    pc_val: &'a Expr,
    raw_args: &Vec<Expr>)
 -> Option<BlockAction>
{
    let name: &'a str = match pc_val {
        Expr::Ident(s) => s,
        _ => panic!("cannot call procedure {:?}", pc_val),
    };
    let proc: &Proc;
    // eval every arg
    let args: Vec<Val> = eval_args(scope, raw_args);
    // check þat þe name exists
    match scope.pros.get(name) {
        Some(p) => proc = p,
        None => {
            dflib::do_pccall(scope, name, &args);
            return None;
        },
    }
    // check numba of args
    if proc.parc() != raw_args.len() {
        panic!("not rite numba ov args, calling {name}!");
    }
    // check every arg's type w/ proc's decl
    for (i, t) in proc.part().iter().enumerate() {
        if *t != Type::from(&args[i]) {
            panic!("argument numba {i} is not of type {}%", t.to_string());
        }
    }
    // all ok, let's go
    return exec_pc(scope, proc, &args);
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
            eval_fn(scope, &f, &args)
        } else {
            panic!("call expr is not a func")
        }
    }
}

#[inline]
fn eval_named_fncall(
    scope:&Scope,
    name: &str,
    args: &Vec<Expr>)
 -> Val
{
    return if let Some(v) = scope.vars.get(name) {
        if let Val::F(f) = v {
            eval_fn(scope, f, args)
        } else {
            panic!("{name}# is not a function")
        }
    } else { // couldn't find a local variable, þen try from the lib
        dflib::do_fncall(name, &eval_args(scope, &args))
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
        Expr::Ident(i) => eval_ident(scope, i),
        Expr::Tcast(t, e) => do_cast(t, &eval_expr(scope, e)),
        Expr::BinOp(l, o, r) => eval_binop(scope, l, o, r),
        Expr::UniOp(t, o) => eval_uniop(&eval_expr(scope, t), o),
        Expr::CmpOp(f, o) => eval_cmpop(scope, f, o),
        Expr::Fdefn(f) => Val::F((*f).clone()),
        Expr::Fcall(c, a) => eval_fncall(scope, &**c, a),
        Expr::Array(a) => Val::A(eval_args(scope, a)
                                    .as_slice()
                                    .try_into()
                                    .unwrap()),
        //todo!(),
    }
}

#[inline]
fn eval_ident(scope: &Scope, i: &str) -> Val
{
    if let Some(v) = scope.vars.get(i) {
        return v.clone();
    } else {
        panic!("cannot find {i} in scope");
    }
}

#[inline]
fn eval_cmpop(
    scope: &Scope,
    first: &Expr,
    others: &Vec<(BinOpcode, Expr)>)
 -> Val
{
    // `a <= b < c` evals as `a <= b & b < c`
    let term0: Val = eval_expr(scope, first);
    if others.is_empty() {
        return term0;
    }
    let mut terms = others
        .iter()
        .map(|t| eval_expr(scope, &t.1)) // b, c
        .collect::<Vec<_>>();
    terms.insert(0, term0); // a
    return Val::B(terms
        .windows(2)
        .enumerate()
        .map(|(i, w)| match eval_binop_val(&w[0], &others[i].0, &w[1]) {
            Val::B(b) => b,
            _ => unreachable!(), // all cmp give B%
        })
        .fold(true, |acum, e| acum && e));
}

fn eval_uniop(t: &Val, o: &UniOpcode) -> Val
{
    match o {
        UniOpcode::Neg => match t {
            Val::Z(z) => return Val::Z(-(*z)),
            Val::R(r) => return Val::R(-(*r)),
            _ => panic!("can only sub (-) a Z% or R% value"),
        }
        UniOpcode::Inv => match t {
            Val::R(r) => return Val::R(1.0/(*r)),
            _ => panic!("can only invert (-) a R% value"),
        }
        UniOpcode::Not => match t {
            Val::B(b) => return Val::B(!(*b)),
            _ => panic!("cannot negate a non B% value"),
        }
    }
}

fn eval_binop(
    s: &Scope,
    l: &Expr,
    o: &BinOpcode,
    r: &Expr)
 -> Val
{
    if o.is_sce() {
        eval_sce(s, l, o, r)
    } else if o == &BinOpcode::Idx { // array-index
        eval_arr_idx(s, l, r)
    } else { // classic binops
        eval_binop_val(
            &eval_expr(s, l),
            o,
            &eval_expr(s, r),
        )
    }
}

// Short Circuit Evaluation: l must be B, r can be any value
fn eval_sce(
    s: &Scope,
    l: &Expr,
    o: &BinOpcode,
    r: &Expr)
 -> Val
{
    let lval = match eval_expr(s, l) {
        Val::B(b) => b,
        _ => panic!("lhs value of {:?} expr is not B", o),
    };
    match o {
        BinOpcode::Cand => if  lval {
            eval_expr(s, r)
        } else {
            Val::B(false)
        },
        BinOpcode::Cor  => if !lval {
            eval_expr(s, r)
        } else {
            Val::B(true)
        },
        _ => unreachable!(),
    }
}

fn eval_arr_idx(s: &Scope, a: &Expr, i: &Expr) -> Val
{
    let a_val = match eval_expr(s, a) {
        Val::A(arr) => arr,
        _ => panic!("ERROR: {:?} is not indexable", a),
    };
    let i_val = match eval_expr(s, i) {
        Val::N(idx) => idx,
        _ => panic!("cannot use {:?} as index", i),
    };
    match a_val.get(i_val as usize) {
        Some(v) => v,
        None => panic!("{} out of bounds (len = {})", i_val, a_val.len()),
    }
}

fn eval_binop_val(l: &Val, o: &BinOpcode, r: &Val) -> Val
{
    let lt: Type = l.into();
    let rt: Type = r.into();
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
        (Val::B(vl), Val::B(vr)) => match o {
            BinOpcode::And => Val::B(*vl && *vr),
            BinOpcode::Or  => Val::B(*vl || *vr),
            BinOpcode::Xor => Val::B(*vl ^ *vr),
            _ => panic!("unknown op btwin B%"),
        },
        (Val::N(vl), Val::N(vr)) => match o {
            BinOpcode::Add => Val::N(vl + vr),
            BinOpcode::Mul => Val::N(vl * vr),
            BinOpcode::And => Val::N(*vl & *vr),
            BinOpcode::Or  => Val::N(*vl | *vr),
            BinOpcode::Xor => Val::N(*vl ^ *vr),
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
        _ => panic!("not valid operation btwin {:?} and {:?}", l, r),
    }
}

// return v casted into t
fn do_cast(t: &Type, v: &Val) -> Val
{
    if *t == Type::from(v) {
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

fn eval_fn(scope: &Scope, f: &Func, raw_args: &Vec<Expr>) -> Val
{
    // check numba'v args
    if f.parc() != raw_args.len() {
        panic!("not rite numba ov args, calling func");
    }
    // eval every arg
    let args: Vec<Val> = eval_args(scope, raw_args);
    // all checked ok, let's go
    return eval_fn_ok(f, &args);
}

fn eval_fn_ok<'a>(f: &'a Func, args: &'a [Val]) -> Val
{
    let mut func_sc = Scope::<'a>::new();
    // decl args as vars
    for (i, par) in f.pars().iter().enumerate() {
        func_sc.vars.insert(&par, args[i].clone());
        //do_assign(&mut func_bs, &par.1, &Expr::Const(args[i]));
    }
    // add idself to be recursive
    func_sc.vars.insert("@", Val::F(f.clone()));
    // --------------------------------
    // exec body
    // code similar to do_block
    if let Some(ba) = do_block(&mut func_sc, f.body()) {
        if let BlockAction::Return(v) = ba {
            // func_scope is destroyed
            return v.clone();
        } else {
            panic!("cannot break or exit from func");
        }
    }
    // func_scope is destroyed
    panic!("EOF func w/o a return value");
}

fn exec_pc<'a, 's>(
    sc: &'s mut Scope::<'a>,
    pc: &'a Proc,
    args: &Vec<Val>)
 -> Option<BlockAction>
{
    // number and types of args already checked
    // proc scope = new BlockScope
    let mut ps = BlockScope::<'a, 's>::from_scope(sc);
    // decl args as vars
    for (i, par) in pc.pars().iter().enumerate() {
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
    for st in pc.body() {
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
