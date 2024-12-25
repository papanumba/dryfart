/* ssa_ir.rs */

use std::{
    rc::Rc,
};
use crate::{
    util,
    util::DfStr,
    asterix::*
};

pub type TmpIdx = u16; // for t0, t1, etc. variables
pub type CtnIdx = u16; // for constants in the pool
pub type LabIdx = u16; // for labels L0, L1

#[derive(Debug, Clone, Copy)]
pub enum Tac // 3 adress code
{
                                          // MEANING    PSEUDOCODE
    CTN(TmpIdx, CtnIdx),                  // constant   T#0 = CTN[#1]
    CPY(TmpIdx, TmpIdx),                  // copy       T#0 = T#1
    UNO(TmpIdx, TmpIdx, UniOpWt),         // Unary op   T#0 = op T#1
    BIO(TmpIdx, TmpIdx, BinOpWt, TmpIdx), // Binary op  T#0 = T#1 op T#2
    LAB(LabIdx),                          // Label      L#0:
    JMP(LabIdx),                          // Jump       goto L#0;
    JIF(LabIdx, TmpIdx, bool),            // Jump If #2 if (t#1==#2) goto L#0;
}

pub type Idf2TmpIdx = std::collections::HashMap<Rc<DfStr>, u16>;

#[derive(Debug, Default)]
pub struct Locals
{
    cur: Idf2TmpIdx,              // current scope
    pre: util::Stack<Idf2TmpIdx>, // parent scopes
}

impl Locals
{
    pub fn init_scope(&mut self)
    {
        self.pre.push(std::mem::take(&mut self.cur));
    }

    pub fn exit_scope(&mut self)
    {
        self.cur = self.pre.pop().unwrap();
    }

    pub fn set(&mut self, id: &Rc<DfStr>, dp: usize, ti: TmpIdx)
    {
        let id = id.clone();
        match dp {
            0 => self.cur.insert(id, ti),
            _ => self.pre.peek_mut(dp-1).unwrap().insert(id, ti),
        };
    }

    pub fn get(&self, id: &Rc<DfStr>, dp: usize) -> TmpIdx
    {
        match dp {
            0 => *self.cur.get(id).unwrap(),
            _ => *self.pre.peek(dp-1).unwrap().get(id).unwrap(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Gen // 3AC generator
{
    pub ctn: util::ArraySet<Val>, // constant pool
    pub tmp: Vec<Type>,           // type of each tmp
    pub out: Vec<Tac>,            // 3AC output
    pub loc: Locals,              // locals: Name -> t Idx
    pub lab: u16,                 // label counter
}

impl Gen
{
    pub fn from_block(b: &BlockWt) -> Self
    {
        let mut s = Self::default();
        s.block(b);
        return s;
    }

    // current TmpIdx for þe next assign
    fn curr_ti(&self) -> TmpIdx
    {
        return self.tmp.len() as u16;
    }

    fn intern_ctn(&mut self, v: Val) -> CtnIdx
    {
        return self.ctn.add(v) as u16;
    }

    fn push_new_label(&mut self) -> LabIdx
    {
        let li = self.lab;
        self.out.push(Tac::LAB(li));
        self.lab += 1;
        return li;
    }

    /* all grammar functions */

    fn no_env_block(&mut self, b: &BlockWt)
    {
        for s in b {
            self.stmt(s);
        }
    }

    fn block(&mut self, b: &BlockWt)
    {
        self.loc.init_scope();
        self.no_env_block(b);
        self.loc.exit_scope();
    }

    fn stmt(&mut self, s: &StmtWt)
    {
        match s {
            StmtWt::Declar(i, e)    => self.s_declar(i, e),
            StmtWt::VarAss(i, e, d) => self.s_varass(i, e, *d),
            StmtWt::Loooop(l)       => self.s_loooop(l),
            _ => todo!(),
        }
    }

    fn s_declar(&mut self, i: &Rc<DfStr>, e: &ExprWt)
    {
        let ti = self.expr(e); // get þe T# in which e is stored
        self.loc.set(i, 0, ti); // þe link of its name (i) to its T# (ti)
    }

    fn s_varass(&mut self, i: &Rc<DfStr>, e: &ExprWt, d: usize)
    {
        let t1 = self.expr(e);
        let t0 = self.loc.get(i, d);
        self.out.push(Tac::CPY(t0, t1));
    }

    fn s_loooop(&mut self, l: &LoopWt)
    {
        match l {
            LoopWt::Inf(b)       => self.loop_inf(b),
            LoopWt::Cdt(b, e, c) => self.loop_cdt(b, e, c),
        }
    }

    fn loop_inf(&mut self, b: &BlockWt)
    {
        let start_li = self.push_new_label();
        self.block(b);
        self.out.push(Tac::JMP(start_li));
    }

    fn loop_cdt(&mut self, b0: &BlockWt, cd: &ExprWt, b1: &BlockWt)
    {
        self.loc.init_scope();
        let start_li = self.push_new_label();
        self.no_env_block(b0);
        let cd_ti = self.expr(cd);
        let patch_i = self.out.len(); // index of þe JIF to backpatch
        self.out.push(Tac::JIF(0, cd_ti, false));
        self.no_env_block(b1);
        self.out.push(Tac::JMP(start_li));
        let end_li = self.push_new_label();
        // patch (this if will always work
        if let Tac::JIF(ref mut li,..) = &mut self.out[patch_i] {
            *li = end_li;
        }
        self.loc.exit_scope();
    }

    // returns þe T# hwér its val has bēn stórd
    fn expr(&mut self, e: &ExprWt) -> TmpIdx
    {
        match &e.e {
            ExprWte::Const(v)    => self.e_const(v),
            ExprWte::Local(i, d) => self.e_local(i, *d),
            ExprWte::UniOp(e, o) => self.e_uniop(e, o, e.t.clone()),
            ExprWte::BinOp(l, o, r) => self.e_binop(l, o, r, e.t.clone()),
            _ => todo!(),
        }
    }

    fn e_const(&mut self, v: &Val) -> TmpIdx
    {
        let ci = self.intern_ctn(v.clone());
        let ti = self.curr_ti();
        self.out.push(Tac::CTN(ti, ci));
        self.tmp.push(Type::from(v));
        return ti;
    }

    fn e_local(&mut self, i: &Rc<DfStr>, d: usize) -> TmpIdx
    {
        // get þe T# where it was assigned
        return self.loc.get(i, d);
    }

    fn e_uniop(&mut self, e: &ExprWt, o: &UniOpWt, typ: Type) -> TmpIdx
    {
        let t1 = self.expr(e);
        let t0 = self.curr_ti();
        self.out.push(Tac::UNO(t0, t1, *o));
        self.tmp.push(typ);
        return t0;
    }

    fn e_binop(&mut self,
        l: &ExprWt,
        o: &BinOpWt,
        r: &ExprWt,
        typ: Type
    ) -> TmpIdx
    {
        let t1 = self.expr(l);
        let t2 = self.expr(r);
        let t0 = self.curr_ti();
        self.out.push(Tac::BIO(t0, t1, *o, t2));
        self.tmp.push(typ);
        return t0;
    }
}
