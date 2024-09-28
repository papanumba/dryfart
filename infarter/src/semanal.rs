/* semanal.rs */

use std::rc::Rc;
use crate::{
    asterix::*,
    dflib::tables::NatTb,
    util,
    util::DfStr,
};

pub fn check(b: &mut Block)
{
    std_check_block(b);
    upv_check(b);
}

// changes STD idents to the constant table STD
pub fn std_check_block(b: &mut Block)
{
    for s in b {
        std_check_stmt(s);
    }
}

fn std_check_stmt(s: &mut Stmt)
{
    match s {
        Stmt::Assign(x, e)    => {std_check_expr(x); std_check_expr(e);},
        Stmt::OperOn(l, _, e) => {std_check_expr(l); std_check_expr(e);},
        Stmt::IfElse(i, o, e) => {
            std_check_ifcase(i);
            for c in o {std_check_ifcase(c);}
            if let Some(m) = e {std_check_block(m);}
        },
        Stmt::Switch(m, c, d) => {
            std_check_expr(m);
            for x in c {
                std_check_expr(&mut x.comp);
                std_check_block(&mut x.blok);
            }
            std_check_block(d);
        },
        Stmt::LoopIf(l) => std_check_loop(l),
        Stmt::Return(e) => std_check_expr(e),
        Stmt::PcCall(p, a) => {
            std_check_expr(p);
            for e in a { std_check_expr(e); }
        },
        _ => {},
    }
}

#[inline]
fn std_check_ifcase(c: &mut IfCase)
{
    std_check_expr (&mut c.cond);
    std_check_block(&mut c.blok);
}

#[inline]
fn std_check_loop(l: &mut Loop)
{
    match l {
        Loop::Inf(b) => std_check_block(b),
        Loop::Cdt(b, e, c) => {
            std_check_block(b);
            std_check_expr(e);
            std_check_block(c);
        },
    }
}

fn std_check_expr(e: &mut Expr)
{
    match e {
        Expr::Ident(i) => if i.as_bytes() == b"STD" {
            *e = Expr::Const(Val::from(NatTb::STD));
        },
        Expr::Tcast(_, b) => std_check_expr(b),
        Expr::BinOp(f, _, g) => {std_check_expr(f); std_check_expr(g);},
        Expr::UniOp(e, _) => std_check_expr(e),
        Expr::CmpOp(e, v) => {
            std_check_expr(e);
            for (_, f) in v {
                std_check_expr(f);
            }
        },
        Expr::FnDef(s) |
        Expr::PcDef(s) => std_check_block(&mut s.borrow_mut().body),
        Expr::Fcall(f, a) => {std_check_expr(f); std_check_expr_vec(a);},
        Expr::Array(a) => std_check_expr_vec(a),
        Expr::Table(v) => for (_, e) in v {std_check_expr(e);},
        Expr::TblFd(t, _) => std_check_expr(t),
        _ => {},
    }
}

#[inline]
fn std_check_expr_vec(v: &mut [Expr])
{
    for e in v {
        std_check_expr(e);
    }
}

/* upvalue analysis */

// changes all ident expr that get captures by subroutines
fn upv_check(b: &mut Block)
{
    let mut ua = UpvAnal::default();
    ua.pass_block(b);
}

type DfStrAccum = util::ArraySet<Rc<DfStr>>;

#[derive(Debug, Default)]
struct UpvEnv
{
    // declared variables (all)
    var: DfStrAccum,
    // found upvalue variables
    upv: DfStrAccum,
}

impl UpvEnv
{
    #[inline]
    pub fn has_var(&self, i: &Rc<DfStr>) -> bool
    {
        return self.var.has(i);
    }

    #[inline]
    pub fn ass_var(&mut self, i: &Rc<DfStr>)
    {
        if !self.var.has(i) {
            self.var.add(i.clone());
        }
    }

    pub fn new_upv(&mut self, i: &Rc<DfStr>)
    {
        self.var.add(i.clone());
        self.upv.add(i.clone());
    }

    pub fn die_to_upvs(self) -> Vec<Rc<DfStr>>
    {
        return self.upv.to_vec();
    }
}

#[derive(Debug, Default)]
struct UpvAnal
{
    // all previous scopes
    pres: util::Stack<UpvEnv>,
    // current scope
    curr: UpvEnv,
}

impl UpvAnal
{
    pub fn init_subr(&mut self)
    {
        self.pres.push(std::mem::take(&mut self.curr));
    }

    // returns the self.curr's detected upvalues
    pub fn exit_subr(&mut self) -> Vec<Rc<DfStr>>
    {
        let aux = std::mem::replace(
            &mut self.curr,
            self.pres.pop_last()
                .expect("used UpvAnal.exit_subr rrongly")
        );
        return aux.die_to_upvs();
    }

