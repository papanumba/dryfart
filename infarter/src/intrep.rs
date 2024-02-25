/* src/intrep.rs */

//use std::collections::HashMap;
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

    LGX(IdfIdx),
    SGX(IdfIdx),
    LLX(LocIdx),
    SLX(LocIdx),
    ULX(LocIdx),

    AMN,
    APE,
    AGE,
    ASE,

    TMN,
    TSF(IdfIdx),
    TGF(IdfIdx),

    FMN(PagIdx),
    FCL(u8),

    PMN(PagIdx),
    PCL(u8), // called arity

    CAN,
    CAZ,
    CAR,

    DUP,
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
        match self {
            ImOp::TGF(_) | ImOp::TSF(_) => true,
            _ => false,
        }
    }

    pub fn is_subr(&self) -> bool
    {
        match self {
            ImOp::PMN(_) |
            ImOp::PCL(_) |
            ImOp::FMN(_) |
            ImOp::FCL(_) => true,
            _ => false,
        }
    }
}

// Addressing modes
type CtnIdx = usize; // in constant pool
type IdfIdx = usize; // in identifier pool
type LocIdx = usize; // in þe stack
pub type BbIdx = usize; // in Cfg's BasicBlock vec
type PagIdx = usize; // in bytecode pages for subroutines

