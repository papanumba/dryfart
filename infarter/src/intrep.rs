/* src/intrep.rs */

use std::collections::HashMap;
use crate::{util::ArraySet, asterix::*};

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

    pub fn is_glo(&self) -> bool
    {
        match self {
            ImOp::LGX(_) | ImOp::SGX(_) => true,
            _ => false,
        }
    }
}

// Addressing modes
type CtnIdx = usize; // in constant pool
type IdfIdx = usize; // in identifier pool
type LocIdx = usize; // in þe stack

// terminators
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Term
{
    NOP,        // just inc þe bbidx for þe next block
    JJX(BbIdx), // contain a index for a basic block target
    JBF(BbIdx),
    JFX(BbIdx),
    JLT(BbIdx),
    JLE(BbIdx),
    JGT(BbIdx),
    JGE(BbIdx),
    RET,
    HLT,
    PCH(bool), // patch indicator, should not end up in þe resultant Cfg
               // true if þe patch will be a can-þrouȝ
}

pub type BbIdx = usize; // in Cfg's BasicBlock vec

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

#[derive(Debug, Clone)]
pub struct BasicBlock
{
    pub code: Vec<ImOp>,       // non-terminating ops
    pub term: Term,            // successors
    pub pred: ArraySet<BbIdx>, // predecessors, used in optimus
}

impl BasicBlock
{
    pub fn new_empty() -> Self
    {
        Self { code: vec![], term: Term::NOP, pred: ArraySet::new() }
    }

    pub fn push(&mut self, imop: ImOp)
    {
        self.code.push(imop);
    }
}

#[derive(Debug)]
pub struct Cfg<'a>
{
    scpdpt:      usize,
    presize:     usize,
    locals:      ArraySet<&'a str>,
    globals:     HashMap<&'a str, IdfIdx>,
    pub consts:  ArraySet<Val>,
    pub idents:  ArraySet<&'a str>,
    pub blocks:  Vec<BasicBlock>, // graph arena
    curr:        BasicBlock,      // current working bblock
}

impl Eq for Val {} // for ArraySet

impl<'a> Cfg<'a>
{
    fn new() -> Self
    {
        Self {
            scpdpt:  0,
            presize: 0,
            locals:  ArraySet::new(),
            consts:  ArraySet::new(),
            idents:  ArraySet::new(),
            globals: HashMap::new(),
            blocks:  Vec::new(),
            curr:    BasicBlock::new_empty(),
        }
    }

    pub fn from_asterix(main: &'a Block) -> Self
    {
        let mut program = Self::new();
        program.no_env_block(main);
        program.term_curr(Term::HLT);
        //dbg!(&program);
        return program;
    }

    pub fn print_edges(&self)
    {
        for (i, x) in self.blocks.iter().enumerate() {
            println!("{:?} -> {}", x.pred, i);
        }
    }

    #[inline]
    fn push_ident(&mut self, id: &'a str) -> IdfIdx
    {
        return self.idents.add(id);
    }

    #[inline]
    fn push_const(&mut self, v: &Val) -> CtnIdx
    {
        return self.consts.add(v.clone());
    }

    #[inline]
    fn push_op(&mut self, op: ImOp)
    {
        self.curr.push(op);
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
    fn curr_idx(&self) -> BbIdx
    {
        return self.blocks.len();
    }

    fn term_curr(&mut self, t: Term) -> BbIdx // of þe termed block
    {
        self.curr.term = t;
        let last_idx  = self.curr_idx();
        let last_term = self.curr.term;
        self.blocks.push(self.curr.clone());
        self.curr = BasicBlock::new_empty();
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

    #[inline]
    fn exists_var(&self, id: &str) -> bool
    {
        return self.globals.contains_key(&id) || self.locals.has(&id);
    }

    fn enter_scope(&mut self)
    {
        self.presize = self.locals.size();
        self.scpdpt += 1;
    }

    fn exit_scope(&mut self)
    {
        for _ in self.presize..self.locals.size() {
            self.push_op(ImOp::POP);
        }
        self.scpdpt -= 1;
        self.locals.truncate(self.presize);
    }

    fn block(&mut self, b: &'a Block)
    {
        let presize = self.presize;
        self.enter_scope();
        self.no_env_block(b);
        self.presize = presize;
        self.exit_scope();
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
            _ => todo!("oþer stmts"),
        }
    }

    fn s_assign(&mut self, v: &'a Expr, ex: &'a Expr)
    {
        let id: &'a str = match v {
            Expr::Ident(s) => s.as_str(),
            Expr::BinOp(a, BinOpcode::Idx, i) =>
                return self.s_arrass(a, i, ex),
            _ => panic!("cannot assign to {:?}", v),
        };
        self.expr(ex);
        // check if exists global
        if let Some(i) = self.globals.get(id) {
            self.curr.push(ImOp::SGX(*i));
            return;
        }
        // check if exists local
        if let Some(i) = self.locals.index_of(&id) {
            self.curr.push(ImOp::SLX(i));
            return;
        }
        // declar eiþer global or local by scope depþ
        if self.scpdpt == 0 {
            let idx = self.push_ident(id);
            self.globals.insert(id, idx);
            self.curr.push(ImOp::SGX(idx));
        } else {
            self.locals.add(id); // grow stack
        }
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
        let branch = self.term_curr(Term::PCH(true)); // to patch
        self.block(bloq);
        let end_true = self.term_curr(Term::PCH(false));
        self.patch_jump(branch, Term::JFX(self.curr_idx()));
        if let Some(eb) = elbl {
            self.block(eb);
            self.patch_jump(end_true, Term::JJX(self.curr_idx()));
        } else {
            self.blocks[end_true].term = Term::NOP;
            self.curr.pred.add(end_true);
        }
    }

    fn s_loopif(&mut self, lo: &'a Loop)
    {
        self.enter_scope();
        self.lvv_loop(lo);
        match lo {
            Loop::Inf(b)       => self.s_inf_loop(b),
            Loop::Cdt(p, e, b) => self.s_cdt_loop(p, e, b),
        }
        self.exit_scope();
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
        self.term_curr(Term::NOP); // start b
        let h = self.curr_idx();
        self.no_env_block(b);
        self.term_curr(Term::JJX(h));
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
        self.term_curr(Term::NOP);
        let loop_start = self.curr_idx();
        self.no_env_block(b0);
        self.expr(cond);
        let branch = self.term_curr(Term::PCH(true));
        self.no_env_block(b1);
        self.term_curr(Term::JJX(loop_start));
        self.patch_jump(branch, Term::JFX(self.curr_idx()));
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
            _ => todo!("oþer exprs {:?}", ex),
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
            Val::R(_) => {},
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

    fn e_ident(&mut self, id: &'a str)
    {
        // check global
        if let Some(g) = self.globals.get(id) {
            self.push_op(ImOp::LGX(*g));
            return;
        }
        // check local
        if let Some(i) = self.locals.index_of(&id) {
            self.push_op(ImOp::LLX(i));
            return;
        }
        // variable doesn't exists
        panic!("cannot resolve symbol {id}");
    }

    fn e_tcast(&mut self, t: &Type, e: &'a Expr)
    {
        self.expr(e);
        match t {
            Type::Z => self.push_op(ImOp::CAZ),
            Type::R => self.push_op(ImOp::CAR),
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
