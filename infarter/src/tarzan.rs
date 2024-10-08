/* tarzan.rs */

use std::rc::Rc;
use crate::{
    asterix::*,
    util,
    util::{MutRc, DfStr},
};

/* MAIN FUNCTION to execute all þe programm */
pub fn exec_main(prog: &Block)
{
    if exec_block(prog).is_some() {
        panic!("ERROR: at main script: cannot return, exit or break");
    }
}

macro_rules! do_loop_block {
    ($zelf:expr, $block:expr) => {
        if let Some(ba) = $zelf.no_env_block($block) {
            let BlockAction::Loo(0, is_again) = ba else {
                return ba.exiting_loop();
            };
            if is_again {
                continue;
            } else {
                break;
            }
        }
    }
}

fn exec_block(b: &Block) -> Option<BlockAction>
{
    let mut scope = Scope::new();
    return scope.do_block(b);
}

pub struct Scope
{
    vars: util::VecMap<Rc<DfStr>, Val>,
    callee: Option<Val>, // main (None), func or proc
}

impl Scope
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
            println!("{} {} = {:?}.", Type::from(v), i, v);
        }
    }

    #[inline]
    pub fn clean(&mut self, pre: usize)
    {
        self.vars.trunc(pre);
    }

    fn declar(&mut self, v: &Rc<DfStr>, e: Val)
    {
        self.vars.set(v.clone(), e);
    }

    /******** executing functions ********/

    pub fn do_block(&mut self, block: &Block) -> Option<BlockAction>
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
    fn no_env_block(&mut self, block: &Block) -> Option<BlockAction>
    {
        for s in block {
            if let Some(ba) = self.do_stmt(s) {
                return Some(ba);
            }
        }
        return None;
    }

    fn do_stmt(&mut self, s: &Stmt) -> Option<BlockAction>
    {
        match s {
            Stmt::Assign(v, e)    => self.do_assign(v, e),
            Stmt::OperOn(l, o, e) => self.do_operon(l, o, e),
            Stmt::IfElse(i, o, e) => return self.do_ifelse(i, o, e),
            Stmt::Switch(..)      => todo!(),
            Stmt::LoopIf(l)       => return self.do_loopif(l),
            Stmt::AgainL(l)       => return Some(BlockAction::Loo(*l, true)),
            Stmt::BreakL(l)       => return Some(BlockAction::Loo(*l, false)),
            Stmt::Return(e)       => return Some(BlockAction::Ret(
                self.eval_expr(e)
            )),
            Stmt::PcExit          => return Some(BlockAction::End),
            Stmt::PcCall(p, a)    => self.do_pccall(p, a),
            Stmt::TbPCal(..)      => todo!(),
        }
        return None;
    }

    #[inline]
    fn do_assign(&mut self, ad: &Expr, ex: &Expr)
    {
        match ad {
            Expr::Ident(i) =>
                self.do_var_ass(i, ex),
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                self.do_arr_ass(a, i, ex),
            Expr::TblFd(t, f) =>
                self.do_tbl_ass(t, f, ex),
            _ => panic!("cannot assign to {ad:?}"),
        }
    }

    // v = e.
    #[inline]
    fn do_var_ass(&mut self, v: &Rc<DfStr>, e: &Expr)
    {
        self.declar(v, self.eval_expr(e));
    }

    // a_i = e.
    fn do_arr_ass(&self, a: &Expr, i: &Expr, e: &Expr)
    {
        let Val::A(arr) = self.eval_expr(a) else {
            panic!("not indexable");
        };
        let idx: u32 = match self.eval_expr(i) {
            Val::N(n) => n,
            Val::Z(z) => u32::try_from(z)
                .expect("ERROR: negative index"),
            _ => panic!("ERROR: index is not N% or Z%"),
        };
        let e_val = self.eval_expr(e);
        arr.borrow_mut().try_set(idx as usize, e_val).unwrap();
    }

    // t$f = e.
    fn do_tbl_ass(&self, t: &Expr, f: &Rc<DfStr>, e: &Expr)
    {
        let Val::T(mut t) = self.eval_expr(t) else {
            panic!("not a table");
        };
        let e_val = self.eval_expr(e);
        t.set(f, e_val);
    }

    #[inline]
    fn do_operon(&mut self, lhs: &Expr, op: &BinOpcode, ex: &Expr)
    {
        let val = self.eval_binop(lhs, op, ex);
        self.do_assign(lhs, &Expr::Const(val));
    }

    // helper for do_ifstmt & do_loopif
    #[inline]
    fn eval_cond(&self, cd: &Expr) -> bool
    {
        match self.eval_expr(cd) {
            Val::B(b) => b,
            _ => panic!("condition is not B%"),
        }
    }

    fn do_ifelse(
        &mut self,
        ic: &IfCase,
        ei: &[IfCase],
        eb: &Option<Block>)
     -> Option<BlockAction>
    {
        macro_rules! do_ifcase {
            ($zelf:ident, $c:ident) => {
                if $zelf.eval_cond(&$c.cond) {
                    return $zelf.do_block(&$c.blok);
                }
            }
        }
        do_ifcase!(self, ic);
        for c in ei {
            do_ifcase!(self, c);
        }
        return match eb {
            Some(b) => self.do_block(b),
            None => None,
        };
    }

    fn do_loopif(&mut self, lo: &Loop) -> Option<BlockAction>
    {
        let pre = self.vars.size();
        //self.enter_loop(lo); // preset Vs
        let ba = match lo {
            Loop::Inf(b      ) => self.do_inf_loop(b),
            Loop::Cdt(b, c, f) => self.do_cdt_loop(b, c, f),
        };
        self.clean(pre);
        return ba;
    }

    fn do_inf_loop(&mut self, block: &Block) -> Option<BlockAction>
    {
        loop {
            do_loop_block!(self, block);
        }
        return None;
    }

    fn do_cdt_loop(
        &mut self,
        blok0: &Block,
        condt: &Expr,
        blok1: &Block)
     -> Option<BlockAction>
    {
        loop {
            do_loop_block!(self, blok0);
            if !self.eval_cond(condt) {
                break;
            }
            do_loop_block!(self, blok1);
        }
        return None;
    }

    fn do_pccall(&mut self, p: &Expr, a: &[Expr])
    {
        let pc_val = self.eval_expr(p);
        let Val::P(p) = pc_val.clone() else {
            panic!("cannot call procedure {pc_val}");
        };
        if p.arity() != a.len() {
            panic!("not correct arity ({}) calling {:?}", a.len(), p);
        }
        let args = self.eval_args(a);
        match p {
            Proc::Nat(n) => n.exec(&args),
            Proc::Usr(u, s) => self.exec_usr_proc(&u, args, &s),
        }
    }

    #[inline]
    fn eval_args(&self, a: &[Expr]) -> Vec<Val>
    {
        a.iter()
         .map(|b| self.eval_expr(b))
         .collect()
    }

    fn exec_usr_proc(
        &self,
            subr: &MutRc<Subr>,
        mut args: Vec<Val>,
            upvs: &UpVals,
        )
    {
        // Future optimization: doesn't need a callee if it's not !@
        let mut proc_scope = Scope::with_callee(
            Val::new_usr_pc(subr.clone(), upvs.clone())
        );
        let subr = subr.borrow();
        if let Some(uv) = upvs {
            for (name, upval) in std::iter::zip(&subr.upvs, &**uv) {
                proc_scope.declar(name, upval.clone());
            }
        }
        for name in subr.pars.iter().rev() {
            let val = args.pop().unwrap(); // already checked arity
            proc_scope.declar(name, val);
        }
        if let Some(ba) = proc_scope.do_block(&subr.body) {
            match ba {
                BlockAction::End => return, // ok
                _ => panic!("cannot return or break from proc"),
            }
        };
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
            Expr::FnDef(s)       => self.eval_fndef(s),
            Expr::Fcall(c, a)    => self.eval_fcall(c, a),
            Expr::RecFn          => self.get_rec_f(),
            Expr::PcDef(s)       => self.eval_pcdef(s),
            Expr::RecPc          => self.get_rec_p(),
            Expr::Array(a)       => self.eval_array(a),
            Expr::Table(v)       => self.eval_table(v),
            Expr::TblFd(e, f)    => self.eval_tblfd(e, f),
            Expr::IfExp(c, e)    => self.eval_if_expr(c, e),
            _ => todo!("{:?}", e),
        }
    }

    #[inline]
    fn eval_ident(&self, i: &Rc<DfStr>) -> Val
    {
        // try variable
        if let Some(v) = self.vars.get(i) {
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
            .all(|e| e)
        );
    }

    #[inline]
    fn eval_fndef(&self, s: &MutRc<Subr>) -> Val
    {
        let mut upvals = vec![];
        let su = &s.borrow().upvs;
        if su.is_empty() {
            return Val::new_usr_fn(s.clone(), None);
        }
        for name in &s.borrow().upvs {
            upvals.push(self.eval_ident(name));
        }
        return Val::new_usr_fn(s.clone(), Some(Rc::new(upvals)));
    }

    #[inline]
    fn eval_fcall(&self, f: &Expr, a: &[Expr]) -> Val
    {
        let fn_val = self.eval_expr(f);
        let Val::F(f) = fn_val.clone() else {
            panic!("cannot call a non-function {fn_val}");
        };
        if f.arity() != a.len() {
            panic!("not correct arity ({}) calling {}", a.len(), fn_val);
        }
        let args = self.eval_args(a);
        match f {
            Func::Usr(u, s) => self.eval_usr_func(&u, args, &s),
            Func::Nat(n) => n.eval(&args).unwrap(),
        }
    }

    fn eval_usr_func(
        &self,
            subr: &MutRc<Subr>,
        mut args: Vec<Val>,
            upvs: &UpVals,
        ) -> Val
    {
        // Future optimization: doesn't need a callee if it's not #@
        let mut func_scope = Scope::with_callee(
            Val::new_usr_fn(subr.clone(), upvs.clone())
        );
        let subr = subr.borrow();
        if let Some(uv) = upvs {
            for (name, upval) in std::iter::zip(&subr.upvs, &**uv) {
                func_scope.declar(name, upval.clone());
            }
        }
        for name in subr.pars.iter().rev() {
            let val = args.pop().unwrap(); // already checked arity
            func_scope.declar(name, val);
        }
        let Some(ba) = func_scope.do_block(&subr.body) else {
            panic!("ended function w/o returning a value");
        };
        match ba {
            BlockAction::Ret(v) => return v,
            _ => panic!("cannot return or break from func"),
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
        let lval = self.eval_expr(l);
        let Val::B(lval) = lval else {
            panic!("lhs value of {lval} is not B%");
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
        let Val::A(a_val) = self.eval_expr(a) else {
            panic!("ERROR: {a:?} is not indexable (must _%)");
        };
        let ival = self.eval_expr(i);
        let i_val = match ival {
            Val::N(n) => n,
            Val::Z(z) => u32::try_from(z)
                .expect("ERROR: negative index"),
            _ => panic!("cannot use {ival} as index"),
        };
        let a_ref = a_val.borrow();
        match a_ref.get(i_val as usize) {
            Some(v) => v,
            None => panic!("{} out of bounds (len = {})",
                i_val, a_ref.len()),
        }
    }

    #[inline]
    fn eval_pcdef(&self, s: &MutRc<Subr>) -> Val
    {
        let mut upvals = vec![];
        let su = &s.borrow().upvs;
        if su.is_empty() {
            return Val::new_usr_pc(s.clone(), None);
        }
        for name in &s.borrow().upvs {
            upvals.push(self.eval_ident(name));
        }
        return Val::new_usr_pc(s.clone(), Some(Rc::new(upvals)));
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

    fn eval_table(&self, e: &[(Rc<DfStr>, Expr)]) -> Val
    {
        let mut t = Table::new_empty();
        // TODO eke $@
        for (k, ve) in e {
            let v = self.eval_expr(ve);
            t.set(k, v);
        }
        return Val::T(t);
    }

    fn eval_tblfd(&self, t: &Expr, f: &DfStr) -> Val
    {
        let tbl = self.eval_expr(t);
        let Val::T(trc) = tbl else {
            panic!("{tbl} is not a table");
        };
        return trc.get(f).expect(&format!("table hasn't ${f}"));
    }

    #[inline]
    fn eval_if_expr(&self, cases: &[(Expr, Expr)], elze: &Expr) -> Val
    {
        for (cond, res) in cases {
            if self.eval_cond(cond) {
                return self.eval_expr(res);
            }
        }
        return self.eval_expr(elze);
    }
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
            _ => panic!("can only invert (/) a R% value"),
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
    if *o == BinOpcode::Mod { match (l, r) {
        (Val::N(vl), Val::N(vr)) => return Val::N(vl % vr),
        (Val::Z(vl), Val::N(vr)) => {
            let vrz = *vr as i32;
            return Val::N(((vl % vrz + vrz) % vrz).try_into().unwrap());
        },
        _ => panic!("invalid types in mod op"),
    }}

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
        (Val::A(a), Val::A(b)) => match o {
            BinOpcode::Add => Val::from_array(a.borrow().add(&b.borrow())
                .unwrap()),
            _ => panic!("not valid operation btwin _%"),

        },
        _ => panic!("not valid operation btwin {l:?} and {r:?}"),
    }
}

#[derive(Debug, Clone)]
pub enum BlockAction
{
    Ret(Val),
    End,
    Loo(u32, bool), // level, {Again or Break} as bool
}

impl BlockAction
{
    #[inline]
    fn exiting_loop(&self) -> Option<Self>
    {
        match self {
            // decrease level by 1, bcoz broke from current loop
            BlockAction::Loo(lev, t) => match lev {
                0 => None,
                _ => Some(Self::Loo(lev-1, *t)),
            },
            // maybe return or endproc
            _ => Some(self.clone()),
        }
    }
}