// terminators
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Term
{
    #[default]
    NOP,        // just inc þe bbidx for þe next block
    JJX(BbIdx), // contain a index for a basic block target
    JBF(BbIdx),
    JFX(BbIdx),
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
            Term::JBF(_) |
            Term::JFX(_) |
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
            Term::JBF(i) |
            Term::JFX(i) |
            Term::JLT(i) |
            Term::JLE(i) |
            Term::JGT(i) |
            Term::JGE(i) => Some(*i),
            _ => None,
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
    pub locals:   VecMap<IdfIdx, LocIdx>,
    pub blocks:   Vec<BasicBlock>, // graph arena
    pub curr:     BasicBlock,      // current working bblock
    pub rect:     Stack<LocIdx>,   // accumulating $@N
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

    fn assign(&mut self, idx: IdfIdx)
    {
        // if exists local, it's an assign
        if let Some(i) = self.locals.get(&idx) {
            self.push_op(ImOp::SLX(*i));
        } else { // it's a declar
            self.locals.set(idx, self.locsize);
            self.locsize += 1;
        }
    }

    fn term_curr_bb(&mut self, t: Term) -> BbIdx // of þe termed block
    {
        self.curr.term = t;
        let last_idx  = self.curr_idx();
        let last_term = self.curr.term;
        let aux = std::mem::replace(&mut self.curr, BasicBlock::default());
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
}

#[derive(Debug, Default)]
pub struct Page
{
    pub arity: usize,
    pub code:  Vec<BasicBlock>,
}

impl Page
{
    pub fn new(a: usize, c: Vec<BasicBlock>) -> Self
    {
        Self { arity: a, code: c }
    }
}

#[derive(Debug)]
pub struct Compiler<'ast>
{
    pub consts:  ArraySet<&'ast Val>,   // constant pool
    pub idents:  ArraySet<&'ast str>,   // identifier pool
    pub subrs:   Vec<Page>,
    pub curr:    SubrEnv,
}

impl Eq for Val {} // for ArraySet

impl<'a> Compiler<'a>
{
    fn new() -> Self
    {
        Self {
            consts: ArraySet::default(),
            idents: ArraySet::default(),
            subrs:  vec![],
            curr: SubrEnv::default(),
        }
    }

    pub fn from_asterix(main: &'a Block) -> Self
    {
        let mut program = Self::new();
        program.subrs.push(Page::default()); // dummy main
        program.no_env_block(main);
        program.term_curr_bb(Term::HLT);
        // here program.curr will be þe main proc
        program.subrs[0].code = std::mem::replace(
            &mut program.curr,
            SubrEnv::default()
        ).blocks;
        return program;
    }

/*    #[allow(dead_code)]
    pub fn print_edges(&self)
    {
        for (i, x) in self.blocks.iter().enumerate() {
            println!("{:?} -> {}", x.pred, i);
        }
    }*/

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
    fn push_ident(&mut self, id: &'a str) -> IdfIdx
    {
        return self.idents.add(id);
    }

    #[inline]
    fn push_const(&mut self, v: &'a Val) -> CtnIdx
    {
        return self.consts.add(v);
    }

    #[inline]
    fn term_subr(&mut self, arity: usize, outer: SubrEnv) -> PagIdx
    {
        // extract byte code þe dying subrenv
        let curr = std::mem::replace(&mut self.curr, outer);
        let idx = self.subrs.len();
        self.subrs.push(Page::new(arity, curr.blocks));
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
    fn resolve_local(&self, id: &'a str) -> Option<&LocIdx>
    {
        let idx = self.idents.index_of(&id)?;
        return self.curr.locals.get(&idx);
    }

    #[inline]
    fn exists_var(&self, id: &'a str) -> bool
    {
        return self.resolve_local(id).is_some();
    }

    fn block(&mut self, b: &'a Block)
    {
        let presize = self.locsize();
        self.curr.enter_scope();
        self.no_env_block(b);
        self.curr.presize = presize;
        self.curr.exit_scope();
    }

    #[inline]
    fn no_env_block(&mut self, b: &'a Block) //-> Patches
    {
        for s in b {
            self.stmt(s);
        }
    }

    fn stmt(&mut self, s: &'a Stmt)
    {
        match s {
            Stmt::Assign(v, e)    => self.s_assign(v, e),
            Stmt::IfStmt(c, b, e) => self.s_ifstmt(c, b, e),
            Stmt::LoopIf(l)       => self.s_loopif(l),
            Stmt::PcCall(p, a)    => self.s_pccall(p, a),
            Stmt::PcExit          => {self.term_curr_bb(Term::END);},
            Stmt::Return(e)       => self.s_return(e),
            _ => todo!("oþer stmts {:?}", s),
        }
    }

    fn s_assign(&mut self, v: &'a Expr, ex: &'a Expr)
    {
        match v {
            Expr::Ident(s) => self.s_varass(s, ex),
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                return self.s_arrass(a, i, ex),
            Expr::TblFd(t, f) =>
                return self.s_tblass(t, f, ex),
            _ => panic!("cannot assign to {:?}", v),
        }
    }

    #[inline]
    fn new_local(&mut self, id: &'a str)
    {
        let idx = self.push_ident(id);
        self.curr.assign(idx);
    }

    fn s_varass(&mut self, id: &'a str, ex: &'a Expr)
    {
        self.expr(ex);
        self.new_local(id);
    }

    fn s_arrass(
        &mut self,
        arr: &'a Expr,
        idx: &'a Expr,
        exp: &'a Expr)
    {
        self.expr(arr);
        self.expr(idx);
        self.expr(exp);
        self.push_op(ImOp::ASE);
    }

    fn s_tblass(
        &mut self,
        t: &'a Expr,
        f: &'a str,
        e: &'a Expr)
    {
        self.expr(t);
        self.expr(e);
        let idx = self.push_ident(f);
        self.push_op(ImOp::TSF(idx));
        self.push_op(ImOp::POP);
    }

    fn s_ifstmt(&mut self,
        cond: &'a Expr,
        bloq: &'a Block,
        elbl: &'a Option<Block>)
    {
        /*
        **  [cond]--+
        **   V      | (if False)
        **  [bloq]  V
        **   |     [elbl]?
        **   V      |
        **  ... <---+
        */
        self.expr(cond);
        let branch = self.term_curr_bb(Term::PCH(true)); // to patch
        self.block(bloq);
        let end_true = self.term_curr_bb(Term::PCH(false));
        self.curr.patch_jump(branch, Term::JFX(self.curr.curr_idx()));
        if let Some(eb) = elbl {
            self.block(eb);
            self.curr.patch_jump(end_true, Term::JJX(self.curr.curr_idx()));
        } else {
            self.curr.blocks[end_true].term = Term::NOP;
            self.curr.curr.pred.add(end_true);
        }
    }

    fn s_loopif(&mut self, lo: &'a Loop)
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
    fn lvv_loop(&mut self, lo: &'a Loop)
    {
        let block = match lo {
            Loop::Inf(b) => b,
            Loop::Cdt(b, _, _) => b, // will check 2nd block later
        };
        self.lvv_in_block(block);
        if let Loop::Cdt(_, _, b) = lo {
            self.lvv_in_block(b);
        }
    }

    // helper
    fn lvv_in_block(&mut self, block: &'a Block)
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

    fn s_inf_loop(&mut self, b: &'a Block)
    {
        /*
        **  [b]<-+ (h)
        **   |   |
        **   +---+
        */
        self.term_curr_bb(Term::NOP); // start b
        let h = self.curr.curr_idx();
        self.no_env_block(b);
        self.term_curr_bb(Term::JJX(h));
    }

    fn s_cdt_loop(&mut self,
        b0:   &'a Block,    // miȝt be empty
        cond: &'a Expr,
        b1:   &'a Block)
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
        let loop_start = self.curr.curr_idx();
        self.no_env_block(b0);
        self.expr(cond);
        let branch = self.term_curr_bb(Term::PCH(true));
        self.no_env_block(b1);
        self.term_curr_bb(Term::JJX(loop_start));
        self.curr.patch_jump(branch, Term::JFX(self.curr.curr_idx()));
    }

    fn s_pccall(&mut self, proc: &'a Expr, args: &'a [Expr])
    {
        self.expr(proc);
        for a in args {
            self.expr(a);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in proc call: max 255");
        self.push_op(ImOp::PCL(ari));
    }

    fn s_return(&mut self, e: &'a Expr)
    {
        self.expr(e);
        self.term_curr_bb(Term::RET);
    }

    fn expr(&mut self, ex: &'a Expr)
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
            Expr::RecsT(l)       => self.e_recst(l),
            Expr::FnDef(s)       => self.e_fndef(&*s),
            Expr::Fcall(f, a)    => self.e_fcall(f, a),
            Expr::PcDef(s)       => self.e_pcdef(&*s),
            Expr::RecFn |
            Expr::RecPc => self.push_op(ImOp::LLX(0)), // unchecked
        }
    }

    // þis checks predefined consts
    fn e_const(&mut self, v: &'a Val)
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
            Val::R(_) => {},
            _ => todo!("oþer consts {:?}", v),
        }
        self.e_new_const(v);
    }

    // called when self couldn't find a predefined L op
    fn e_new_const(&mut self, v: &'a Val)
    {
        let idx = self.push_const(v);
        self.push_op(ImOp::LKX(idx));
    }

    fn e_ident(&mut self, id: &'a str)
    {
        let i = self.resolve_local(&id)
            .expect(&format!("cannot resolve symbol {id}"));
        self.push_op(ImOp::LLX(*i));
    }


    fn e_tcast(&mut self, t: &Type, e: &'a Expr)
    {
        self.expr(e);
        match t {
            Type::Z => self.push_op(ImOp::CAZ),
            Type::R => self.push_op(ImOp::CAR),
            Type::N => self.push_op(ImOp::CAN),
            _ => todo!(),
        }
    }

    fn e_uniop(&mut self, e: &'a Expr, o: &UniOpcode)
    {
        self.expr(e);
        self.push_uniop(o);
    }

    fn e_binop(&mut self, l: &'a Expr, o: &BinOpcode, r: &'a Expr)
    {
        if o.is_sce() {
            todo!("short circuits");
        }
        self.expr(l);
        self.expr(r);
        self.push_binop(o);
    }

    fn e_cmpop(&mut self, l: &'a Expr, v: &'a [(BinOpcode, Expr)])
    {
        self.expr(l);
        match v.len() {
            0 => {},
            1 => { // normal cmpop
                self.expr(&v[0].1);
                self.push_binop(&v[0].0);
            },
            _ => todo!("multi cmpop"),
        }
    }

    fn e_array(&mut self, a: &'a [Expr])
    {
        self.push_op(ImOp::AMN);
        for e in a {
            self.expr(e);
            self.push_op(ImOp::APE);
        }
    }

    fn e_table(&mut self, v: &'a [(String, Expr)])
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

    fn e_tblfd(&mut self, t: &'a Expr, f: &'a str)
    {
        self.expr(t);
        let idx = self.push_ident(f);
        self.push_op(ImOp::TGF(idx));
    }

    fn e_recst(&mut self, level: &u32)
    {
        if let Some(loc) = self.curr.rect.peek(*level as usize) {
            self.push_op(ImOp::LLX(*loc));
        } else {
            panic!("$@{level} too deep");
        }
    }

    pub fn e_fndef(&mut self, subr: &'a Subr)
    {
        let pagidx = self.comp_subr(subr, SubrType::F);
        self.push_op(ImOp::FMN(pagidx));
    }

    pub fn e_fcall(&mut self, func: &'a Expr, args: &'a [Expr])
    {
        self.expr(func);
        for arg in args {
            self.expr(arg);
        }
        let ari = u8::try_from(args.len())
            .expect("too many args in func call: max 255");
        self.push_op(ImOp::FCL(ari));
    }

    pub fn e_pcdef(&mut self, subr: &'a Subr /* eke upvals here */ )
    {
        let pagidx = self.comp_subr(subr, SubrType::P);
        self.push_op(ImOp::PMN(pagidx));
    }

    pub fn comp_subr(&mut self, s: &'a Subr, stype: SubrType) -> PagIdx
    {
        let outer = std::mem::replace(&mut self.curr, SubrEnv::default());
        self.incloc(); // !@ xor #@
        // declare pars as locals
        for par in &s.pars {
            self.new_local(par);
        }
        self.block(&s.body);
        match stype {
            SubrType::F => self.term_curr_bb(Term::HLT),
            SubrType::P => self.term_curr_bb(Term::END),
        };
        return self.term_subr(s.arity(), outer);
    }
}

impl From<&Type> for u8
{
    fn from(t: &Type) -> u8
    {
        match t {
            Type::B => 0x02,
            Type::C => 0x04,
            Type::N => 0x06,
            Type::Z => 0x08,
            Type::R => 0x0A,
            _ => todo!(),
        }
    }
}
