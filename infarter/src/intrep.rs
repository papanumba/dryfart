/* intrep.rs */

use std::rc::Rc;
use crate::{util::*, asterix::*};

// Intermediate Opcodes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImOp
{
    LVV,
    LBX(bool),
    LN0,
    LN1,
    LN2,
    LN3,
    LM1,
    LZ0,
    LZ1,
    LZ2,
    LR0,
    LR1,
    LKX(CtnIdx),

    NEG,
    ADD,
    SUB,
    MUL,
    DIV,
    INV,
    INC,
    DEC,
    MOD,

    CEQ,
    CNE,
    CLT,
    CLE,
    CGT,
    CGE,

    NOT,
    AND,
    IOR,
    XOR,

    LGX(DfStrIdx),
    SGX(DfStrIdx),
    LLX(LocIdx),
    SLX(LocIdx),
    ULX(LocIdx),

    AMN,
    APE,
    AGE,
    ASE,

    TMN,
    TSF(DfStrIdx),
    TGF(DfStrIdx),

    FMN(PagIdx),
    FCL(u8),

    PMN(PagIdx),
    PCL(u8), // called arity

    LUV(UpvIdx), // Load UpValue (from current norris)

    CAN,
    CAZ,
    CAR,

    DUP,
    SWP,
    ROT,
    POP,
    // TODO: add opcodes
}

impl ImOp
{
    pub fn get_operand(&self) -> Option<usize>
    {
        match self {
            ImOp::LKX(i) |
            ImOp::LGX(i) |
            ImOp::SGX(i) |
            ImOp::LLX(i) |
            ImOp::SLX(i) |
            ImOp::ULX(i) => return Some(*i),
            _ => None,
        }
    }

    pub fn is_tbl(&self) -> bool
    {
        matches!(self, ImOp::TGF(_) | ImOp::TSF(_))
    }

    pub fn is_subr(&self) -> bool
    {
        matches!(self,
            ImOp::PMN(_) |
            ImOp::PCL(_) |
            ImOp::FMN(_) |
            ImOp::FCL(_) |
            ImOp::LUV(_)
        )
    }
}

// Addressing modes
type    CtnIdx = usize; // in constant pool
type  DfStrIdx = usize; // in identifier pool
type    LocIdx = usize; // in þe stack
type    UpvIdx = usize; // in þe curr subr's upv arr
pub type BbIdx = usize; // in Cfg's BasicBlock vec
type    PagIdx = usize; // in bytecode pages for subroutines

