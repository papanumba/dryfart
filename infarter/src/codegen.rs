/* src/codegen.rs */

#![allow(non_camel_case_types)]

use std::fs;
use std::io::Write;
use std::collections::HashSet;
use crate::asterix::*;

/* ÞA 1 & ONLY pub fn in þis mod*/

// transfarts only 1 assignement,
// which is printed directly.
pub fn transfart(b: &Block, of: &str)
{
    if b.len() != 1 {
        panic!("transfart program must be len 1, by now");
    }
    let s = &b[0];
    let e = match s {
        Stmt::Assign(_, ex) => ex,
        _ => panic!("3"),
    };
    let mut g = CodeGen::new();
    g.transfart(e);

    // write to bin file
    let mut ofile = match fs::File::create(of) {
        Ok(f) => f,
        Err(_) => panic!("could create file"),
    };
    dbg!(ofile.write_all(&g.id_to_bytes())); // dummy idents len
    dbg!(ofile.write_all(&g.cp_to_bytes()));
    dbg!(ofile.write_all(&g.bc));
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OpCode
{
    OP_CTN = 0x00, /* load constant (1 byte) */
    OP_CTL = 0x01, /* load constant long (2 byte index) */
    OP_LVV = 0x02,
    OP_LBT = 0x03,
    OP_LBF = 0x04,
    OP_LN0 = 0x05,
    OP_LN1 = 0x06,
    OP_LM1 = 0x07, /* -Z%1 */
    OP_LZ0 = 0x08,
    OP_LZ1 = 0x09,
    OP_LR0 = 0x0C,
    OP_LR1 = 0x0D,

    OP_NEG = 0x10, /* unary int negate */
    OP_ADD = 0x11,
    OP_SUB = 0x12,
    OP_MUL = 0x13,
    OP_DIV = 0x14,
    OP_INV = 0x15,

    OP_CEQ = 0x18,
    OP_CNE = 0x19,
    OP_CLT = 0x1A,
    OP_CLE = 0x1B,
    OP_CGT = 0x1C,
    OP_CGE = 0x1D,

    OP_NOT = 0x20,
    OP_AND = 0x21,
    OP_IOR = 0x22,

    OP_RET = 0xF0 /* return from current function */
    /* TODO: add opcodes */
}

struct CodeGen
{
    id: HashSet<String>,
    cp_len: u16,
    pub cp: Vec<Val>, // constant pool
    pub bc: Vec<u8>, // bytecode acumulator
}

impl CodeGen
{
    pub fn new() -> Self
    {
        return Self {
            id: HashSet::new(),
            cp_len: 0,
            cp: vec![],
            bc: vec![]
        };
    }

    pub fn transfart(&mut self, e: &Expr)
    {
        for _ in 0..4 { // dummy u32 value for bc_len
            self.bc.push(0);
        }
        self.gen_expr(e);
        self.bc.push(OpCode::OP_RET as u8);
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

    pub fn id_to_bytes(&self) -> Vec<u8>
    {
        let mut res = Vec::<u8>::new();
        // push len
        let id_len = u16::try_from(self.id.len())
            .expect("too many idents");
        res.extend_from_slice(&id_len.to_be_bytes());
        // push every Val
        for i in self.id.iter() {
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

    fn gen_expr(&mut self, e: &Expr)
    {
        match e {
            Expr::Const(v)       => self.gen_const(v),
            Expr::Ident(i)       => self.gen_ident(i),
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
                0 => {self.bc.push(OpCode::OP_LN0 as u8); return;},
                1 => {self.bc.push(OpCode::OP_LN1 as u8); return;},
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
            self.bc.push(OpCode::OP_CTN as u8);
            self.bc.push(idx as u8);
        } else {
            self.bc.push(OpCode::OP_CTL as u8);
            self.bc.extend_from_slice(&idx.to_be_bytes());
        }
    }

    fn gen_ident(&mut self, i: &str)
    {
        self.id.insert(i.to_owned());
        self.gen_const(&Val::N(0));
    }

    fn gen_uniop(&mut self, e: &Expr, o: &UniOpcode)
    {
        self.gen_expr(e);
        self.bc.push(match o {
            UniOpcode::Neg => OpCode::OP_NEG,
            UniOpcode::Not => OpCode::OP_NOT,
            UniOpcode::Inv => OpCode::OP_INV,
        } as u8);
    }

    fn gen_binop(&mut self, l: &Expr, o: &BinOpcode, r: &Expr)
    {
        self.gen_expr(l);
        self.gen_expr(r);
        self.bc.push(match o {
            BinOpcode::Add => OpCode::OP_ADD,
            BinOpcode::Sub => OpCode::OP_SUB,
            BinOpcode::Mul => OpCode::OP_MUL,
            BinOpcode::Div => OpCode::OP_DIV,
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