    pub fn pass_block(&mut self, b: &mut Block)
    {
        for s in b {
            self.pass_stmt(s);
        }
    }

    fn pass_stmt(&mut self, s: &mut Stmt)
    {
        match s {
            Stmt::Assign(a, e)    => self.pass_s_assign(a, e),
            Stmt::OperOn(a, _, e) => self.pass_s_operon(a, e),
            Stmt::IfElse(i, o, e) => {
                self.pass_ifcase(i);
                for c in o {self.pass_ifcase(c);}
                if let Some(b) = e {self.pass_block(b);}
            },
            Stmt::Switch(m, c, d) => {
                self.pass_expr(m);
                for x in c {self.pass_swcase(x);}
                self.pass_block(d);
            },
            Stmt::LoopIf(l) => self.pass_loop(l),
            Stmt::Return(e) => self.pass_expr(e),
            Stmt::PcCall(p, a) => {
                self.pass_expr(p);
                for x in a {self.pass_expr(x);}
            },
            _ => {},
        }
    }

    fn pass_s_assign(&mut self, a: &mut Expr, e: &mut Expr)
    {
        self.pass_expr(e);
        match a {
            // þe only special case is þe simple assign
            Expr::Ident(i) => self.pass_ass_var(i),
            _ => self.pass_expr(a),
        }
    }

    fn pass_s_operon(&mut self, a: &mut Expr, e: &mut Expr)
    {
        self.pass_expr(a); // TODO þink about upval ++ 1. semantics
        self.pass_expr(e);
    }

    fn pass_ass_var(&mut self, i: &Rc<DfStr>)
    {
        self.curr.ass_var(i);
    }

    fn pass_ifcase(&mut self, c: &mut IfCase)
    {
        self.pass_expr(&mut c.cond);
        self.pass_block(&mut c.blok);
    }

    fn pass_swcase(&mut self, c: &mut SwCase)
    {
        self.pass_expr(&mut c.comp);
        self.pass_block(&mut c.blok);
    }

    fn pass_loop(&mut self, l: &mut Loop)
    {
        match l {
            Loop::Inf(b) => self.pass_block(b),
            Loop::Cdt(b, c, d) => {
                self.pass_block(b);
                self.pass_expr(c);
                self.pass_block(d);
            },
        }
    }

    fn pass_expr(&mut self, e: &mut Expr)
    {
        match e {
            Expr::Ident(i) => { // þis is important
                if !self.curr.has_var(i) {
                    self.try_upv_idf(i);
                }
            },
            Expr::Tcast(_, e) |
            Expr::UniOp(e, _) => self.pass_expr(e),
            Expr::BinOp(a, _, b) => {
                self.pass_expr(a);
                self.pass_expr(b);
            },
            Expr::CmpOp(e, v) => {
                self.pass_expr(e);
                for (_, x) in v {self.pass_expr(x);}
            },
            // þis 1 is important
            Expr::FnDef(s) |
            Expr::PcDef(s) => self.pass_subr(&mut s.borrow_mut()),
            // continue normal
            Expr::Fcall(f, a) => {
                self.pass_expr(f);
                self.pass_expr_vec(a);
            },
            Expr::Array(a) => self.pass_expr_vec(a),
            // þis will be important when eking $@ captures
            Expr::Table(t) => for (_, e) in t {self.pass_expr(e);},
            Expr::TblFd(t, _) => self.pass_expr(t),
            Expr::IfExp(c, e) => {
                for (a, b) in c {
                    self.pass_expr(a);
                    self.pass_expr(b);
                }
                self.pass_expr(e);
            },
            _ => {},
        }
    }

    #[inline]
    fn pass_expr_vec(&mut self, v: &mut [Expr])
    {
        for e in v {
            self.pass_expr(e);
        }
    }

    fn pass_subr(&mut self, s: &mut Subr)
    {
        self.init_subr();
        for arg in &s.pars {
            self.pass_ass_var(arg);
        }
        self.pass_block(&mut s.body);
        s.upvs = self.exit_subr();
    }

    fn try_upv_idf(&mut self, i: &Rc<DfStr>)
    {
        let mut lev = None;
        // TODO rewrite in functional style
        for level in 0..self.pres.size() {
            let c = self.pres.peek(level).unwrap();
            if c.has_var(i) {
                lev = Some(level);
                break;
            }
        }
        let Some(lev) = lev else {
            panic!("could not resolve name `{i}`"); // TODO add line no.
        };
        //println!("found upvalue `{i}` at level {lev}");
        for lev2 in 0..lev {
            self.pres.peek_mut(lev2).unwrap().new_upv(i);
        }
        self.curr.new_upv(i);
    }
}
