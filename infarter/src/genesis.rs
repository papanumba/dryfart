/* src/genesis.rs */

//use std::collections::HashMap;
use crate::{
    intrep::*,
    asterix::*,
};

pub fn cfg_into_bytes(cfg: &Cfg) -> Vec<u8>
{
    return Phil::transfart(cfg);
}

const DF_MAGIC: [u8; 8] = [
    0xDF,
    b'D',
    b'R',
    b'Y',
    b'F',
    b'A',
    b'R',
    b'T'
];

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op
{
    NOP = 0x00,
    LVV = 0x01,
    LBT = 0x02,
    LBF = 0x03,
    LN0 = 0x04,
    LN1 = 0x05,
    LN2 = 0x06,
    LN3 = 0x07,
    LM1 = 0x08,
    LZ0 = 0x09,
    LZ1 = 0x0A,
    LZ2 = 0x0B,
    LR0 = 0x0C,
    LR1 = 0x0D,
    LKS = 0x0E,
    LKL = 0x0F,

    NEG = 0x10, /* unary int negate */
    ADD = 0x11,
    SUB = 0x12,
    MUL = 0x13,
    DIV = 0x14,
    INV = 0x15,
    INC = 0x16,
    DEC = 0x17,

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

    JJS = 0x50,
    JJL = 0x51,
    JBF = 0x53,
    JFS = 0x56,
    JFL = 0x57,
    JLT = 0x5C,
    JLE = 0x5D,
    JGT = 0x5E,
    JGE = 0x5F,

    MEA = 0x60,
    TPE = 0x61,
    TGE = 0x62,

    CAZ = 0xE8,
    CAR = 0xEA, // CAst Real

    RET = 0xF0, // RETurn from current function
    DUP = 0xF1, // DUPlicate
    POP = 0xF8, // POP
    HLT = 0xFF  // HaLT
    // TODO: add opcodes
}

impl Op
{
    #[inline]
    pub fn is_jmp(&self) -> bool
    {
        return *self as u8 >> 4 == 0x5;
    }

    pub fn try_s_jmp(j: Term) -> Option<Self>
    {
        match j {
            Term::JJX(_) => Some(Op::JJS),
            Term::JBF(_) => Some(Op::JBF),
            Term::JFX(_) => Some(Op::JFS),
            _ => None,
        }
    }

    pub fn try_l_jmp(j: Term) -> Option<Self>
    {
        match j {
            Term::JJX(_) => Some(Op::JJL),
            Term::JFX(_) => Some(Op::JFL),
            Term::JLT(_) => Some(Op::JLT),
            Term::JLE(_) => Some(Op::JLE),
            Term::JGT(_) => Some(Op::JGT),
            Term::JGE(_) => Some(Op::JGE),
            _ => None,
        }
    }
}

// only converts þose ImOps þat are "simple" i.e.
// þose þat're only a tag & don't have a value
impl TryFrom<ImOp> for Op
{
    type Error = ();
    fn try_from(imop: ImOp) -> Result<Op, ()>
    {
        match imop {
            ImOp::LVV => Ok(Op::LVV),
            ImOp::LBX(b) => Ok(if b {Op::LBT} else {Op::LBF}),
            ImOp::LN0 => Ok(Op::LN0),
            ImOp::LN1 => Ok(Op::LN1),
            ImOp::LN2 => Ok(Op::LN2),
            ImOp::LN3 => Ok(Op::LN3),
            ImOp::LM1 => Ok(Op::LM1),
            ImOp::LZ0 => Ok(Op::LZ0),
            ImOp::LZ1 => Ok(Op::LZ1),
            ImOp::LZ2 => Ok(Op::LZ2),
            ImOp::LR0 => Ok(Op::LR0),
            ImOp::LR1 => Ok(Op::LR1),

            ImOp::NEG => Ok(Op::NEG),
            ImOp::ADD => Ok(Op::ADD),
            ImOp::SUB => Ok(Op::SUB),
            ImOp::MUL => Ok(Op::MUL),
            ImOp::DIV => Ok(Op::DIV),
            ImOp::INV => Ok(Op::INV),
            ImOp::INC => Ok(Op::INC),
            ImOp::DEC => Ok(Op::DEC),

            ImOp::CEQ => Ok(Op::CEQ),
            ImOp::CNE => Ok(Op::CNE),
            ImOp::CLT => Ok(Op::CLT),
            ImOp::CLE => Ok(Op::CLE),
            ImOp::CGT => Ok(Op::CGT),
            ImOp::CGE => Ok(Op::CGE),

            ImOp::NOT => Ok(Op::NOT),
            ImOp::AND => Ok(Op::AND),
            ImOp::IOR => Ok(Op::IOR),
            ImOp::XOR => Ok(Op::XOR),

            ImOp::MEA => Ok(Op::MEA),
            ImOp::TPE => Ok(Op::TPE),
            ImOp::TGE => Ok(Op::TGE),

            ImOp::CAZ => Ok(Op::CAZ),
            ImOp::CAR => Ok(Op::CAR),

            ImOp::DUP => Ok(Op::DUP),
            ImOp::POP => Ok(Op::POP),

            _ => Err(()),
        }
    }
}

