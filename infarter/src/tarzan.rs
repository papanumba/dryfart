/* src/tarzan.rs */

#![allow(dead_code, unused_variables, unused_parens)]

/*use std::{
    collections::HashMap
};*/
use std::rc::Rc;
use crate::{
    asterix::*,
    util,
    dflib,
};


/* MAIN FUNCTION to execute all þe programm */
pub fn exec_main(prog: &Block)
{
    if let Some(_) = exec_block(prog) {
        panic!("ERROR: at main script: cannot return, exit or break");
    }
}

fn exec_block<'a>(b: &'a Block) -> Option<BlockAction>
{
    let mut scope = Scope::<'a>::new();
    return scope.do_block(b);
}

pub struct Scope<'a>
{
    vars: util::VecMap<&'a str, Val>,
    callee: Option<Val>, // main, func or proc
}

impl<'a> Scope<'a>
{
    fn new() -> Self
    {
        Self { vars: util::VecMap::new(), callee: None }
    }

    pub fn with_callee(c: Val) -> Self
    {
        Self { vars: util::VecMap::new(), callee: Some(c) }
    }

    pub fn print(&self)
    {
        println!("\nScope:");
        for (i, v) in self.vars.as_slice() {
            println!("{} {} = {:?}.", Type::from(v).to_string(), i, v);
        }
    }

    #[inline]
    pub fn clean(&mut self, pre: usize)
    {
        self.vars.trunc(pre);
    }

