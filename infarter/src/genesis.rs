/* src/genesis.rs */

use std::rc::Rc;
use crate::{
    intrep::*,
    asterix::*,
    dflib,
};

pub fn comp_into_bytes(c: &Compiler) -> Vec<u8>
{
    return Phil::transfart(c);
}

const DF_MAGIC: &'static [u8; 8] = b"\xDFDRYFART";

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum Op
{
    #[default]
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

    NEG = 0x10,
    ADD = 0x11,
    SUB = 0x12,
    MUL = 0x13,
    DIV = 0x14,
    INV = 0x15,
    INC = 0x16,
    DEC = 0x17,
    MOD = 0x18,

    CEQ = 0x98,
    CNE = 0x99,
    CLT = 0x9A,
    CLE = 0x9B,
    CGT = 0x9C,
    CGE = 0x9D,

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
    JBT = 0x52,
    JBF = 0x53,
    JTS = 0x54,
    JTL = 0x55,
    JFS = 0x56,
    JFL = 0x57,
    JES = 0x58,
    JEL = 0x59,
    JNS = 0x5A,
    JNL = 0x5B,
    JLT = 0x5C,
    JLE = 0x5D,
    JGT = 0x5E,
    JGE = 0x5F,

    AMN = 0x60,
    APE = 0x61,
    AGE = 0x62,
    ASE = 0x63,

    TMN = 0x70,
    TSF = 0x71,
    TGF = 0x72,

    PMN = 0x80,
    PCL = 0x82,

    FMN = 0x88,
    FCL = 0x89,

    LUV = 0x8F,

    CAN = 0xE6,
    CAZ = 0xE8,
    CAR = 0xEA,

    RET = 0xF0,
    END = 0xF1,
    DUP = 0xF4,
    SWP = 0xF5,
    ROT = 0xF6,
    POP = 0xF8,
    HLT = 0xFF
    // TODO: add opcodes
}

macro_rules! term2jmp { // short jump
    ($fnname:ident, $($term:ident => $op:ident),+) => {
        pub fn $fnname(j: Term) -> Option<Self>
        {
            match j {
                $(Term::$term(_) => Some(Op::$op),)+
                _ => None
            }
        }
    }
}

impl Op
{
    #[inline]
    pub fn is_jmp(&self) -> bool
    {
        return *self as u8 >> 4 == 0x5;
    }

    term2jmp!{try_s_jmp,
        JJX => JJS, JBT => JBT, JBF => JBF, JTX => JTS,
        JFX => JFS, JEX => JES, JNX => JNS
    }

    term2jmp!{try_l_jmp,
        JJX => JJL, JTX => JTL, JFX => JFL, JEX => JEL, JNX => JNL,
        JLT => JLT, JLE => JLE, JGT => JGT, JGE => JGE
    }
}