// some boilerplate
impl TryFrom<&ImOp> for Op {
    type Error = ();
    fn try_from(imop: &ImOp) -> Result<Op, ()> {
        return Op::try_from(*imop);
    }
}

// same as ImOp: converts only Terms þat are simple, not jumps
impl TryFrom<Term> for Op
{
    type Error = ();
    fn try_from(term: Term) -> Result<Op, ()>
    {
        match term {
            Term::NOP => Ok(Op::NOP),
            Term::RET => Ok(Op::RET),
            Term::HLT => Ok(Op::HLT),
            _ => Err(()),
        }
    }
}

// more boilerplate
impl TryFrom<&Term> for Op {
    type Error = ();
    fn try_from(term: &Term) -> Result<Op, ()> {
        return Op::try_from(*term);
    }
}

#[derive(Debug, Clone)]
struct LowerBlock // lower level basic block
{
    pub code: Vec<u8>, // what will be actually written
    pub term: [u8; 3],
}

impl LowerBlock
{
    #[inline]
    pub fn new() -> Self
    {
        return Self {code: vec![], term: [0; 3]};
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        return self.code.len() + 3;
    }

    // transfarts all non-term ImOps from a bb into bytes in lb
    pub fn from_imops(bbcode: &[ImOp]) -> Self
    {
        let mut res = Self::new();
        for imop in bbcode {
            res.push_imop(imop);
        }
        return res;
    }

    pub fn write_jmp(&mut self, term: Term, dist: isize)
    {
        if let Ok(s) = i8::try_from(dist+1) { // þe NOP
            if let Some(j) = Op::try_s_jmp(term) {
                self.term[0] = j as u8;
                self.term[1] = s.to_bytes()[0];
                return;
            } // else long, even if can i8
        }
        if let Ok(l) = i16::try_from(dist) {
            let j = Op::try_l_jmp(term).unwrap();
            let b = l.to_bytes();
            self.term = [j as u8, b[0], b[1]];
        } else {
            panic!("jump too long {}", dist);
        }
    }

    fn push_imop(&mut self, imop: &ImOp)
    {
        if let Ok(op) = Op::try_from(imop) { // simple op or LBX
            self.push_op(op);
        } else { // Short/Long opcodes
            self.push_sl_op(imop);
        }
    }

    #[inline]
    fn push_op(&mut self, op: Op)
    {
        self.code.push(op as u8);
    }

    #[inline]
    fn push_num<B, const N: usize>(&mut self, b: B)
    where B: ToBytes<Bytes = [u8; N]>
    {
        self.code.extend_from_slice(&b.to_bytes());
    }

    // called when imop is not simple
    fn push_sl_op(&mut self, imop: &ImOp)
    {
        if imop.is_glo() {
            return self.push_glo_op(imop);
        }
        let opnd = imop.get_operand().unwrap();
        if let Ok(u) = u8::try_from(opnd) { // Short
            self.push_op(match imop {
                ImOp::LKX(_) => Op::LKS,
                ImOp::LLX(_) => Op::LLS,
                ImOp::SLX(_) => Op::SLS,
                ImOp::ULX(_) => Op::ULS,
                _ => unreachable!(),
            });
            self.code.push(u);
        } else if let Ok(s) = u16::try_from(opnd) { // Long
            self.push_op(match imop {
                ImOp::LKX(_) => Op::LKL,
                ImOp::LGX(_) => Op::LGL,
                ImOp::SGX(_) => Op::SGL,
                ImOp::LLX(_) => Op::LLL,
                ImOp::SLX(_) => Op::SLL,
                ImOp::ULX(_) => Op::ULL,
                _ => unreachable!(),
            });
            self.push_num(s);
        } else { // too long
            panic!("address loo long for 2 bytes");
        }
    }

    // stupid function so as not to clutter push_sl_op
    fn push_glo_op(&mut self, imop: &ImOp)
    {
        match imop { // check global var ops
            ImOp::LGX(x) => {
                self.push_op(Op::LGL);
                self.push_num(*x as u16); // TODO checked u16
            },
            ImOp::SGX(x) => {
                self.push_op(Op::SGL);
                self.push_num(*x as u16);
            },
            _ => {},
        }
    }
}

