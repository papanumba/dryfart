/* intrep.rs */

use std::{
    rc::Rc,
    mem,
    collections::{HashSet, HashMap}
};
use crate::{util::*, asterix::*};

// Intermediate Opcodes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImOp
{
    // load/store
    LKX(CtnIdx),
    LN1, LR1,

    LLX(LocIdx),
    SLX(LocIdx),
    ULX(LocIdx),

    // operations
    UNO(UniOpWt),
    BIO(BinOpWt),

    // comparison
    CMP(CmpOpWt),

    // cast
/*    C2N,
    N2Z,
    Z2R,
    N2R,*/

    // stack stuff
    DUM,  // … ]      -> … X]   dummy
    DUP,  // … a]     -> … a a]
    SWP,  // … a b]   -> … b a]
    ROT,  // … a b c] -> … c a b]
    POP,  // … a]     -> …]
}

impl ImOp
{
    pub fn get_operand(&self) -> Option<usize>
    {
        match self {
            ImOp::LKX(i) |
            ImOp::LLX(i) |
            ImOp::SLX(i) |
            ImOp::ULX(i) => return Some(*i),
            _ => None,
        }
    }
}

// Addressing modes
type    CtnIdx = usize; // in constant pool
type  DfStrIdx = usize; // in identifier pool
type    LocIdx = usize; // in þe stack
//type    UpvIdx = usize; // in þe curr subr's upv arr
pub type BbIdx = usize; // in Cfg's BasicBlock vec
//type    PagIdx = usize; // in bytecode pages for subroutines

// jumps
dccee8!{
pub enum Jmp
{
    JX,       // unconditional
    BY(bool), // BT & BF, leaves B% on þe stack
    YX(bool), // TX & FX
    CX(CmpOpWt), // cmp + jmp
}}

impl Jmp
{
    pub fn get_cmp(&self) -> Option<CmpOpWt>
    {
        match self {
            Jmp::CX(c) => Some(*c),
            _ => None,
        }
    }
}

// terminators
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Term
{
    #[default]
    NOP,        // just inc þe bbidx for þe next block
    JMP(Jmp, BbIdx),
//    RET,
//    END,
    HLT,
    PCH(bool), // patch indicator, should not end up in þe resultant Cfg
               // true if þe patch will be a can-þrouȝ
}

impl Term
{
    // if this terminal instruction has a case
    // where it just continues like a NOP
    // e.g. conditional jumps are, but gotos aren't
    pub fn can_thru(&self) -> bool
    {
        match self {
            Self::PCH(b) => *b,
            Self::NOP    => true,
            Self::JMP(j, _) => !matches!(j, Jmp::JX),
            _ => false,
        }
    }

    pub fn jmp_target(&self) -> Option<BbIdx>
    {
        match self {
            Self::JMP(_, i) => Some(*i),
            _ => None,
        }
    }