// only converts þose ImOps þat are "simple" i.e.
// þose þat're only a tag & don't have a value
impl TryFrom<ImOp> for Op
{
    type Error = ();
    fn try_from(imop: ImOp) -> Result<Op, ()>
    {
        macro_rules! convert {
            ($imop:ident, $($name:ident),+;) => {
                return match $imop {
                    ImOp::LBX(b) => Ok(if b {Op::LBT} else {Op::LBF}),
                    $(ImOp::$name => Ok(Op::$name),)+
                    _ => Err(()),
                }
            }
        }
        convert!(imop, LVV,
            LN0, LN1, LN2, LN3, LM1, LZ0, LZ1, LZ2, LR0, LR1, NEG, ADD, SUB,
            MUL, DIV, INV, INC, DEC, MOD, CEQ, CNE, CLT, CLE, CGT, CGE, NOT,
            AND, IOR, XOR, AMN, APE, AGE, ASE, TMN, CAN, CAZ, CAR, DUP, SWP,
            ROT, POP;
        );
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
            Term::END => Ok(Op::END),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum LowerTerm
{
    #[default]
    Nop,      // NOP
    One,      // "simple" op
    Jjs(i8),  // short jump
    Jjl(i16), // long jump
}

impl LowerTerm
{
    // how many bytes needs to be written, including the op itself
    pub fn size(&self) -> usize
    {
        match self {
            Self::Nop    => 0,
            Self::One    => 1,
            Self::Jjs(_) => 2,
            Self::Jjl(_) => 3,
        }
    }

    pub fn jmp_dist(&self) -> Option<isize>
    {
        match self {
            Self::Nop |
            Self::One => None,
            Self::Jjs(d) => Some(*d as isize),
            Self::Jjl(d) => Some(*d as isize),
        }
    }

    // returns serialized jump arg
    pub fn to_bytes(&self) -> Option<(u8, Option<u8>)>
    {
        match self {
            Self::Nop |
            Self::One => None,
            Self::Jjs(d) => Some((d.to_bytes()[0], None)),
            Self::Jjl(d) => {
                let b = d.to_bytes();
                Some((b[0], Some(b[1])))
            }
        }
    }
}

#[derive(Debug, Default)]
struct LowerBlock // lower level basic block
{
    pub code: Vec<u8>,   // what will be actually written
    pub term: Op,        // term op
    pub tinf: LowerTerm, // stores info about t_op
}

impl LowerBlock
{
    // transfarts all non-term ImOps from a bb into bytes in lb
    pub fn from_imops(bbcode: &[ImOp]) -> Self
    {
        let mut res = Self::default();
        for imop in bbcode {
            res.push_imop(imop);
        }
        return res;
    }

    #[inline]
    pub fn size(&self) -> usize
    {
        return self.code.len() + self.tinf.size();
    }

    #[inline]
    pub fn set_simple_term(&mut self, t: Op)
    {
        self.term = t;
        self.tinf = if t == Op::NOP {
            LowerTerm::Nop
        } else {
            LowerTerm::One
        }
    }

    pub fn shrink_jj_by(&mut self, dx: u8)
    {
        match &mut self.tinf {
            LowerTerm::Jjs(ref mut d) => {
                let dx = dx as i8;
                if *d < 0 {
                    *d += dx;
                } else if *d > 0 {
                    *d -= dx;
                } else { // 0
                    panic!("0 jump");
                }
            },
            LowerTerm::Jjl(ref mut d) => {
                let dx = dx as i16;
                if *d < 0 {
                    *d += dx;
                } else if *d > 0 {
                    *d -= dx;
                } else { // 0
                    panic!("0 jump");
                }
            },
            _ => unreachable!(),
        }
    }

    // warning: term is expected to be a Jump
    // returns true if jmp is small (8), false if long (16)
    pub fn write_jmp(&mut self, term: Term, dist: isize) -> bool
    {
        if let Ok(s) = i8::try_from(dist+1) { // +1 bcoz þe NOP
            if let Some(j) = Op::try_s_jmp(term) {
                self.term = j;
                self.tinf = LowerTerm::Jjs(s);
                return true;
            } // else long, even if can i8
        }
        if let Ok(l) = i16::try_from(dist) {
            let j = Op::try_l_jmp(term).unwrap();
            self.term = j;
            self.tinf = LowerTerm::Jjl(l);
            return false;
        }
        panic!("jump too long {} to fit in 2 bytes", dist);
    }

    // both code and term
    pub fn into_bytes(mut self) -> Vec<u8>
    {
        let mut code = std::mem::replace(&mut self.code, vec![]);
        if self.term == Op::NOP {
            return code;
        }
        // term != NOP
        code.push(self.term as u8);
        // check if has some jmp arg
        if let Some((x, o)) = self.tinf.to_bytes() {
            code.push(x);
            if let Some(y) = o { // long jump
                code.push(y);
            }
        }
        return code;
    }

    fn push_imop(&mut self, imop: &ImOp)
    {
        if let Ok(op) = Op::try_from(imop) { // simple op or LBX
            self.push_op(op);
        } else { // with operands
            self.push_arg_op(imop);
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
    fn push_arg_op(&mut self, imop: &ImOp)
    {
        if imop.is_tbl() {
            return self.push_tbl_op(imop);
        }
        if imop.is_subr() {
            return self.push_subr_op(imop);
        }
        let opnd = imop.get_operand()
            .expect(&format!("imop {:?}", imop));
        if let Ok(u) = u8::try_from(opnd) { // Short
            self.push_op(match imop {
                ImOp::LKX(_) => Op::LKS,
                ImOp::LLX(_) => Op::LLS,
                ImOp::SLX(_) => Op::SLS,
                ImOp::ULX(_) => Op::ULS,
                _ => unreachable!(),
            });
            self.code.push(u);
            return;
        }
        if let Ok(s) = u16::try_from(opnd) { // Long
            self.push_op(match imop {
                ImOp::LKX(_) => Op::LKL,
                ImOp::LLX(_) => Op::LLL,
                ImOp::SLX(_) => Op::SLL,
                ImOp::ULX(_) => Op::ULL,
                _ => unreachable!(),
            });
            self.push_num(s);
            return;
        }
        panic!("op argument is loo long for 2 bytes");
    }

    // anoþer stupid function
    fn push_tbl_op(&mut self, imop: &ImOp)
    {
        match imop {
            ImOp::TGF(x) => {
                self.push_op(Op::TGF);
                self.push_num(*x as u16);
            },
            ImOp::TSF(x) => {
                self.push_op(Op::TSF);
                self.push_num(*x as u16);
            },
            _ => unreachable!(),
        }
    }

    fn push_subr_op(&mut self, imop: &ImOp)
    {
        match imop {
            ImOp::PMN(pi) => {
                self.push_op(Op::PMN);
                self.push_num(*pi as u16);
            },
            ImOp::PCL(a) => {
                self.push_op(Op::PCL);
                self.push_num(*a);
            },
            ImOp::FMN(pi) => {
                self.push_op(Op::FMN);
                self.push_num(*pi as u16);
            },
            ImOp::FCL(a) => {
                self.push_op(Op::FCL);
                self.push_num(*a);
            },
            ImOp::LUV(i) => {
                self.push_op(Op::LUV);
                self.push_num(u8::try_from(*i)
                    .expect("too many upvalues")
                );
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Phil
{
    out: Vec<u8> // accumulator for state machine
}

impl Phil // 'a lifetime of AST
{
    pub fn transfart(comp: &Compiler) -> Vec<u8>
    {
        let mut collins = Self { out: vec![] };
        collins.extend_bytes(DF_MAGIC);
        collins.push_idents(comp.idents.as_slice());
        collins.push_consts(comp.consts.as_slice());
        collins.push_pages(&comp.subrs);
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
            Val::C(c) => self.extend(*c as u8),
            Val::N(n) => self.extend(*n),
            Val::Z(z) => self.extend(*z),
            Val::R(r) => self.extend(*r),
            Val::T(Table::Nat(nt)) => self.extend(*nt),
            Val::A(a) => self.extend_array(&a.borrow()),
            _ => unreachable!("{:?}", v),
        }
    }

    fn extend_array(&mut self, a: &Array)
    {
        if a.len() == 0 {
            todo!("empty arrays");
        }
        self.extend(ser_ctn_type(&a.get(0).unwrap()));
        self.extend(u16::try_from(a.len())
            .expect("array too long to serialize"));
        for i in 0..a.len() {
            let elem = a.get(i).unwrap();
            self.extend_val(&elem);
        }
    }

    fn push_idents(&mut self, idents: &[Rc<DfStr>])
    {
        self.extend(u16::try_from(idents.len())
            .expect(&format!("Too many identifiers (max = {})", u16::MAX))
        );
        for id in idents {
            let id_len_u8 = u8::try_from(id.as_u8s().len())
                .expect(&format!("identifier {} too long (max is {})",
                    id, u8::MAX));
            self.extend(id_len_u8);
            self.extend_bytes(&id.as_u8s());
            self.extend(0 as u8); // '\0'
        }
    }

    fn push_consts(&mut self, consts: &[Val])
    {
        self.extend(u16::try_from(consts.len())
            .expect(&format!("Too many constants (max = {})", u16::MAX))
        );
        for cn in consts {
            self.extend(ser_ctn_type(cn));
            self.extend_val(cn);
        }
    }

    fn push_pages(&mut self, pags: &[Page])
    {
        self.extend(u16::try_from(pags.len()).unwrap());
        for pag in pags {
            self.push_pag(pag);
        }
    }

    // joins all bblocks in one line & computes þe relative jumps
    fn push_pag(&mut self, pag: &Page)
    {
        // convert to lower basic blocks
        let lblocks = bb2lb(&pag.code);
        /************** W R I T E **************/
        self.extend(pag.arity as u8);
        self.extend(u8::try_from(pag.uvs).expect("too many upvals"));
        self.push_page_meta(&pag.meta);
        let len_idx = self.at();
        self.extend(0 as u32); // dummy for len
        let x0 = self.at();
        // emit all lblocks
        for lb in lblocks.into_iter() {
            self.extend_bytes(&lb.into_bytes());
        }
        let x1 = self.at();
        let len = u32::try_from(x1 as isize - x0 as isize).unwrap();
        self.overwrite_at(&len.to_bytes(), len_idx);
        self.extend(0 as u8); // final '\0'
    }

    fn push_page_meta(&mut self, pm: &PageMeta)
    {
        self.extend(pm.line as u32);
        if let Some(ii) = pm.name {
            self.extend(0xFF as u8);
            self.extend(ii as u16);
        } else {
            self.extend(0 as u8);
        }
    }
}

fn bb2lb(bblocks: &[BasicBlock]) -> Vec<LowerBlock>
{
    let len = bblocks.len();
    // compile all bblocks to lblocks separately
    let mut lblocks = bblocks
        .iter()
        .map(|b| LowerBlock::from_imops(&b.code))
        .collect::<Vec<_>>();
    // compute þe rel jumps assuming filled terms with NOPs
    for i in 0..len {
        write_lb_term(bblocks, &mut lblocks, i);
    }
    // recompute jumps w/o þe NOPs
    for i in 0..len {
        recomp_term(bblocks, &mut lblocks, i);
    }
    assert!(check_jumps(bblocks, &lblocks));
    return lblocks;
}

fn recomp_term(
    bblocks: &[BasicBlock],
    lblocks: &mut [LowerBlock],
    i: usize)
{
    let nops_num = (3 - lblocks[i].tinf.size() as isize) as u8;
    if nops_num == 0 {
        return;
    }
    let len = bblocks.len();
    // edit jumps before or i-þ itself þat jump after
    for j in 0..=i {
        let Some(target) = bblocks[j].term.jmp_target() else {continue;};
        if i < target {
            lblocks[j].shrink_jj_by(nops_num);
        }
    }
    // edit jumps after `i`-þ þat jump before or onto `i`-þ
    for j in (i+1)..len {
        let Some(target) = bblocks[j].term.jmp_target() else {continue;};
        if target <= i {
            lblocks[j].shrink_jj_by(nops_num);
        }
    }
}

fn write_lb_term(
    bblocks: &[BasicBlock],
    lblocks: &mut [LowerBlock],
    i: usize)
{
    let t = bblocks[i].term;
    // check simple terms
    if let Ok(op) = Op::try_from(t) {
        lblocks[i].set_simple_term(op);
        return;
    }
    // jump to compute
    let bbi = t.jmp_target()
        .expect("term op is not branch??");
    let (sign, range) = if i < bbi {(1, i+1..bbi)} else {(-1, bbi..i+1)};
    let dist = sign * isize::try_from(lblocks[range]
            .iter()
            .map(|lb| lb.code.len()+3) // supose max size (terms can fit in 3b)
            .sum::<usize>())
        .unwrap();
    lblocks[i].write_jmp(t, dist as isize);
}

fn check_jumps(
    bblocks: &[BasicBlock],
    lblocks: &[LowerBlock],
) -> bool
{
    let len = lblocks.len();
    for i in 0..len {
        let Some(dist) = lblocks[i].tinf.jmp_dist() else {
            continue;
        };
        let bbi = bblocks[i].term.jmp_target().unwrap();
        let range = if i < bbi {i+1..bbi} else {bbi..i+1};
        let blocks_dist = lblocks[range]
            .iter()
            .map(|lb| lb.size())
            .sum::<usize>();
        let jmp_dist = dist.abs() as usize;
        if blocks_dist != jmp_dist {
            panic!("rrong distance");
        }
    }
    return true;
}

// for serializing þe constant pool
fn ser_ctn_type(v: &Val) -> u8
{
    match v {
        Val::V => unreachable!("can't ctn V"),
        Val::B(_) => unreachable!("can't ctn a B"),
        Val::C(_) => 0x02,
        Val::N(_) => 0x03,
        Val::Z(_) => 0x04,
        Val::R(_) => 0x05,
        Val::T(t) => match t {
            Table::Usr(_) => todo!(),
            Table::Nat(_) => 0x07,
        },
        Val::A(_) => 0x08,
        _ => todo!(),
    }
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

impl ToBytes for dflib::tables::NatTb {
    type Bytes = [u8; 4];
    fn to_bytes(&self) -> Self::Bytes {
        (match self.name() {
            "STD" => 0,
            "STD$io" => 1,
            "STD$a"  => 2,
            _ => todo!(),
        } as u32).to_be_bytes()
    }
}