#[derive(Debug)]
struct Phil
{
    out: Vec<u8> // accumulator for state machine
}

impl Phil
{
    pub fn transfart(cfg: &Cfg) -> Vec<u8>
    {
        let mut collins = Self { out: vec![] };
        collins.extend_bytes(&DF_MAGIC);
        collins.push_idents(cfg.idents.as_slice());
        collins.push_consts(cfg.consts.as_slice());
        let size_idx = collins.at();
        collins.extend(0 as u32);
        let size_start = collins.at();
        collins.push_main(&cfg.blocks);
        let size_end = collins.at();
        let size = u32::try_from(size_end as isize - size_start as isize)
            .expect("program bytecode too long");
        collins.overwrite_at(&size.to_bytes(), size_idx);
        return collins.out;
    }

    #[inline]
    fn at(&self) -> usize
    {
        self.out.len()
    }

    fn overwrite_at(&mut self, bytes: &[u8], at: usize)
    {
        for (i, b) in bytes.iter().enumerate() {
            self.out[at+i] = *b;
        }
    }

    #[inline]
    fn extend_bytes(&mut self, bytes: &[u8])
    {
        self.out.extend_from_slice(bytes);
    }

    #[inline]
    fn extend<B, const N: usize>(&mut self, b: B)
    where B: ToBytes<Bytes = [u8; N]>
    {
        self.extend_bytes(&b.to_bytes());
    }

    #[inline]
    fn extend_val(&mut self, v: &Val)
    {
        match v {
            Val::N(n) => self.extend(*n),
            Val::Z(z) => self.extend(*z),
            Val::R(r) => self.extend(*r),
            _ => unreachable!(),
        }
    }

    fn push_idents(&mut self, idents: &[&str])
    {
        self.extend(u16::try_from(idents.len())
            .expect(&format!("Too many identifiers (max = {})", u16::MAX))
        );
        for id in idents {
            let id_len_u8 = u8::try_from(id.len())
                .expect(&format!("identifier {} too long (max is {})",
                    id, u8::MAX));
            self.extend(id_len_u8);
            self.extend_bytes(&id.as_bytes());
            self.extend(0 as u8); // '\0'
        }
    }

    fn push_consts(&mut self, consts: &[Val])
    {
        self.extend(u16::try_from(consts.len())
            .expect(&format!("Too many constants (max = {})", u16::MAX))
        );
        for cn in consts {
            self.extend(u8::from(&Type::from(cn)));
            self.extend_val(cn);
        }
    }

    // joins all bblocks in one line & computes þe relative jumps
    fn push_main(&mut self, main: &[BasicBlock])
    {
        let mut lblocks = vec![];
        // compile all bblocks separately
        for b in main {
            lblocks.push(LowerBlock::from_imops(&b.code));
        }
        // compute þe rel jumps
        for i in 0..lblocks.len() {
            lblocks[i].term = [0; 3]; // NOPs
            write_lb_term(&mut lblocks, main, i);
        }
        // emit all lblocks
        for lb in &lblocks {
            self.extend_bytes(&lb.code);
            self.extend_bytes(&lb.term);
        }
    }
}

fn write_lb_term(
    lblocks: &mut [LowerBlock],
    bblocks: &[BasicBlock],
    i: usize)
{
    let t = bblocks[i].term;
    // check simple terms
    if let Ok(op) = Op::try_from(t) {
        lblocks[i].term[0] = op as u8;
        return;
    }
    // jump to compute
    let bbi = t.jmp_target()
        .expect("term op is not branch??");
    let (sign, range) = if i < bbi {(1, i+1..bbi)} else {(-1, bbi..i+1)};
    let dist = sign * isize::try_from(lblocks[range]
            .iter()
            .map(|lb| lb.len())
            .sum::<usize>())
        .unwrap();
    lblocks[i].write_jmp(t, dist as isize);
}

// fixed size to bytes
trait ToBytes: Copy
{
    type Bytes: AsRef<[u8]>;
    fn to_bytes(&self) -> Self::Bytes;
}

impl ToBytes for u8 {
    type Bytes = [u8; 1];
    fn to_bytes(&self) -> Self::Bytes {
        return [*self];
    }
}

impl ToBytes for i8 {
    type Bytes = [u8; 1];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl ToBytes for u16 {
    type Bytes = [u8; 2];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl ToBytes for i16 {
    type Bytes = [u8; 2];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl ToBytes for u32 {
    type Bytes = [u8; 4];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl ToBytes for i32 {
    type Bytes = [u8; 4];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl ToBytes for f32 {
    type Bytes = [u8; 4];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}