// terminators
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Term
{
    #[default]
    NOP,        // just inc þe bbidx for þe next block
    JJX(BbIdx), // contain a index for a basic block target
    JBT(BbIdx),
    JBF(BbIdx),
    JTX(BbIdx),
    JFX(BbIdx),
    JEX(BbIdx),
    JNX(BbIdx),
    JLT(BbIdx),
    JLE(BbIdx),
    JGT(BbIdx),
    JGE(BbIdx),
    RET,
    END,
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
            Term::PCH(b) => *b,
            Term::NOP    |
            Term::JBT(_) |
            Term::JBF(_) |
            Term::JTX(_) |
            Term::JFX(_) |
            Term::JEX(_) |
            Term::JNX(_) |
            Term::JLT(_) |
            Term::JLE(_) |
            Term::JGT(_) |
            Term::JGE(_) => true,
            _ => false,
        }
    }

    pub fn jmp_target(&self) -> Option<BbIdx>
    {
        match self {
            Term::JJX(i) |
            Term::JBT(i) |
            Term::JBF(i) |
            Term::JTX(i) |
            Term::JFX(i) |
            Term::JEX(i) |
            Term::JNX(i) |
            Term::JLT(i) |
            Term::JLE(i) |
            Term::JGT(i) |
            Term::JGE(i) => Some(*i),
            _ => None,
        }
    }

    // panics if self is not jmp
    pub fn set_jmp_target(&mut self, new_i: BbIdx)
    {
        match &mut *self {
            Term::JJX(i) |
            Term::JBT(i) |
            Term::JBF(i) |
            Term::JTX(i) |
            Term::JFX(i) |
            Term::JEX(i) |
            Term::JNX(i) |
            Term::JLT(i) |
            Term::JLE(i) |
            Term::JGT(i) |
            Term::JGE(i) => *i = new_i,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BasicBlock
{
    pub code: Vec<ImOp>,       // non-terminating ops
    pub term: Term,            // successors
    pub pred: ArraySet<BbIdx>, // predecessors, used in optimus
}

impl BasicBlock
{
    pub fn push(&mut self, imop: ImOp)
    {
        self.code.push(imop);
    }
}

#[derive(Debug, Default)]
pub struct SubrEnv // subroutine environment compiler
{
    pub scpdpt:   usize,
    pub presize:  usize,
    pub locsize:  usize,
    pub locals:   VecMap<DfStrIdx, LocIdx>, // name, stack index
    pub upvals:   ArraySet<DfStrIdx>, // upvalue names
    pub blocks:   Vec<BasicBlock>,  // graph arena
    pub curr:     BasicBlock,       // current working bblock
    pub rect:     Stack<LocIdx>,    // accumulating $@N
    pub agn:      Stack<BbIdx>,     // stack of þe loops from outer to inner
                                    // þe indices are each loop's start
    pub brk:      Stack<Vec<BbIdx>>,// þis one is a stack for each loop
                                    // all þe blocks wiþ terms to be patched
}

impl SubrEnv
{
    pub fn enter_scope(&mut self)
    {
        self.presize = self.locsize;
        self.scpdpt += 1;
    }

    pub fn exit_scope(&mut self)
    {
        assert!(self.scpdpt != 0);
        for _ in self.presize..self.locals.size() {
            self.push_op(ImOp::POP);
        }
        self.scpdpt -= 1;
        self.locals.trunc(self.presize);
    }

    #[inline]
    fn push_op(&mut self, op: ImOp)
    {
        self.curr.push(op);
    }

    #[inline]
    fn curr_idx(&self) -> BbIdx
    {
        return self.blocks.len();
    }

    fn assign(&mut self, idx: DfStrIdx)
    {
        // if exists local, it's an assign
        if let Some(i) = self.locals.get(&idx) {
            self.push_op(ImOp::SLX(*i));
        } else { // it's a declar, even if þer's an upvale, it will shadow
            self.locals.set(idx, self.locsize);
            self.locsize += 1;
        }
    }

    fn term_curr_bb(&mut self, t: Term) -> BbIdx // of þe termed block
    {
        self.curr.term = t;
        let last_idx  = self.curr_idx();
        let last_term = self.curr.term;
        let aux = std::mem::take(&mut self.curr);
        self.blocks.push(aux);
        if last_term.can_thru() { // NOP, JF, etc.
            self.curr.pred.add(last_idx);
        }
        return last_idx;
    }

    fn patch_jump(&mut self, from: BbIdx, term: Term)
    {
        if let Some(t) = term.jmp_target() {
            self.blocks[from].term = term;
            if t == self.curr_idx() {
                self.curr.pred.add(from);
            } else {
                self.blocks[t].pred.add(from);
            }
        } else {
            unreachable!();
        }
    }

    pub fn start_loop(&mut self, start_bbi: BbIdx)
    {
        self.agn.push(start_bbi);
        self.brk.push(vec![]);
    }

    pub fn end_loop(&mut self, end_bbi: BbIdx)
    {
        self.agn.pop();
        let patches = self.brk.pop_last()
            .unwrap();
        let jj = Term::JJX(end_bbi);
        for p in patches {
            self.patch_jump(p, jj);
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
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
}

#[derive(Debug)]
pub struct Compiler
{
    pub consts:  ArraySet<Val>,       // constant pool
    pub idents:  ArraySet<Rc<DfStr>>, // identifier pool
    pub subrs:   Vec<Page>,
    pub curr:    SubrEnv,
}

impl Compiler
{
    pub fn from_asterix(main: &Block) -> Self
    {
        let mut program = Self::default();
        program.subrs.push(Page::default()); // dummy main
        program.no_env_block(main);
        program.term_curr_bb(Term::HLT);
        // here program.curr will be þe main proc
        program.subrs[0].code = std::mem::take(&mut program.curr).blocks;
        return program;
    }

    #[inline]
    fn locsize(&self) -> usize
    {
        return self.curr.locsize;
    }

    #[inline]
    fn incloc(&mut self)
    {
        self.curr.locsize += 1;
    }

    #[inline]
    fn decloc(&mut self)
    {
        assert!(self.curr.locsize != 0);
        self.curr.locsize -= 1;
    }

    #[inline]
    fn push_ident(&mut self, id: &Rc<DfStr>) -> DfStrIdx
    {
        if let Some(i) = self.idents.index_of(id) {
            i
        } else {
            self.idents.add(id.clone()) // return þe new index
        }
    }

    #[inline]
    fn push_const(&mut self, v: &Val) -> CtnIdx
    {
        if let Some(i) = self.consts.index_of(v) {
            i
        } else {
            self.consts.add(v.clone()) // return þe new index
        }
    }

    #[inline]
    fn term_subr(&mut self,
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
    }

    #[inline]
    fn push_op(&mut self, op: ImOp)
    {
        self.curr.push_op(op);
    }

    #[inline]
    fn push_uniop(&mut self, o: &UniOpcode)
    {
        self.push_op(match o {
            UniOpcode::Neg => ImOp::NEG,
            UniOpcode::Not => ImOp::NOT,
            UniOpcode::Inv => ImOp::INV,
        });
    }

    #[inline]
    fn push_binop(&mut self, o: &BinOpcode)
    {
        self.push_op(match o {
            BinOpcode::Add => ImOp::ADD,
            BinOpcode::Sub => ImOp::SUB,
            BinOpcode::Mul => ImOp::MUL,
            BinOpcode::Div => ImOp::DIV,
            BinOpcode::Mod => ImOp::MOD,
            BinOpcode::And => ImOp::AND,
            BinOpcode::Or  => ImOp::IOR,
            BinOpcode::Xor => ImOp::XOR,
            BinOpcode::Eq  => ImOp::CEQ,
            BinOpcode::Ne  => ImOp::CNE,
            BinOpcode::Lt  => ImOp::CLT,
            BinOpcode::Le  => ImOp::CLE,
            BinOpcode::Gt  => ImOp::CGT,
            BinOpcode::Ge  => ImOp::CGE,
            BinOpcode::Idx => ImOp::AGE,
            _ => unreachable!(),
        });
    }

    #[inline]
    fn term_curr_bb(&mut self, t: Term) -> BbIdx
    {
        return self.curr.term_curr_bb(t);
    }

    #[inline]
    fn curr_idx(&self) -> BbIdx
    {
        return self.curr.curr_idx();
    }

    #[inline]
    fn resolve_local(&self, id: &Rc<DfStr>) -> Option<&LocIdx>
    {
        let idx = self.idents.index_of(id)?;
        return self.curr.locals.get(&idx);
    }

    #[inline]
    fn resolve_upval(&self, id: &Rc<DfStr>) -> Option<UpvIdx>
    {
        let idx = self.idents.index_of(id)?;
        return self.curr.upvals.index_of(&idx);
    }

    #[inline]
    fn exists_var(&self, id: &Rc<DfStr>) -> bool
    {
        return self.resolve_local(id).is_some()
            || self.resolve_upval(id).is_some();
    }

    fn block(&mut self, b: &Block)
    {
        if b.is_empty() {
            return;
        }
        let presize = self.locsize();
        self.curr.enter_scope();
        self.no_env_block(b);
        self.curr.presize = presize;
        self.curr.exit_scope();
    }

    #[inline]
    fn no_env_block(&mut self, b: &Block) //-> Patches
    {
        for s in b {
            self.stmt(s);
        }
    }

    fn stmt(&mut self, s: &Stmt)
    {
        match s {
            Stmt::Assign(v, e)    => self.s_assign(v, e),
            Stmt::OperOn(l, o, e) => self.s_operon(l, o, e),
            Stmt::IfElse(i, o, e) => self.s_ifelse(i, o, e),
            Stmt::Switch(m, c, d) => self.s_switch(m, c, d),
            Stmt::LoopIf(l)       => self.s_loopif(l),
            Stmt::PcCall(p, a)    => self.s_pccall(p, a),
            Stmt::TbPCal(t, f, a) => self.obj_call(t, f, a, SubrType::P),
            Stmt::PcExit          => {self.term_curr_bb(Term::END);},
            Stmt::Return(e)       => self.s_return(e),
            Stmt::AgainL(l)       => self.s_againl(*l),
            Stmt::BreakL(l)       => self.s_breakl(*l),
        }
    }

    fn s_assign(&mut self, v: &Expr, ex: &Expr)
    {
        match v {
            Expr::Ident(s) => self.s_varass(s, ex),
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                return self.s_arrass(a, i, ex),
            Expr::TblFd(t, f) =>
                return self.s_tblass(t, f, ex),
            _ => panic!("cannot assign to {v:?}"),
        }
    }

    #[inline]
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
    }

    #[inline]
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
    }

    #[inline]
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
    }

    #[inline]
    fn new_local(&mut self, id: &Rc<DfStr>)
    {
        let idx = self.push_ident(id);
        self.curr.assign(idx);
    }

    fn s_varass(&mut self, id: &Rc<DfStr>, ex: &Expr)
    {
        self.expr(ex);
        self.new_local(id);
    }

    fn s_arrass(
        &mut self,
        arr: &Expr,
        idx: &Expr,
        exp: &Expr)
    {
        self.expr(arr);
        self.expr(idx);
        self.expr(exp);
        self.push_op(ImOp::ASE);
    }

    fn s_tblass(
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
    }

    fn s_ifelse(
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
    }

    fn s_switch(&mut self,
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
    }

    fn s_loopif(&mut self, lo: &Loop)
    {
        self.curr.enter_scope();
        self.lvv_loop(lo);
        match lo {
            Loop::Inf(b)       => self.s_inf_loop(b),
            Loop::Cdt(p, e, b) => self.s_cdt_loop(p, e, b),
        }
        self.curr.exit_scope();
    }

    // assigns all loop's locals to Void
    // so as not to enter & exit its scope at every
    fn lvv_loop(&mut self, lo: &Loop)
    {
        let block = match lo {
            Loop::Inf(b) |
            Loop::Cdt(b, _, _) => b, // will check 2nd block later
        };
        self.lvv_in_block(block);
        if let Loop::Cdt(_, _, b) = lo {
            self.lvv_in_block(b);
        }
    }

    // helper
    fn lvv_in_block(&mut self, block: &Block)
    {
        for s in block {
            if let Stmt::Assign(v, _) = s {
                if let Expr::Ident(i) = v {
                    if !self.exists_var(i) { // is new locar var
                        self.s_assign(v, &Expr::Const(Val::V));
                    }
                }
            }
        }
    }

    fn s_inf_loop(&mut self, b: &Block)
    {
        /*
        **  [b]<-+ (h)
        **   |   |
        **   +---+
        */
        self.term_curr_bb(Term::NOP); // start b
        let h = self.curr_idx();
        self.curr.start_loop(h);
        self.no_env_block(b);
        self.term_curr_bb(Term::JJX(h));
        self.curr.end_loop(self.curr_idx());
    }

    fn s_cdt_loop(&mut self,
        b0:   &Block,    // miȝt be empty
        cond: &Expr,
        b1:   &Block)
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
        self.curr.start_loop(loop_start);
        self.no_env_block(b0);
        self.expr(cond);
        let branch = self.term_curr_bb(Term::PCH(true));
        self.no_env_block(b1);
        self.term_curr_bb(Term::JJX(loop_start));
        let outside = self.curr_idx();
        self.curr.end_loop(outside);
        self.curr.patch_jump(branch, Term::JFX(outside));
    }

    fn s_pccall(&mut self, proc: &Expr, args: &[Expr])
    {
        self.expr(proc);
        for a in args {
            self.expr(a);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in proc call: max 255");
        self.push_op(ImOp::PCL(ari));
    }

    fn s_return(&mut self, e: &Expr)
    {
        self.expr(e);
        self.term_curr_bb(Term::RET);
    }

    fn s_againl(&mut self, lev: u32)
    {
        let loop_start = self.curr.agn.peek(lev as usize)
            .expect("@@ too deep, þer'r no so many levels");
        self.term_curr_bb(Term::JJX(*loop_start));
    }

    fn s_breakl(&mut self, lev: u32)
    {
        let here = self.curr_idx();
        self.curr.brk.peek_mut(lev as usize)
            .expect(".@ too deep, þer'r no so many levels")
            .push(here);
    }

    fn expr(&mut self, ex: &Expr)
    {
        match ex {
            Expr::Const(v)       => self.e_const(v),
            Expr::Ident(i)       => self.e_ident(i),
            Expr::Tcast(t, e)    => self.e_tcast(t, e),
            Expr::UniOp(e, o)    => self.e_uniop(e, o),
            Expr::BinOp(l, o, r) => self.e_binop(l, o, r),
            Expr::CmpOp(l, v)    => self.e_cmpop(l, v),
            Expr::Array(a)       => self.e_array(a),
            Expr::Table(v)       => self.e_table(v),
            Expr::TblFd(t, f)    => self.e_tblfd(t, f),
            Expr::RecsT(l)       => self.e_recst(*l),
            Expr::FnDef(s)       => self.e_fndef(&s.borrow()),
            Expr::Fcall(f, a)    => self.e_fcall(f, a),
            Expr::TbFcl(t, f, a) => self.obj_call(t, f, a, SubrType::F),
            Expr::PcDef(s)       => self.e_pcdef(&s.borrow()),
            Expr::RecFn |
            Expr::RecPc => self.push_op(ImOp::LLX(0)), // unchecked
            Expr::IfExp(c, e)    => self.e_ifexp(c, e),
        }
    }

    // þis checks predefined consts
    fn e_const(&mut self, v: &Val)
    {
        match v {
            Val::V => return self.push_op(ImOp::LVV),
            Val::B(b) => return self.push_op(ImOp::LBX(*b)),
            Val::N(n) => match n {
                0 => return self.push_op(ImOp::LN0),
                1 => return self.push_op(ImOp::LN1),
                2 => return self.push_op(ImOp::LN2),
                3 => return self.push_op(ImOp::LN3),
                _ => {},
            },
            Val::Z(z) => match z {
                -1 => return self.push_op(ImOp::LM1),
                0 => return self.push_op(ImOp::LZ0),
                1 => return self.push_op(ImOp::LZ1),
                2 => return self.push_op(ImOp::LZ2),
                _ => {},
            },
            // oþers must be internalized
            Val::C(_) |
            Val::R(_) |
            Val::T(Table::Nat(_)) |
            Val::A(_) => {},
            _ => todo!("oþer consts {:?}", v),
        }
        self.e_new_const(v);
    }

    // called when self couldn't find a predefined L op
    fn e_new_const(&mut self, v: &Val)
    {
        let idx = self.push_const(v);
        self.push_op(ImOp::LKX(idx));
    }

    fn e_ident(&mut self, id: &Rc<DfStr>)
    {
        if id.as_bytes() == b"STD" {
            todo!("STD");
        }
        if let Some(i) = self.resolve_local(id) {
            self.push_op(ImOp::LLX(*i));
            return;
        }
        if let Some(i) = self.resolve_upval(id) {
            self.push_op(ImOp::LUV(i));
            return;
        }
        panic!("could not resolve symbol {id}");
    }

    fn e_tcast(&mut self, t: &Type, e: &Expr)
    {
        self.expr(e);
        match t {
            Type::Z => self.push_op(ImOp::CAZ),
            Type::R => self.push_op(ImOp::CAR),
            Type::N => self.push_op(ImOp::CAN),
            _ => todo!(),
        }
    }

    fn e_uniop(&mut self, e: &Expr, o: &UniOpcode)
    {
        self.expr(e);
        self.push_uniop(o);
    }

    fn e_binop(&mut self, l: &Expr, o: &BinOpcode, r: &Expr)
    {
        if o.is_sce() {
            self.e_bin_sce(l, o, r);
            return;
        }
        self.expr(l);
        self.expr(r);
        self.push_binop(o);
    }

    fn e_bin_sce(&mut self, l: &Expr, o: &BinOpcode, r: &Expr)
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
    }

    fn e_cmpop(&mut self, l: &Expr, v: &[(BinOpcode, Expr)])
    {
        self.expr(l);
        match v.len() {
            0 => return,
            1 => { // normal cmpop
                self.expr(&v[0].1);
                self.push_binop(&v[0].0);
            },
            _ => todo!("multi cmpop"),
        }
    }

    fn e_array(&mut self, a: &[Expr])
    {
        self.push_op(ImOp::AMN);
        for e in a {
            self.expr(e);
            self.push_op(ImOp::APE);
        }
    }

    fn e_table(&mut self, v: &[(Rc<DfStr>, Expr)])
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
    }

    fn e_tblfd(&mut self, t: &Expr, f: &Rc<DfStr>)
    {
        self.expr(t);
        let idx = self.push_ident(f);
        self.push_op(ImOp::TGF(idx));
    }

    fn e_recst(&mut self, level: u32)
    {
        let Some(loc) = self.curr.rect.peek(level as usize) else {
            panic!("$@{level} too deep");
        };
        self.push_op(ImOp::LLX(*loc));
    }

    pub fn e_fndef(&mut self, subr: &Subr)
    {
        let pagidx = self.comp_subr(subr, SubrType::F);
        for id in &subr.upvs {
            self.e_ident(id);
        }
        self.push_op(ImOp::FMN(pagidx));
    }

    pub fn e_pcdef(&mut self, subr: &Subr)
    {
        let pagidx = self.comp_subr(subr, SubrType::P);
        for id in &subr.upvs {
            self.e_ident(id);
        }
        self.push_op(ImOp::PMN(pagidx));
    }

    pub fn e_fcall(&mut self, func: &Expr, args: &[Expr])
    {
        self.expr(func);
        for arg in args {
            self.expr(arg);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in func call: max 255");
        self.push_op(ImOp::FCL(ari));
    }

    fn e_ifexp(&mut self, cases: &[(Expr, Expr)], elze: &Expr)
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
    }

    pub fn obj_call(
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
    }

    // helper fn
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
    }
}

impl Default for Compiler
{
    fn default() -> Self
    {
        Self {
            consts: ArraySet::default(),
            idents: ArraySet::default(),
            subrs:  vec![],
            curr: SubrEnv::default(),
        }
    }
}
