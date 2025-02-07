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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TacBinOp
{
    Cmp(CmpOpWt),
    Bin(BinOpWt),
}

#[derive(Debug, Clone, Copy)]
pub enum Tac // 3 adress code
{
                                          // MEANING    PSEUDOCODE
    CTN(TmpIdx, CtnIdx),                  // constant   T#0 = CTN[#1]
    CPY(TmpIdx, TmpIdx),                  // copy       T#0 = T#1
    UNO(TmpIdx, TmpIdx, UniOpWt),         // Unary op   T#0 = op T#1
    BIO(TmpIdx, TmpIdx, TacBinOp, TmpIdx),// Binary op  T#0 = T#1 op T#2
    LAB(LabIdx),                          // Label      L#0:
    JMP(LabIdx),                          // Jump       goto L#0;
    JIF(LabIdx, TmpIdx, bool),            // Jump If #2 if (t#1==#2) goto L#0;
}
    // TODO idea: Join JMP & JIF doing Option<(TmpIdx, bool)>

pub type Idf2TmpIdx = std::collections::HashMap<IdfIdx, u16>;

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

    pub fn set(&mut self, id: &IdfIdx, dp: usize, ti: TmpIdx)
    {
        let id = id.clone();
        match dp {
            0 => self.cur.insert(id, ti),
            _ => self.pre.peek_mut(dp-1).unwrap().insert(id, ti),
        };
    }

    pub fn get(&self, id: &IdfIdx, dp: usize) -> TmpIdx
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
    pub fn from_prog(p: &ProgWt) -> Self
    {
        let mut s = Self::default();
        s.block(&p.main);
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

    // pushes Tac and returns its index in self.out
    fn push_tac(&mut self, t: Tac) -> usize
    {
        let i = self.out.len();
        self.out.push(t);
        return i;
    }

    fn patch_jmp(&mut self, at: usize, new_li: LabIdx)
    {
        // þis if will always work
        if let Tac::JMP(ref mut li) = &mut self.out[at] {
            *li = new_li;
        }
    }

    fn patch_jif(&mut self, at: usize, new_li: LabIdx)
    {
        // þis if will always work
        if let Tac::JIF(ref mut li,..) = &mut self.out[at] {
            *li = new_li;
        }
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
            StmtWt::IfElse(i, f, e) => self.s_ifelse(i, f, e),
            StmtWt::Loooop(l)       => self.s_loooop(l),
            //_ => todo!(),
        }
    }

    fn s_declar(&mut self, i: &IdfIdx, e: &ExprWt)
    {
        let ti = self.expr(e); // get þe T# in which e is stored
        self.loc.set(i, 0, ti); // þe link of its name (i) to its T# (ti)
    }

    fn s_varass(&mut self, i: &IdfIdx, e: &ExprWt, d: usize)
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
        let patch_i = self.push_tac(Tac::JIF(0, cd_ti, false)); // 0 dummy
        self.no_env_block(b1);
        self.out.push(Tac::JMP(start_li));
        let end_li = self.push_new_label();
        self.patch_jif(patch_i, end_li);
        self.loc.exit_scope();
    }

    fn s_ifelse(
        &mut self,
        if0: &IfCaseWt,
        eis: &[IfCaseWt],
        els: &Option<BlockWt>,
    ) {
        let mut afters = vec![];
        afters.push(self.if_case(if0));
        for ic in eis {
            afters.push(self.if_case(ic));
        }
        if let Some(b) = els {
            self.block(b);
        }
        let end_li = self.push_new_label();
        for a in &afters {
            self.patch_jmp(*a, end_li);
        }
    }

    // aux fn for s_ifelse
    // returns þe index to patch on JMP after
    // þis cás's block has bēn successfully dón
    fn if_case(&mut self, ic: &IfCaseWt) -> usize
    {
        // all "0" values are dummies to be patched
        let cond_ti = self.expr(&ic.cond);
        let patch = self.push_tac(Tac::JIF(0, cond_ti, false));
        self.block(&ic.blok);
        let after = self.push_tac(Tac::JMP(0));
        let end_li = self.push_new_label();
        self.patch_jif(patch, end_li);
        return after;
    }

    // returns þe T# hwér its val has bēn stórd
    fn expr(&mut self, e: &ExprWt) -> TmpIdx
    {
        match &e.e {
            ExprWte::Const(v)       => self.e_const(v),
            ExprWte::Local(i, d)    => self.e_local(i, *d),
            ExprWte::UniOp(e, o)    => self.e_uniop(e, o, e.t.clone()),
            ExprWte::BinOp(l, o, r) => self.e_binop(l, o, r, e.t.clone()),
            ExprWte::CmpOp(f, o)    => self.e_cmpop(f, o),
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

    fn e_local(&mut self, i: &IdfIdx, d: usize) -> TmpIdx
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
        let op = TacBinOp::Bin(*o);
        self.out.push(Tac::BIO(t0, t1, op, t2));
        self.tmp.push(typ);
        return t0;
    }

    fn e_cmpop(&mut self, first: &ExprWt, others: &[(CmpOpWt, ExprWt)]) -> TmpIdx
    {
        let first_ti = self.expr(first);
        let len_others = others.len();
        if len_others == 0 {
            return first_ti;
        }
        if len_others == 1 {
            let second_ti = self.expr(&others[0].1);
            let res_ti = self.curr_ti();
            let op = TacBinOp::Cmp(others[0].0);
            self.out.push(Tac::BIO(res_ti, first_ti, op, second_ti));
            self.tmp.push(Type::B);
            return res_ti;
        }
        todo!("multi cmp");
    }
}