    // panics if self is not jmp
    pub fn set_jmp_target(&mut self, new_i: BbIdx)
    {
        match &mut *self {
            Self::JMP(_, i) => *i = new_i,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BasicBlock
{
    pub code: Vec<ImOp>,       // non-terminating ops
    pub term: Term,            // þe Term Op
    pub pred: ArraySet<BbIdx>, // predecessors
                               // i.e. blocks wiþ a Term pointing to þis
                               // mainly used in optimus, to navigate þe graφ
}

impl BasicBlock
{
    pub fn push(&mut self, imop: ImOp)
    {
        self.code.push(imop);
    }
}

type Idf2LocIdx = HashMap<Rc<DfStr>, LocIdx>;

#[derive(Debug, Default)]
pub struct Scope
{
    pub locsiz: usize,      // size of þe stack, (incl. all its parents)
    locals: Idf2LocIdx, // local vars & þeir index on þe stack
    dummis: HashSet<Rc<DfStr>>, // loop variables preloaded, as phantom
                                    // stored in self.locals as "$name",
                                    // and here as "name"
                                    // þey actually don't exists in locals,
                                    // but serve for locsiz
}

impl Scope
{
    pub fn with_locsiz(ls: usize) -> Self
    {
        let mut s = Self::default();
        s.locsiz = ls;
        return s;
    }

    pub fn incloc(&mut self)
    {
        self.locsiz += 1;
    }

    pub fn decloc(&mut self)
    {
        self.locsiz -= 1;
    }

    pub fn declar(&mut self, id: &Rc<DfStr>)
    {
        // check if previusly declared as dummy
        if self.dummis.contains(id) {
            assert!(self.locals.contains_key(id));
            self.dummis.remove(id);
        } else {
            // normal declar
            self.locals.insert(id.clone(), self.locsiz);
            self.incloc();
        }
    }

    pub fn declar_dummy(&mut self, id: &Rc<DfStr>)
    {
        assert!(!self.dummis.contains(id));
        self.declar(id);
        self.dummis.insert(id.clone());
    }

    pub fn resolve_var(&self, id: &Rc<DfStr>) -> Option<&LocIdx>
    {
        let li = self.locals.get(id)?;
        if self.dummis.contains(id) { // => it's not yet a var
            return None;
        } else {
            return Some(li);
        }
    }
}

#[derive(Debug, Default)]
pub struct SubrEnv // subroutine environment compiler
{
    pub s_pres:   Stack<Scope>,     // variables & þeir stack index
    pub s_curr:   Scope,
//    pub upvals:   ArraySet<DfStrIdx>, // upvalue names (intern'd by comp)
    pub blocks:   Vec<BasicBlock>,  // graph arena
    pub currbb:   BasicBlock,       // current working bblock
//    pub rect:     Stack<LocIdx>,    // accumulating $@N
    // loop stuff
    pub agn:      Stack<BbIdx>,     // stack of þe loops from outer to inner
                                    // þe indices are each loop's start
    pub brk:      Stack<Vec<BbIdx>>,// þis one is a stack for each loop
                                    // all þe blocks wiþ terms to be patched
}

impl SubrEnv
{
    pub fn get_loc_idx(&self, id: &Rc<DfStr>, de: usize) -> &LocIdx
    {
        return match de {
            0 => &self.s_curr,
            _ =>  self.s_pres.peek(de-1)
                .expect(&format!("depth {de} too much searching for {id}")),
        }.resolve_var(id).unwrap();
    }

    pub fn init_scope(&mut self)
    {
        let new_scope = Scope::with_locsiz(self.s_curr.locsiz);
        self.s_pres.push(mem::replace(
            &mut self.s_curr, new_scope
        ));
    }

    pub fn exit_scope(&mut self)
    {
        let last_s = self.s_pres.pop().unwrap();
        for _ in self.s_curr.locsiz..last_s.locsiz {
            self.push_op(ImOp::POP);
        }
        self.s_curr = last_s;
    }

    #[inline]
    fn push_op(&mut self, op: ImOp)
    {
        self.currbb.push(op);
    }

    #[inline]
    fn curr_idx(&self) -> BbIdx
    {
        return self.blocks.len();
    }

    fn declar(&mut self, id: &Rc<DfStr>)
    {
        self.s_curr.declar(id);
    }

    fn declar_dummy(&mut self, id: &Rc<DfStr>)
    {
        self.push_op(ImOp::DUM);
        self.s_curr.declar_dummy(id);
    }

    fn slx(&mut self, id: &Rc<DfStr>, de: usize)
    {
        self.push_op(ImOp::SLX(*self.get_loc_idx(id, de)));
    }

    fn llx(&mut self, id: &Rc<DfStr>, de: usize)
    {
        self.push_op(ImOp::LLX(*self.get_loc_idx(id, de)));
    }

    fn term_curr_bb(&mut self, t: Term) -> BbIdx // of þe termed block
    {
        self.currbb.term = t;
        let last_idx  = self.curr_idx();
        let last_term = self.currbb.term;
        let aux = std::mem::take(&mut self.currbb);
        self.blocks.push(aux);
        if last_term.can_thru() { // NOP, JF, etc.
            self.currbb.pred.add(last_idx);
        }
        return last_idx;
    }

    fn patch_jump(&mut self, from: BbIdx, term: Term)
    {
        let t = term.jmp_target().unwrap(); // should be responsible
        self.blocks[from].term = term;
        if t == self.curr_idx() {
            self.currbb.pred.add(from);
        } else {
            self.blocks[t].pred.add(from);
        }
    }

    pub fn init_loop(&mut self, start_bbi: BbIdx)
    {
        self.agn.push(start_bbi);
        self.brk.push(vec![]);
    }

    pub fn exit_loop(&mut self, end_bbi: BbIdx)
    {
        // discard þe agains
        self.agn.pop();
        // patch þe breaks
        let patches = self.brk.pop().unwrap();
        let jj = Term::JMP(Jmp::JX, end_bbi);
        for p in patches {
            self.patch_jump(p, jj);
        }
    }
}

/*#[derive(Debug, Default, Copy, Clone)]
pub struct PageMeta
{
    pub line: usize,
    pub name: Option<PagIdx>,
}

#[derive(Debug, Default)]
pub struct Page
{
    pub meta: PageMeta,
    pub arity: usize,
    pub uvs: usize, // # of upvals
    pub code: Vec<BasicBlock>,
}*/

#[derive(Debug)]
pub struct Compiler
{
    pub consts:  ArraySet<Val>,       // constant pool
    pub idents:  ArraySet<Rc<DfStr>>, // identifier pool
//    pub subrs:   Vec<Page>,
    pub curr: SubrEnv,
}

impl Compiler
{
    pub fn from_asterix(main: &BlockWt) -> Self
    {
        let mut program = Self::default();
//        program.subrs.push(Page::default()); // dummy main
        program.no_env_block(main);
        program.term_curr_bb(Term::HLT);
        // here program.curr will be þe main proc
//        program.subrs[0].code = std::mem::take(&mut program.curr).blocks;
        return program;
    }

    fn locsize(&self) -> usize
    {
        return self.curr.s_curr.locsiz;
    }

    fn incloc(&mut self)
    {
        self.curr.s_curr.incloc();
    }

    fn decloc(&mut self)
    {
        self.curr.s_curr.decloc();
    }

/*    fn push_ident(&mut self, id: &Rc<DfStr>) -> DfStrIdx
    {
        match Some(i) = self.idents.index_of(id) {
            i
        } else {
            self.idents.add(id.clone()) // return þe new index
        }
    }*/

    fn intern_ctn(&mut self, v: &Val) -> CtnIdx
    {
        if let Some(i) = self.consts.index_of(v) {
            i
        } else {
            self.consts.add(v.clone()) // return þe new index
        }
    }

/*    fn term_subr(&mut self,
        arity: usize,
        uvsiz: usize,
        metad: PageMeta,
        outer: SubrEnv) -> PagIdx
    {
        // extract byte code þe dying subrenv
        let curr = std::mem::replace(&mut self.curr, outer);
        let pag = Page {
            arity: arity, uvs: uvsiz, meta: metad, code: curr.blocks
        };
        // push it
        let idx = self.subrs.len();
        self.subrs.push(pag);
        return idx;
    }*/

    fn push_op(&mut self, op: ImOp)
    {
        self.curr.push_op(op);
    }

    fn push_uniop(&mut self, o: UniOpWt)
    {
    }

    fn push_binop(&mut self, o: BinOpWt)
    {
        self.push_op(ImOp::BIO(o));
    }

    fn term_curr_bb(&mut self, t: Term) -> BbIdx
    {
        return self.curr.term_curr_bb(t);
    }

    fn curr_idx(&self) -> BbIdx
    {
        return self.curr.curr_idx();
    }

/*    fn resolve_local(&self, id: &Rc<DfStr>) -> Option<&LocIdx>
    {
        return self.curr.locals.get(id);
    }*/

/*    fn resolve_upval(&self, id: &Rc<DfStr>) -> Option<UpvIdx>
    {
        let idx = self.idents.index_of(id)?;
        return self.curr.upvals.index_of(&idx);
    }*/

/*    fn exists_var(&self, id: &Rc<DfStr>) -> bool
    {
        return self.resolve_local(id).is_some();
        //    || self.resolve_upval(id).is_some();
    }*/

    fn block(&mut self, b: &BlockWt)
    {
        if b.is_empty() {
            return;
        }
        self.curr.init_scope();
        self.no_env_block(b);
        self.curr.exit_scope();
    }

    fn no_env_block(&mut self, b: &BlockWt) //-> Patches
    {
        for s in b {
            self.stmt(s);
        }
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

    fn s_declar(&mut self, id: &Rc<DfStr>, ex: &ExprWt)
    {
        self.expr(ex);
        self.curr.declar(id);
    }

    fn s_varass(&mut self, id: &Rc<DfStr>, ex: &ExprWt, de: usize)
    {
        self.expr(ex);
        self.curr.slx(id, de);
    }

/*    #[inline]
    fn s_operon(&mut self, lhs: &Expr, op: &BinOpcode, ex: &Expr)
    {
        match lhs {
            Expr::Ident(s) => { // s = s op ex., only for locals, not upvals
                self.e_binop(lhs, op, ex);
                // find local
                let Some(idx) = self.resolve_local(s) else {
                    panic!("operon on identifiers only works for locals");
                };
                self.push_op(ImOp::SLX(*idx));
            },
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                self.s_operon_arr(a, i, op, ex),
            Expr::TblFd(t, f) => self.s_operon_tbl(t, f, op, ex),
            _ => panic!("cannot operon to {lhs:?}"),
        }
    }*/

/*    #[inline]
    fn s_operon_arr( // a_i oo e.
        &mut self,
        a: &Expr,
        i: &Expr,
        o: &BinOpcode,
        e: &Expr)
    {
        self.expr(a);            // a
        self.push_op(ImOp::DUP); // a, a
        self.expr(i);            // a, a, i
        self.push_op(ImOp::DUP); // a, a, i, i
        self.push_op(ImOp::ROT); // a, i, a, i
        self.push_op(ImOp::AGE); // a, i, a_i
        self.expr(e);            // a, i, a_i, e
        self.push_binop(o);      // a, i, a_i o e
        self.push_op(ImOp::ASE); // Ø
    }*/

/*    #[inline]
    fn s_operon_tbl( // t$f oo e.
        &mut self,
        t: &Expr,
        f: &Rc<DfStr>,
        o: &BinOpcode,
        e: &Expr)
    {
        self.expr(t);             // ... t
        self.push_op(ImOp::DUP);      // t, t
        let idx = self.push_ident(f);
        self.push_op(ImOp::TGF(idx)); // t, t$f
        self.expr(e);                 // t, t$f, e
        self.push_binop(o);           // t, t$f o e
        self.push_op(ImOp::TSF(idx)); // t with f=t$f o e
        self.push_op(ImOp::POP);      // Ø
    }*/

/*    fn s_arrass(
        &mut self,
        arr: &Expr,
        idx: &Expr,
        exp: &Expr)
    {
        self.expr(arr);
        self.expr(idx);
        self.expr(exp);
        self.push_op(ImOp::ASE);
    }*/

/*    fn s_tblass(
        &mut self,
        t: &Expr,
        f: &Rc<DfStr>,
        e: &Expr)
    {
        self.expr(t);
        self.expr(e);
        let idx = self.push_ident(f);
        self.push_op(ImOp::TSF(idx));
        self.push_op(ImOp::POP);
    }*/

    /*fn s_ifelse(
        &mut self,
        if_0: &IfCase,
        eifs: &[IfCase],
        elze: &Option<Block>)
    {
        // IDEA: a variable for the last JFX þat will go to þe next case,
        // & an accumulator for all the JJX þat will go to þe end of þe else
        let mut last_if_idx;
        let mut jjx_idxs = vec![];
        // 0st if case
        self.expr(&if_0.cond);
        last_if_idx = self.term_curr_bb(Term::PCH(true));
        self.block(&if_0.blok);
        jjx_idxs.push(self.term_curr_bb(Term::PCH(false)));
        // oþer if cases
        for elseif in eifs {
            self.curr.patch_jump(
                last_if_idx, Term::JFX(self.curr_idx()));
            self.expr(&elseif.cond);
            last_if_idx = self.term_curr_bb(Term::PCH(true));
            self.block(&elseif.blok);
            jjx_idxs.push(self.term_curr_bb(Term::PCH(false)));
        }
        // opt else
        let last_patch = if let Some(eb) = elze {
            self.block(eb);
            self.term_curr_bb(Term::NOP)
        } else { // connect last if to the end
            self.curr_idx()
        };
        // join last If to Else
        self.curr.patch_jump(last_if_idx, Term::JFX(last_patch));
        // close all
        let eo_if = self.curr_idx();
        for i in jjx_idxs {
            self.curr.patch_jump(i, Term::JJX(eo_if));
        }
    }*/

/*    fn s_switch(&mut self,
        mat: &Expr,
        cas: &[SwCase],
        def: &Block)
    {
        // prepare þe matchee
        self.expr(mat);
        // early optimization
        if cas.is_empty() {
            self.push_op(ImOp::POP);
            self.block(def);
            return;
        }
        // code following, very similar to s_ifstmt
        // do þe cases
        let mut last_case_idx;
        let mut jjx_idxs = vec![];
        // 0st case, coz `cas` is !mt here
        self.push_op(ImOp::DUP);
        self.expr(&cas[0].comp);
        last_case_idx = self.term_curr_bb(Term::PCH(true)); // JNX
        self.block(&cas[0].blok);
        jjx_idxs.push(self.term_curr_bb(Term::PCH(false))); // JJX
        // oþer if cases
        for c in cas {
            self.curr.patch_jump(
                last_case_idx, Term::JNX(self.curr_idx()));
            // DUP (mat mat)
            // [c.comp]
            // JNX next case
            // [c.blok]
            // JJX default
            self.push_op(ImOp::DUP);
            self.expr(&c.comp);
            last_case_idx = self.term_curr_bb(Term::PCH(true));
            self.block(&c.blok);
            jjx_idxs.push(self.term_curr_bb(Term::PCH(false)));
        }
        // default case, even if mt, it will be optimized away
        self.block(def);
        let last_patch = self.term_curr_bb(Term::NOP);
        // join last case to default
        self.curr.patch_jump(last_case_idx, Term::JNX(last_patch));
        // patch all jjx's
        let end = self.curr_idx();
        for i in jjx_idxs {
            self.curr.patch_jump(i, Term::JJX(end));
        }
        // POP þe matchee
        self.push_op(ImOp::POP);
    }*/

    fn s_loooop(&mut self, lo: &LoopWt)
    {
        self.curr.init_scope();
        self.preload_loop(lo);
        match lo {
            LoopWt::Inf(b)       => self.s_inf_loop(b),
            LoopWt::Cdt(p, e, b) => self.s_cdt_loop(p, e, b),
        }
        self.curr.exit_scope();
    }

    // assigns all loop's locals to 0%N
    // so as not to enter & exit its scope at every iter
    fn preload_loop(&mut self, lo: &LoopWt)
    {
        let block = match lo {
            LoopWt::Inf(b) |
            LoopWt::Cdt(b, _, _) => b, // will check 2nd block later
        };
        self.preload_block(block);
        if let LoopWt::Cdt(_, _, b) = lo {
            self.preload_block(b);
        }
    }

    // helper
    fn preload_block(&mut self, block: &BlockWt)
    {
        for s in block {
            if let StmtWt::Declar(i, _) = s {
                self.curr.declar_dummy(i);
            }
        }
    }

    fn s_inf_loop(&mut self, b: &BlockWt)
    {
        /*
        **  [b]<-+ (h)
        **   |   |
        **   +---+
        */
        self.term_curr_bb(Term::NOP); // start b
        let h = self.curr_idx();
        self.curr.init_loop(h);
        self.no_env_block(b);
        self.term_curr_bb(Term::JMP(Jmp::JX, h));
        self.curr.exit_loop(self.curr_idx());
    }

    fn s_cdt_loop(&mut self,
        b0:   &BlockWt,    // miȝt be empty
        cond: &ExprWt,
        b1:   &BlockWt)
    {
        /*
        **  [b0]<--+
        **   V     |
        **  [cond]-|-+ (if false)
        **   V     | |
        **  [b1]---+ |
        **  ...<-----+
        */
        self.term_curr_bb(Term::NOP);
        let loop_start = self.curr_idx();
        self.curr.init_loop(loop_start);
        self.no_env_block(b0);
        self.expr(cond);
        let branch = self.term_curr_bb(Term::PCH(true));
        self.no_env_block(b1);
        self.term_curr_bb(Term::JMP(Jmp::JX, loop_start));
        let outside = self.curr_idx();
        self.curr.exit_loop(outside);
        self.curr.patch_jump(branch, Term::JMP(Jmp::YX(false), outside));
    }

/*    fn s_pccall(&mut self, proc: &Expr, args: &[Expr])
    {
        self.expr(proc);
        for a in args {
            self.expr(a);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in proc call: max 255");
        self.push_op(ImOp::PCL(ari));
    }*/

/*    fn s_return(&mut self, e: &Expr)
    {
        self.expr(e);
        self.term_curr_bb(Term::RET);
    }*/

/*    fn s_againl(&mut self, lev: u32)
    {
        let loop_start = self.curr.agn.peek(lev as usize)
            .expect("@@ too deep, þer'r no so many levels");
        self.term_curr_bb(Term::JJX(*loop_start));
    }*/

/*    fn s_breakl(&mut self, lev: u32)
    {
        let here = self.curr_idx();
        self.curr.brk.peek_mut(lev as usize)
            .expect(".@ too deep, þer'r no so many levels")
            .push(here);
    }*/

    fn expr(&mut self, ex: &ExprWt)
    {
        match &ex.e {
            ExprWte::Const(v)       => self.e_const(v),
            ExprWte::Local(i, d)    => self.curr.llx(i, *d),
//            ExprWte::Tcast(t, e)    => self.e_tcast(t, e),
            ExprWte::UniOp(e, o)    => self.e_uniop(e, o),
            ExprWte::BinOp(l, o, r) => self.e_binop(l, o, r),
            ExprWte::CmpOp(f, v)    => self.e_cmpop(f, v),
            _ => todo!(),
//            ExprWte::CmpOp(l, v)    => self.e_cmpop(l, v),
/*            Expr::Array(a)       => self.e_array(a),
            Expr::Table(v)       => self.e_table(v),
            Expr::TblFd(t, f)    => self.e_tblfd(t, f),
            Expr::RecsT(l)       => self.e_recst(*l),
            Expr::FnDef(s)       => self.e_fndef(&s.borrow()),
            Expr::Fcall(f, a)    => self.e_fcall(f, a),
            Expr::TbFcl(t, f, a) => self.obj_call(t, f, a, SubrType::F),
            Expr::PcDef(s)       => self.e_pcdef(&s.borrow()),
            Expr::RecFn |
            Expr::RecPc => self.push_op(ImOp::LLX(0)), // unchecked
            Expr::IfExp(c, e)    => self.e_ifexp(c, e),*/
        }
    }

    fn e_const(&mut self, v: &Val)
    {
        // TODO: special cases like B(T), B(F), N([0..3])
        match v {
            Val::N(1) => {self.push_op(ImOp::LN1); return;},
            Val::R(r) => {
                if (r - 1.0).abs() < f64::EPSILON {
                    self.push_op(ImOp::LR1);
                    return;
                }
            }
            _ => {}, // below
        }
        let idx = self.intern_ctn(v);
        self.push_op(ImOp::LKX(idx));
    }

/*    fn e_tcast(&mut self, t: &Type, e: &Expr)
    {
        self.expr(e);
        match t {
            Type::Z => self.push_op(ImOp::CAZ),
            Type::R => self.push_op(ImOp::CAR),
            Type::N => self.push_op(ImOp::CAN),
            _ => todo!(),
        }
    }*/

    fn e_uniop(&mut self, e: &ExprWt, o: &UniOpWt)
    {
        self.expr(e);
        self.push_op(ImOp::UNO(*o));
    }

    fn e_binop(&mut self, l: &ExprWt, o: &BinOpWt, r: &ExprWt)
    {
/*        if o.is_sce() {
            self.e_bin_sce(l, o, r);
            return;
        }*/
        self.expr(l);
        self.expr(r);
        self.push_op(ImOp::BIO(*o));
    }

/*    fn e_bin_sce(&mut self, l: &Expr, o: &BinOpcode, r: &Expr)
    {
        self.expr(l);
        let branch_i = self.term_curr_bb(Term::PCH(true));
        self.push_op(ImOp::POP); // unneeded lhs term
        self.expr(r);
        self.term_curr_bb(Term::NOP);
        // patch
        let here = self.curr_idx();
        self.curr.patch_jump(branch_i, match o {
            BinOpcode::Cand => Term::JBF(here), // F & * = F
            BinOpcode::Cor  => Term::JBT(here), // T | * = T
            _ => unreachable!(),
        });
    }*/

    fn e_cmpop(&mut self, l: &ExprWt, v: &[(CmpOpWt, ExprWt)])
    {
        self.expr(l);
        match v.len() {
            0 => return,
            1 => { // normal cmpop
                self.expr(&v[0].1);
                self.push_op(ImOp::CMP(v[0].0));
            },
            _ => todo!("multi cmpop"),
        }
    }

/*    fn e_array(&mut self, a: &[Expr])
    {
        self.push_op(ImOp::AMN);
        for e in a {
            self.expr(e);
            self.push_op(ImOp::APE);
        }
    }*/

/*    fn e_table(&mut self, v: &[(Rc<DfStr>, Expr)])
    {
        self.push_op(ImOp::TMN);
        self.curr.rect.push(self.locsize()); // new $@0 will be on þe stack
        self.incloc();
        for (f, e) in v {
            self.expr(e);
            let idx = self.push_ident(f);
            self.push_op(ImOp::TSF(idx));
        }
        self.curr.rect.pop();
        self.decloc();
    }*/

/*    fn e_tblfd(&mut self, t: &Expr, f: &Rc<DfStr>)
    {
        self.expr(t);
        let idx = self.push_ident(f);
        self.push_op(ImOp::TGF(idx));
    }*/

/*    fn e_recst(&mut self, level: u32)
    {
        let Some(loc) = self.curr.rect.peek(level as usize) else {
            panic!("$@{level} too deep");
        };
        self.push_op(ImOp::LLX(*loc));
    }*/

/*    pub fn e_fndef(&mut self, subr: &Subr)
    {
        let pagidx = self.comp_subr(subr, SubrType::F);
        for id in &subr.upvs {
            self.e_ident(id);
        }
        self.push_op(ImOp::FMN(pagidx));
    }*/

/*    pub fn e_pcdef(&mut self, subr: &Subr)
    {
        let pagidx = self.comp_subr(subr, SubrType::P);
        for id in &subr.upvs {
            self.e_ident(id);
        }
        self.push_op(ImOp::PMN(pagidx));
    }*/

/*    pub fn e_fcall(&mut self, func: &Expr, args: &[Expr])
    {
        self.expr(func);
        for arg in args {
            self.expr(arg);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in func call: max 255");
        self.push_op(ImOp::FCL(ari));
    }*/

/*    fn e_ifexp(&mut self, cases: &[(Expr, Expr)], elze: &Expr)
    {
        // copypasted from stmt if else
        macro_rules! if_case {
            ($zelf:ident, $li:ident, $jjx:ident, $c:expr) => {
                $zelf.expr(&$c.0);
                $li = $zelf.term_curr_bb(Term::PCH(true));
                $zelf.expr(&$c.1);
                $jjx.push($zelf.term_curr_bb(Term::PCH(false)));
            };
        }
        let mut last_if_idx;
        let mut jjx_idxs = vec![];
        if_case!(self, last_if_idx, jjx_idxs, cases[0]);
        for c in &cases[1..] {
            self.curr.patch_jump(last_if_idx, Term::JFX(self.curr_idx()));
            if_case!(self, last_if_idx, jjx_idxs, c);
        }
        self.expr(elze);
        let last_patch = self.term_curr_bb(Term::NOP);
        self.curr.patch_jump(last_if_idx, Term::JFX(last_patch));
        let end = self.curr_idx();
        for i in jjx_idxs {
            self.curr.patch_jump(i, Term::JJX(end));
        }
    }*/

/*    pub fn obj_call(
        &mut self,
        obj: &Expr,
        field: &Rc<DfStr>,
        args: &[Expr],
        st: SubrType)
    {
        /*
        **  [obj]
        **  DUP
        **  TGF (field)
        **  SWP
        **  [args...]
        **  [FP]CL [#args + 1] // +1 bcoz obj itself is passed
        */
        self.expr(obj);
        self.push_op(ImOp::DUP);
        let idx = self.push_ident(field);
        self.push_op(ImOp::TGF(idx));
        self.push_op(ImOp::SWP);
        for a in args {
            self.expr(a);
        }
        let ari = u8::try_from(args.len() + 1)
            .expect("too many args in func call: max 255");
        self.push_op(match st {
            SubrType::F => ImOp::FCL(ari),
            SubrType::P => ImOp::PCL(ari),
        });
    }*/

/*    // helper fn
    fn declar_upvs(&mut self, upvs: &[Rc<DfStr>])
    {
        for upv in upvs {
            let idfidx = self.push_ident(upv);
            self.curr.upvals.add(idfidx);
        }
    }

    pub fn comp_subr(&mut self, s: &Subr, stype: SubrType) -> PagIdx
    {
        let outer = std::mem::take(&mut self.curr);
        self.declar_upvs(&s.upvs);
        self.incloc(); // !@ xor #@
        for par in &s.pars {
            self.new_local(par);
        }
        self.block(&s.body);
        match stype {
            SubrType::F => self.term_curr_bb(Term::HLT),
            SubrType::P => self.term_curr_bb(Term::END),
        };
        let low_name = s.meta.name.as_ref().map(|n| self.push_ident(n));
        let m = PageMeta { line: s.meta.line, name: low_name };
        return self.term_subr(s.arity(), s.upvs.len(), m, outer);
    }*/
}

impl Default for Compiler
{
    fn default() -> Self
    {
        Self {
            consts: ArraySet::default(),
            idents: ArraySet::default(),
            //subrs:  vec![],
            curr: SubrEnv::default(),
        }
    }
}
