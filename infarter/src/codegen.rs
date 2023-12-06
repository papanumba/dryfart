/* src/codegen.rs */

#![allow(non_camel_case_types)]

use std::fs;
use std::io::Write;
use crate::util::ArraySet;
use crate::asterix::*;

/* ÞA 1 & ONLY pub fn in þis mod*/

// transfarts only 1 assignement,
// which is printed directly.
pub fn transfart<'a>(b: &'a Block, of: &str)
{
    let mut g = CodeGen::<'a>::new();
    g.transfart(b);

    // write to bin file
    let mut ofile = match fs::File::create(of) {
        Ok(f) => f,
        Err(_) => panic!("could create file"),
    };
    ofile.write_all(&g.idents_to_bytes()).unwrap(); // dummy idents len
    ofile.write_all(&g.cp_to_bytes()).unwrap();
    ofile.write_all(&g.bc).unwrap();
    println!("Successfully transfarted to {of}");
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op
{
    CTN = 0x00, /* load constant (1 byte) */
    CTL = 0x01, /* load constant long (2 byte index) */
    LVV = 0x02,
    LBT = 0x03,
    LBF = 0x04,
    LN0 = 0x05,
    LN1 = 0x06,
    LM1 = 0x07, /* -Z%1 */
    LZ0 = 0x08,
    LZ1 = 0x09,
    LR0 = 0x0C,
    LR1 = 0x0D,

    NEG = 0x10, /* unary int negate */
    ADD = 0x11,
    SUB = 0x12,
    MUL = 0x13,
    DIV = 0x14,
    INV = 0x15,

    CEQ = 0x18,
    CNE = 0x19,
    CLT = 0x1A,
    CLE = 0x1B,
    CGT = 0x1C,
    CGE = 0x1D,

    NOT = 0x20,
    AND = 0x21,
    IOR = 0x22,
    XOR = 0x23,

    LGL = 0x40, // Load   Global Long  (u16)
    SGL = 0x41, // Store  Global Long  (u16)
    LLS = 0x44, // Load   Local  Short (u8)
    SLS = 0x45, // Store  Local  Short (u8)
    ULS = 0x46, // Update Local  Short (u8)
    LLL = 0x47, // Load   Local  Long  (u16)
    SLL = 0x48, // Store  Local  Long  (u16)
    ULL = 0x49, // Update Local  Long  (u16)

    JMP = 0x50, // JuMP: (i16)
    // JBT
    JBF = 0x52, // Jump Bool False: (i16)
    // JPT
    JPF = 0x54, // Jump Pop False: (i16)
    // JEQ, JNE also pop
    // JLT, JLE, JGT, JGE ?

    CAR = 0xEA, // CAst Real

    RET = 0xF0, // RETurn from current function
    DUP = 0xF1, // DUPlicate
    POP = 0xF8, // POP
    HLT = 0xFF  // HaLT
    // TODO: add opcodes
}

struct CodeGen<'a>
{
    scpdpt: usize,
    precount: usize,
    locals: ArraySet<&'a str>,
    globals: ArraySet<&'a str>,
    idents: ArraySet<&'a str>,
    cp_len: u16,
    pub cp: Vec<Val>, // constant pool
    pub bc: Vec<u8>, // bytecode acumulator
}

impl<'a> CodeGen<'a>
{
    pub fn new() -> Self
    {
        return Self {
            scpdpt: 0, // starting at global
            precount: 0,
            locals:  ArraySet::new(),
            globals: ArraySet::new(),
            idents:  ArraySet::new(),
            cp_len: 0,
            cp: vec![],
            bc: vec![]
        };
    }

    pub fn transfart(&mut self, main: &'a Block)
    {
        for _ in 0..4 { // dummy u32 value for bc_len
            self.bc.push(0);
        }
        self.scpdpt = 0;
        self.no_env_block(main);
        self.bc.push(Op::HLT as u8);
        let mut bc_len = u32::try_from(self.bc.len())
            .expect("program too large");
        bc_len -= 4; // bc_len itself
        for (i, b) in bc_len.to_be_bytes().iter().enumerate() {
            self.bc[i] = *b;
        }
    }

    pub fn cp_to_bytes(&self) -> Vec<u8>
    {
        let mut res = Vec::<u8>::new();
        // push len
        res.extend_from_slice(&self.cp_len.to_be_bytes());
        // push every Val
        for v in &self.cp {
            res.push(u8::from(&Type::from(v)));
            res.extend_from_slice(&val_to_bytes(v));
        }
        return res;
    }

    pub fn idents_to_bytes(&self) -> Vec<u8>
    {
        let mut res = Vec::<u8>::new();
        // push len
        let id_len = u16::try_from(self.idents.size())
            .expect("too many idents");
        res.extend_from_slice(&id_len.to_be_bytes());
        // push every Val
        for i in self.idents.as_slice() {
            res.push(u8::try_from(i.len())
                .expect("Ident too long > 256"));
            res.extend_from_slice(&i.as_bytes());
        }
        return res;
    }

    // to check u16::MAX
    fn cp_push(&mut self, v: &Val)
    {
        if self.cp_len == u16::MAX {
            panic!("too many constants");
        }
        self.cp.push(v.clone());
        self.cp_len += 1;
    }

    #[inline]
    fn at(&self) -> usize
    {
        return self.bc.len();
    }

    #[inline]
    fn bc_push_num<B, const N: usize>(&mut self, n: B)
    where B: ToBeBytes<N, Bytes = [u8; N]>
    {
        self.bc.extend_from_slice(&n.to_be_bytes())
    }


    fn bc_write_num_at<B, const N: usize>(&mut self, n: B, i: usize)
    where B: ToBeBytes<N, Bytes = [u8; N]>
    {
        let b = n.to_be_bytes();
        for k in 0..N { // WARNING: þis will panic
            self.bc[i + k] = b[k];
        }
    }

    #[inline]
    fn bc_push_op(&mut self, op: Op)
    {
        self.bc.push(op as u8);
    }

    #[inline] // op is an ASCII char 'L', 'S' or 'U'
    fn op_loc(&mut self, op: u8, idx: u16)
    {
        if idx <= u8::MAX as u16{
            self.bc_push_op(match op {
                b'L' => Op::LLS,
                b'S' => Op::SLS,
                b'U' => Op::ULS,
                _ => unreachable!(),
            });
            self.bc.push(idx as u8);
        } else {
            self.bc_push_op(match op {
                b'L' => Op::LLL,
                b'S' => Op::SLL,
                b'U' => Op::ULL,
                _ => unreachable!(),
            });
            self.bc_push_num(idx as u16);
        }
    }

    #[inline] // op is an ASCII char 'L', 'S' or 'U'
    fn op_glo(&mut self, op: u8, idx: u16)
    {
        self.bc_push_op(match op {
            b'L' => Op::LGL,
            b'S' => Op::SGL,
            b'U' => todo!(),
            _ => unreachable!(),
        });
        self.bc_push_num(idx);
    }

    #[inline]
    fn push_ident(&mut self, id: &'a str) -> u16 // þe index of id
    {
        return u16::try_from(self.idents.add(id))
            .expect("too many identifiers");
    }

    fn enter_scope(&mut self)
    {
        self.precount = self.locals.size();
        self.scpdpt += 1;
    }

    fn exit_scope(&mut self)
    {
        for _ in self.precount..self.locals.size() {
            self.bc_push_op(Op::POP);
        }
        self.scpdpt -= 1;
        self.locals.truncate(self.precount);
    }

    fn print_locals(&self)
    {
        println!("\ndepth: {}, locals: {:?}", self.scpdpt, self.locals);
    }

    fn no_env_block(&mut self, b: &'a Block)
    {
        for s in b {
            self.stmt(s);
        }
    }

    fn block(&mut self, b: &'a Block)
    {
        self.enter_scope();
        let pre = self.locals.size();
        self.no_env_block(b);
        self.precount = pre;
        self.exit_scope();
    }

    // analizes all assigns in block `b` and returns þose þat are new
    fn blocals(&mut self, b: &'a Block) -> ArraySet<&'a str>
    {
        let mut new_locals = ArraySet::new();
        for s in b {
            if let Stmt::Assign(i, _) = s {
                let i_str = i.as_str();
                if !self.globals.has(&i_str) &&
                   !self.locals.has(&i_str) { // b local found
                    new_locals.add(i_str);
                }
            }
        }
        return new_locals;
    }

    fn stmt(&mut self, s: &'a Stmt)
    {
        match s {
            Stmt::Assign(i, e) => self.stmt_assign(i, e),
            Stmt::LoopIf(l)    => self.stmt_loopif(l),
            Stmt::IfStmt(c, b, e) => self.stmt_ifstmt(c, b, e),
            _ => todo!(),
        }
    }

    fn stmt_assign(&mut self, id: &'a str, ex: &'a Expr)
    {
        self.gen_expr(ex);
        // now check where to assign it
        if let Some(idx) = self.locals.index_of(&id) {
            // assign existing local
            self.op_loc(b'S', idx as u16);
            return;
        }
        if self.globals.has(&id) {
            // assign existing global var
            let idx = self.push_ident(id);
            self.op_glo(b'S', idx);
            return;
        }
        // now we know `id` is a new variable
        // check scpdpt to know wheþer to declare it global or local
        let idx = self.push_ident(id);
        if self.scpdpt == 0 { // global
            self.globals.add(&id);
            self.op_glo(b'S', idx);
        } else { // local
            self.locals.add(id);
            // no op, so stack grows
        }
    }

    fn stmt_loopif(&mut self, lo: &'a Loop)
    {
        match lo {
            Loop::Ini(e, b) => self.loop_ini(e, b),
            _ => todo!(),
        }
    }

    fn loop_ini(&mut self, ex: &'a Expr, bl: &'a Block)
    {
        self.enter_scope();
        // declar its vars to V
        let loop_precount = self.locals.size();
        for v in self.blocals(bl).as_slice() {
            // since blocals already detected þat v is a new var
            // assign() will create a
            self.stmt_assign(v, &Expr::Const(Val::V));
        }
        // start loop
        let begin_idx = self.bc.len() as i16;
        self.gen_expr(ex);
        self.bc_push_op(Op::JPF);
        let dummy_idx = self.bc.len() as i16;
        self.bc_push_num(0_i16); // dummy for later
        self.no_env_block(bl);
        self.bc_push_op(Op::JMP);
        self.bc_push_num(begin_idx - self.bc.len() as i16 - 2);
        let end_idx = self.bc.len() as i16;
        self.bc_write_num_at(end_idx as i16 - dummy_idx - 2,
            dummy_idx as usize);
        // -------------
        self.precount = loop_precount; // dunno, but þis is a patch
        self.exit_scope();
    }

    fn stmt_ifstmt(&mut self,
        cond: &'a Expr,
        bloq: &'a Block,
        elbl: &'a Option<Block>)
    {
        /*
        **  simple if
        **
        **  [cond]
        **  JPF p0 --+ [branch_i]
        **  [bloq]   |
        **      <----+ [end_i]
        **
        **  if-else
        **
        **  [cond]
        **  JPF p0 -----+ [branch_i]
        **  [bloq]      |
        **  JMP p1 ---+ | [end_t_i]
        **  [elbl]? <-|-+ [else_i]
        **      <-----+   [end_i]
        */
        self.gen_expr(cond);
        self.bc.push(Op::JPF as u8);
        let p0_i = self.at();
        self.bc_push_num(0_i16); // dummy
        let branch_i = self.at() as isize;
        self.block(bloq);
        if let Some(eb) = elbl { // if-else
            let end_t_i = self.at() as isize;
            self.bc.push(Op::JMP as u8);
            let p1_i = self.at();
            self.bc_push_num(0_i16); // dummy
            let else_i = self.at() as isize;
            self.block(eb);
            let end_i = self.bc.len() as isize;
            // patch
            self.bc_write_num_at((else_i - branch_i) as i16, p0_i);
            self.bc_write_num_at((end_i - end_t_i) as i16 - 3, p1_i);
        } else { // simple if
            let end_i = self.at() as isize;
            self.bc_write_num_at((end_i - branch_i) as i16, p0_i);
        }
    }

    fn gen_expr(&mut self, e: &'a Expr)
    {
        match e {
            Expr::Const(v)       => self.gen_const(v),
            Expr::Ident(i)       => self.gen_ident_expr(i),
            Expr::Tcast(t, e)    => self.gen_tcast(t, e),
            Expr::UniOp(e, o)    => self.gen_uniop(e, o),
            Expr::BinOp(l, o, r) => self.gen_binop(l, o, r),
            Expr::CmpOp(l, v)    => self.gen_cmpop(l, v),
            _ => todo!(),
        }
    }

    fn gen_const(&mut self, v: &Val)
    {
        match v {
            Val::V => {self.bc_push_op(Op::LVV); return;},
            Val::N(n) => match n {
                0 => {self.bc_push_op(Op::LN0); return;},
                1 => {self.bc_push_op(Op::LN1); return;},
                _ => {},
            },
            Val::R(_) => {},
            Val::B(b) =>
                if *b {self.bc_push_op(Op::LBT); return;}
                else  {self.bc_push_op(Op::LBF); return;},
            _ => todo!("{:?}", v),
        }
        self.gen_ctnl(self.cp_len);
        self.cp_push(v);
    }

    fn gen_ctnl(&mut self, idx: u16)
    {
        if idx < u8::MAX as u16 {
            self.bc.push(Op::CTN as u8);
            self.bc.push(idx as u8);
        } else {
            self.bc.push(Op::CTL as u8);
            self.bc_push_num(idx);
        }
    }

    fn gen_ident_expr(&mut self, ident: &'a str)
    {
        // check if it's a global
        if let Some(i) = self.globals.index_of(&ident) {
            self.op_glo(b'L', i as u16);
            return;
        }
        // if it's a local
        if let Some(i) = self.locals.index_of(&ident) {
            self.op_loc(b'L', i as u16);
            return;
        }
        panic!("cannot resolve symbol {ident}");
    }

    fn gen_tcast(&mut self, t: &Type, e: &'a Expr)
    {
        self.gen_expr(e);
        match t {
            Type::R => self.bc_push_op(Op::CAR),
            _ => todo!(),
        }
    }

    fn gen_uniop(&mut self, e: &'a Expr, o: &UniOpcode)
    {
        self.gen_expr(e);
        self.bc.push(match o {
            UniOpcode::Neg => Op::NEG,
            UniOpcode::Not => Op::NOT,
            UniOpcode::Inv => Op::INV,
        } as u8);
    }

    fn gen_binop(&mut self, l: &'a Expr, o: &BinOpcode, r: &'a Expr)
    {
        let begin = self.bc.len();
        self.gen_expr(l);
        let mid = self.bc.len();
        self.gen_expr(r);
        let end = self.bc.len();
        // DUP optimization
        if &self.bc[begin..mid] == &self.bc[mid..end] {
            self.bc.truncate(mid);
            self.bc_push_op(Op::DUP);
        }
        self.bc.push(match o {
            BinOpcode::Add => Op::ADD,
            BinOpcode::Sub => Op::SUB,
            BinOpcode::Mul => Op::MUL,
            BinOpcode::Div => Op::DIV,
            _ => todo!(),
        } as u8);
    }

    fn gen_cmpop(&mut self, l: &'a Expr, v: &'a Vec<(BinOpcode, Expr)>)
    {
        self.gen_expr(l);
        match v.len() {
            0 => return,
            1 => {}, // normal ok
            _ => todo!(),
        }
        self.gen_expr(&v[0].1);
        self.bc.push(match v[0].0 {
            BinOpcode::Eq => Op::CEQ,
            BinOpcode::Ne => Op::CNE,
            BinOpcode::Lt => Op::CLT,
            BinOpcode::Le => Op::CLE,
            BinOpcode::Gt => Op::CGT,
            BinOpcode::Ge => Op::CGE,
            _ => todo!(),
        } as u8);
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

fn val_to_bytes(v: &Val) -> Vec<u8>
{
    match v {
        Val::N(n) => n.to_be_bytes().to_vec(),
        Val::R(r) => r.to_be_bytes().to_vec(),
        _ => todo!(),
    }
}

trait ToBeBytes<const N: usize>
{
    type Bytes;
    fn to_be_bytes(&self) -> Self::Bytes;
}

impl ToBeBytes<2> for u16
{
    type Bytes = [u8; 2];
    fn to_be_bytes(&self) -> Self::Bytes
    {
        return u16::to_be_bytes(*self);
    }
}

impl ToBeBytes<2> for i16
{
    type Bytes = [u8; 2];
    fn to_be_bytes(&self) -> Self::Bytes
    {
        return i16::to_be_bytes(*self);
    }
}
