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

    GGL = 0x40, // Get GLobal: operand u16
    SGL = 0x41, // Set GLobal: operand u16

    RET = 0xF0, /* return from current function */
    HLT = 0xFF /* halt */
    /* TODO: add opcodes */
}

struct CodeGen<'a>
{
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
            idents: ArraySet::new(),
            cp_len: 0,
            cp: vec![],
            bc: vec![]
        };
    }

    pub fn transfart(&mut self, b: &'a Block)
    {
        for _ in 0..4 { // dummy u32 value for bc_len
            self.bc.push(0);
        }
        for s in b {
            self.stmt(s);
        }
        self.bc.push(Op::HLT as u8);
        let mut bc_len = u32::try_from(self.bc.len())
            .expect("program too large");
        bc_len -= 4; // bc_len itself
        for (i, b) in bc_len.to_be_bytes().iter().enumerate() {
            self.bc[i] = *b;
        }
        println!("{}", self.bc.len());
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
        println!("{}", res.len());
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
        println!("{}", res.len());
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
    fn bc_push_u16(&mut self, u: u16)
    {
        self.bc.extend_from_slice(&u.to_be_bytes());
    }

    #[inline]
    fn bc_push_op(&mut self, op: Op)
    {
        self.bc.push(op as u8);
    }

    #[inline]
    fn push_ident(&mut self, id: &'a str) -> u16 // þe index of id
    {
        return u16::try_from(self.idents.add(id))
            .expect("too many identifiers");
    }

    fn stmt(&mut self, s: &'a Stmt)
    {
        match s {
            Stmt::Assign(i, e) => self.stmt_assign(i, e),
            _ => todo!(),
        }
    }

    fn stmt_assign(&mut self, id: &'a str, ex: &'a Expr)
    {
        self.gen_expr(ex);
        let idx = self.push_ident(id);
        self.bc.push(Op::SGL as u8); // set first ident to N(0)
        self.bc_push_u16(idx);
    }

    fn gen_expr(&mut self, e: &'a Expr)
    {
        match e {
            Expr::Const(v)       => self.gen_const(v),
            Expr::Ident(i)       => self.gen_ident_expr(i),
            Expr::UniOp(e, o)    => self.gen_uniop(e, o),
            Expr::BinOp(l, o, r) => self.gen_binop(l, o, r),
            Expr::CmpOp(l, _)    => self.gen_expr(l),
            _ => todo!(),
        }
    }

    fn gen_const(&mut self, v: &Val)
    {
        match v {
            Val::N(n) => match n {
                0 => {self.bc_push_op(Op::LN0); return;},
                1 => {self.bc_push_op(Op::LN1); return;},
                _ => {},
            },
            Val::R(_) => {},
            _ => todo!(),
        }
        self.gen_ctnl(self.cp_len);
        self.cp_push(v);
    }

    fn gen_ctnl(&mut self, idx: u16)
    {
        if idx < 256 {
            self.bc.push(Op::CTN as u8);
            self.bc.push(idx as u8);
        } else {
            self.bc.push(Op::CTL as u8);
            self.bc_push_u16(idx);
        }
    }

    fn gen_ident_expr(&mut self, ident: &'a str)
    {
        let idx = self.push_ident(ident);
        self.bc.push(Op::GGL as u8);
        self.bc_push_u16(idx);
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
        self.gen_expr(l);
        self.gen_expr(r);
        self.bc.push(match o {
            BinOpcode::Add => Op::ADD,
            BinOpcode::Sub => Op::SUB,
            BinOpcode::Mul => Op::MUL,
            BinOpcode::Div => Op::DIV,
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