    fn declar(&mut self, v: &'a str, e: Val)
    {
        self.vars.set(&v, e);
    }

    /******** executing functions ********/

    pub fn do_block(&mut self, block: &'a Block) -> Option<BlockAction>
    {
        if block.is_empty() { // optimizing
            return None;
        }
        let presize = self.vars.size();
        let action = self.no_env_block(block);
        self.clean(presize);
        return action;
    }

    #[inline]
    fn no_env_block(&mut self, block: &'a Block) -> Option<BlockAction>
    {
        for s in block {
            if let Some(ba) = self.do_stmt(s) {
                return Some(ba);
            }
        }
        return None;
    }

    fn do_stmt(&mut self, s: &'a Stmt) -> Option<BlockAction>
    {
        match s {
            Stmt::Assign(v, e)    => self.do_assign(v, e),
//            Stmt::OperOn(v, o, e) => todo!(), //do_operon(sc, v, o, e),
            Stmt::IfStmt(c, b, e) => return self.do_ifstmt(c, b, e),
            Stmt::LoopIf(l)       => return self.do_loopif(l),
            Stmt::BreakL(l)       => return Some(BlockAction::Brk(*l)),
            Stmt::Return(e)       => return Some(BlockAction::Ret(
                self.eval_expr(e)
            )),
            Stmt::PcExit          => return Some(BlockAction::End),
            Stmt::PcCall(p, a)    => self.do_pccall(p, a),
            _ => todo!(),
        }
        return None;
    }

    #[inline]
    fn do_assign(&mut self, ad: &'a Expr, ex: &Expr)
    {
        match ad {
            Expr::Ident(i) =>
                self.do_var_ass(i, ex),
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                self.do_arr_ass(a, i, ex),
            Expr::TblFd(t, f) =>
                self.do_tbl_ass(t, f, ex),
            _ => panic!("cannot assign to {:?}", ad),
        }
    }

    // v = e.
    #[inline]
    fn do_var_ass(&mut self, v: &'a str, e: &Expr)
    {
        self.declar(v, self.eval_expr(e));
    }

    // a_i = e.
    fn do_arr_ass(&self, a: &Expr, i: &Expr, e: &Expr)
    {
        if let Val::A(arr) = self.eval_expr(a) {
            if let Val::N(idx) = self.eval_expr(i) {
                let e_val = self.eval_expr(e);
                arr.borrow_mut().try_set(idx as usize, e_val).unwrap();
            } else {
                panic!("not an index");
            }
        } else {
            panic!("not indexable");
        }
    }

    // t$f = e.
    fn do_tbl_ass(&self, t: &Expr, f: &str, e: &Expr)
    {
        if let Val::T(t) = self.eval_expr(t) {
            let e_val = self.eval_expr(e);
            t.borrow_mut().set(f.to_string(), e_val);
        } else {
            panic!("not a table");
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

    // helper for do_ifstmt & do_loopif
    #[inline]
    fn eval_cond(&self, cd: &Expr) -> bool
    {
        match self.eval_expr(cd) {
            Val::B(b) => b,
            _ => panic!("condition is not B%"),
        }
    }

    fn do_ifstmt(
        &mut self,
        cd: &Expr,
        bl: &'a Block,
        eb: &'a Option<Block>)
     -> Option<BlockAction>
    {
         if self.eval_cond(cd) {
            self.do_block(bl)
         } else {
            match eb {
                Some(b) => self.do_block(b),
                None => None,
            }
        }
    }

    fn do_loopif(&mut self, lo: &'a Loop) -> Option<BlockAction>
    {
        let pre = self.vars.size();
        //self.enter_loop(lo); // preset Vs
        let ba = match lo {
            Loop::Inf(b      ) => self.do_inf_loop(b),
            Loop::Cdt(b, c, f) => self.do_cdt_loop(b, c, f),
        };
        self.clean(pre);
        match ba {
            Some(b) => b.from_loop(),
            None => None,
        }
    }

    fn do_inf_loop(&mut self, block: &'a Block) -> Option<BlockAction>
    {
        loop {
            if let Some(ba) = self.no_env_block(block) {
                // TODO future check "continue 0"
                return Some(ba);
            }
        }
    }

    fn do_cdt_loop(
        &mut self,
        blok0: &'a Block,
        condt: &Expr,
        blok1: &'a Block)
     -> Option<BlockAction>
    {
        loop {
            if let Some(ba) = self.no_env_block(blok0) {
                // TODO future check "continue 0"
                return Some(ba);
            }
            if !self.eval_cond(condt) {
                break;
            }
            if let Some(ba) = self.no_env_block(blok1) {
                // TODO future check "continue 0"
                return Some(ba);
            }
        }
        return None;
    }

    fn do_pccall(&mut self, p: &Expr, a: &[Expr])
    {
        let pc_val = self.eval_expr(p);
        if let Val::P(p) = pc_val.clone() {
            if p.arity() != a.len() {
                panic!("not correct arity ({}) calling {:?}", a.len(), p);
            }
            let args = self.eval_args(a);
            match &*p {
                Proc::Nat(n) => n.exec(&args),
                Proc::Usr(u) => exec_usr_proc(pc_val.clone(), &u, args),
            }
        } else {
            panic!("cannot call procedure {:?}", pc_val);
        }
    }

    #[inline]
    fn eval_args(&self, a: &[Expr]) -> Vec<Val>
    {
        a.iter()
         .map(|b| self.eval_expr(b))
         .collect()
    }

    fn eval_expr(&self, e: &Expr) -> Val
    {
        match e {
            Expr::Const(c)       => c.clone(),
            Expr::Ident(i)       => self.eval_ident(i),
            Expr::Tcast(t, e)    => do_cast(t, &self.eval_expr(e)),
            Expr::BinOp(l, o, r) => self.eval_binop(l, o, r),
            Expr::UniOp(t, o)    => eval_uniop(&self.eval_expr(t), o),
            Expr::CmpOp(f, o)    => self.eval_cmpop(f, o),
            Expr::FnDef(s)       => self.make_func(s),
            Expr::Fcall(c, a)    => self.eval_fcall(c, a),
            Expr::RecFn          => self.get_rec_f(),
            Expr::PcDef(s)       => self.make_proc(s),
            Expr::RecPc          => self.get_rec_p(),
            Expr::Array(a)       => self.eval_array(a),
            Expr::Table(v)       => self.eval_table(v),
            Expr::TblFd(e, f)    => self.eval_tblfd(e, f),
            _ => todo!("{:?}", e),
        }
    }

    #[inline]
    fn eval_ident(&self, i: &str) -> Val
    {
        // try variable
        if let Some(v) = self.vars.get(&i) {
            return v.clone();
        }
        // try built in
        if let Some(v) = dflib::get(i) {
            return v.clone();
        }
        panic!("cannot find {i} in scope");
    }

    #[inline]
    fn eval_cmpop(&self, first: &Expr, others: &[(BinOpcode, Expr)]) -> Val
    {
        // `a <= b < c` evals as `a <= b & b < c` but only eval `b` once
        let term0: Val = self.eval_expr(first);
        if others.is_empty() {
            return term0;
        }
        let mut terms = others
            .iter()
            .map(|t| self.eval_expr(&t.1)) // b, c
            .collect::<Vec<_>>();
        terms.insert(0, term0); // a
        return Val::B(terms
            .windows(2)
            .enumerate()
            .map(|(i, w)| match eval_binop_val(&w[0], &others[i].0, &w[1]) {
                Val::B(b) => b,
                _ => unreachable!(), // all cmp give B%
            })
            .fold(true, |acum, e| acum && e)
        );
    }

    #[inline]
    fn eval_fcall(&self, f: &Expr, a: &[Expr]) -> Val
    {
        let fn_val = self.eval_expr(f);
        if let Val::F(f) = fn_val.clone() {
            if f.arity() != a.len() {
                panic!("not correct arity ({}) calling {:?}", a.len(), f);
            }
            let args = self.eval_args(a);
            match &*f {
                Func::Usr(u) => eval_usr_func(fn_val.clone(), &u, args),
            }
        } else {
            panic!("cannot call function {:?}", fn_val);
        }
    }

    fn eval_binop(&self, l: &Expr, o: &BinOpcode, r: &Expr) -> Val
    {
        if o.is_sce() { // short circuit
            self.eval_sce(l, o, r)
        } else if o == &BinOpcode::Idx { // array-index
            self.eval_arr_idx(l, r)
        } else { // classic binops
            eval_binop_val(
                &self.eval_expr(l),
                o,
                &self.eval_expr(r),
            )
        }
    }

    // Short Circuit Evaluation: l must be B, r can be any value
    fn eval_sce(&self, l: &Expr, o: &BinOpcode, r: &Expr) -> Val
    {
        let lval = match self.eval_expr(l) {
            Val::B(b) => b,
            _ => panic!("lhs value of {:?} expr is not B", o),
        };
        match o {
            BinOpcode::Cand => if lval {
                self.eval_expr(r)
            } else {
                Val::B(false)
            },
            BinOpcode::Cor => if !lval {
                self.eval_expr(r)
            } else {
                Val::B(true)
            },
            _ => unreachable!(),
        }
    }

    fn eval_arr_idx(&self, a: &Expr, i: &Expr) -> Val
    {
        let a_val = match self.eval_expr(a) {
            Val::A(arr) => arr,
            _ => panic!("ERROR: {:?} is not indexable", a),
        };
        let i_val = match self.eval_expr(i) {
            Val::N(idx) => idx,
            _ => panic!("cannot use {:?} as index", i),
        };
        let a_ref = a_val.borrow();
        match a_ref.get(i_val as usize) {
            Some(v) => v,
            None => panic!("{} out of bounds (len = {})",
                i_val, a_ref.len()),
        }
    }

    #[inline]
    fn make_func(&self, subr: &Rc<Subr>) -> Val
    {
        Val::new_usr_fn(subr.clone())
    }

    #[inline]
    fn make_proc(&self, subr: &Rc<Subr>) -> Val
    {
        Val::new_usr_pc(subr.clone())
    }

    #[inline]
    fn get_rec_f(&self) -> Val
    {
        if let Some(v) = &self.callee {
            if Type::from(v) == Type::F {
                return v.clone();
            }
        }
        panic!("cannot refer #@ not inside a func");
    }

    #[inline]
    fn get_rec_p(&self) -> Val
    {
        if let Some(v) = &self.callee {
            if Type::from(v) == Type::P {
                return v.clone();
            }
        }
        panic!("cannot refer !@ not inside a proc");
    }

    #[inline]
    fn eval_array(&self, a: &[Expr]) -> Val
    {
        Val::from_array(self.eval_args(a)
            .as_slice()
            .try_into()
            .unwrap()
        )
    }

    fn eval_table(&self, e: &[(String, Expr)]) -> Val
    {
        let mut t = Table::new();
        // TODO eke $@
        for (k, ve) in e {
            let v = self.eval_expr(ve);
            t.set(k.clone(), v);
        }
        return Val::from_table(t);
    }

    fn eval_tblfd(&self, t: &Expr, f: &String) -> Val
    {
        let tbl = self.eval_expr(t);
        if let Val::T(trc) = tbl {
            match trc.borrow().get(f) {
                Some(v) => return v.clone(),
                None => panic!("{:?} table hasn't ${}", t, f),
            }
        } else {
            panic!("{:?} is not a table", tbl);
        }
    }

/*    fn eval_fn(scope: &Scope, f: &Func, raw_args: &Vec<Expr>) -> Val
    {
        // check numba'v args
        if f.parc() != raw_args.len() {
            panic!("not rite numba ov args, calling func");
        }
        // eval every arg
        let args: Vec<Val> = eval_args(scope, raw_args);
        // all checked ok, let's go
        return eval_fn_ok(f, &args);
    }*/

/*    fn eval_fn_ok<'a>(f: &'a Func, args: &'a [Val]) -> Val
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
            if let BlockAction::Ret(v) = ba {
                // func_scope is destroyed
                return v.clone();
            } else {
                panic!("cannot break or exit from func");
            }
        }
        // func_scope is destroyed
        panic!("EOF func w/o a return value");
    }*/

    /*fn exec_pc<'a, 's>(
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
    */
}

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
            Val::N(n) => return Val::N(!(*n)),
            _ => panic!("~ is only for B% & N% values"),
        }
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

//fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result

/*
** þe next 2 functions get rec_val coz it may be used when !@ xor #@
** rec_val is þe same as Val::P(Proc::Nat(proc)), but can be Rc-cloned
*/

pub fn exec_usr_proc(rec_val: Val, subr: &Subr, mut args: Vec<Val>)
{
    let mut proc_scope = Scope::<'_>::with_callee(rec_val);
    for name in subr.pars.iter().rev() {
        let val = args.pop().unwrap(); // already checked arity
        proc_scope.declar(name, val);
    }
    if let Some(ba) = proc_scope.do_block(&subr.body) {
        match ba {
            BlockAction::End => return,
            _ => panic!("cannot return or break from proc"),
        }
    }
}

pub fn eval_usr_func(rec_val: Val, subr: &Subr, mut args: Vec<Val>) -> Val
{
    let mut func_scope = Scope::<'_>::with_callee(rec_val);
    for name in subr.pars.iter().rev() {
        let val = args.pop().unwrap(); // already checked arity
        func_scope.declar(name, val);
    }
    if let Some(ba) = func_scope.do_block(&subr.body) {
        match ba {
            BlockAction::Ret(v) => return v,
            _ => panic!("cannot return or break from func"),
        }
    } else {
        panic!("ended function w/o returning a value");
    }
}

#[derive(Debug, Clone)]
pub enum BlockAction
{
    Ret(Val),
    End,
    Brk(u32), // level
}

impl BlockAction
{
    #[inline]
    fn from_loop(&self) -> Option<Self>
    {
        match self {
            // decrease level by 1, bcoz broke from current loop
            BlockAction::Brk(lev) => match lev {
                0 => None,
                _ => Some(Self::Brk(lev-1)),
            },
            // maybe return or endproc
            _ => Some(self.clone()),
        }
    }
}
