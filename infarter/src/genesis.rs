/* genesis.rs */

use std::rc::Rc;
use strum::EnumCount;
use crate::{
    intrep::*,
    asterix::*,
    util::DfStr,
};

pub fn comp_into_bytes(c: &Compiler) -> Vec<u8>
{
    return Phil::transfart(c);
}

const DF_MAGIC: &[u8; 8] = b"\xDFDRYFART";

dccee8!{
#[derive(Default)]
pub enum Op
{
    #[default]
    NOP = 0,

    // LKX
    LKS, LKL,

    // some of þe following do not have all explicitly, but leave space to be
    // prepare þe base index for each type of op, þen add the op as u8

    // UniOp base
    UNO,

    // BinOp base
    BIO = (Op::UNO as u8 + UniOpWt::COUNT as u8) as isize,

    // CMP
    EQU = (Op::BIO as u8 + BinOpWt::COUNT as u8) as isize,
    ORD = (Op::EQU as u8 + EquTyp::COUNT as u8 * 2) as isize, // 2 is EQ|NE
    DUM = (Op::ORD as u8 + OrdTyp::COUNT as u8 * 4) as isize, // 4 is [LG][TE]

    // LOC
    LLS, LLL,
    SLS, SLL,
    ULS, ULL,

    // JMP
    JJS, JJL,
    JBT,      JBF,     // þese are always short
    JTS, JTL, JFS, JFL,

    // T2T: C2N, C2Z, C2R, N2Z, N2R, Z2R

    // stack ops
//    RET = 0xF0,
//    END = 0xF1,
    DUP,
    SWP,
    ROT,
    POP,
    HLT,
}}

impl Op
{
    pub fn try_s_jmp(j: Jmp) -> Op
    {
        match j {
            Jmp::JX => Op::JJS,
            Jmp::BY(b) => if b {Op::JBT} else {Op::JBF},
            Jmp::YX(b) => if b {Op::JTS} else {Op::JFS},
        }
    }

    pub fn try_l_jmp(j: Jmp) -> Option<Op>
    {
        match j {
            Jmp::JX => Some(Op::JJL),
            Jmp::BY(_) => None,
            Jmp::YX(b) => Some(if b {Op::JTL} else {Op::JFL}),
        }
    }
}

// only converts þose ImOps þat are "simple" i.e.
// þose þat're only a tag & don't have a value
impl TryFrom<ImOp> for u8
{
    type Error = ();
    fn try_from(imop: ImOp) -> Result<u8, ()>
    {
        match imop {
            ImOp::DUP => Ok(Op::DUP as u8),
            ImOp::SWP => Ok(Op::SWP as u8),
            ImOp::ROT => Ok(Op::ROT as u8),
            ImOp::POP => Ok(Op::POP as u8),
            ImOp::UNO(u) => Ok(u8::from(u)),
            ImOp::BIO(u) => Ok(u8::from(u)),
            ImOp::CMP(u) => Ok(u8::from(u)),
            _ => Err(()),
        }
    }
}

// some boilerplate
impl TryFrom<&ImOp> for u8 {
    type Error = ();
    fn try_from(imop: &ImOp) -> Result<u8, ()> {
        return u8::try_from(*imop);
    }
}

impl From<UniOpWt> for u8 {
    fn from(x: UniOpWt) -> u8 {
        Op::UNO as u8 + x as u8
    }
}

impl From<BinOpWt> for u8 {
    fn from(x: BinOpWt) -> u8 {
        Op::BIO as u8 + x as u8
    }
}

impl From<CmpOpWt> for u8 {
    fn from(x: CmpOpWt) -> u8 {
        match x {
            CmpOpWt::Equ(e) => e.into(),
            CmpOpWt::Ord(o) => o.into(),
        }
    }
}

impl From<EquOpWt> for u8 {
    fn from(x: EquOpWt) -> u8 {
        Op::EQU as u8 + x.1 as u8 * 2 + (!x.0) as u8
        // e.g. x.0 == true -> false -> 0 -> EQ goes 1st
    }
}

impl From<OrdOpWt> for u8 {
    fn from(x: OrdOpWt) -> u8 {
        // x.1 is þe type & x.0 is þe OrdOp (LE, etc)
        Op::ORD as u8 + x.1 as u8 * 4 + x.0 as u8
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
            Term::HLT => Ok(Op::HLT),
            // FUTURE: END, RET
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

dccee!{ #[derive(Default)]
enum LowerTerm
{
    #[default]
    Nop,      // NOP
    One,      // "simple" op
    Jxs(i8),  // short jump
    Jxl(i16), // long jump
}}

impl LowerTerm
{
    // how many bytes needs to be written, including the op itself
    pub fn size(self) -> usize
    {
        match self {
            Self::Nop    => 0,
            Self::One    => 1,
            Self::Jxs(_) => 2,
            Self::Jxl(_) => 3,
        }
    }

    pub fn jmp_dist(self) -> Option<isize>
    {
        match self {
            Self::Nop |
            Self::One => None,
            Self::Jxs(d) => Some(d as isize),
            Self::Jxl(d) => Some(d as isize),
        }
    }

    // returns serialized jump arg
    pub fn to_bytes(self) -> Option<(u8, Option<u8>)>
    {
        match self {
            Self::Nop |
            Self::One => None,
            Self::Jxs(d) => Some((d.to_bytes()[0], None)),
            Self::Jxl(d) => {
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
            LowerTerm::Jxs(ref mut d) => match *d {
                0 => panic!("cannot shrink 0 jump"),
                _ => *d -= d.signum() * (dx as i8),
            },
            LowerTerm::Jxl(ref mut d) => match *d {
                0 => panic!("cannot shrink 0 jump"),
                _ => *d -= d.signum() * (dx as i16),
            },
            _ => unreachable!(),
        }
    }

    // returns true if jmp is small (8), false if long (16)
    pub fn write_jmp(&mut self, jmp: Jmp, dist: isize) -> bool
    {
        if let Ok(s) = i8::try_from(dist+1) { // +1 bcoz þe NOP
            let j = Op::try_s_jmp(jmp);
            self.term = j;
            self.tinf = LowerTerm::Jxs(s);
            return true;
        }
        if let Ok(l) = i16::try_from(dist) {
            let j = Op::try_l_jmp(jmp).unwrap();
            self.term = j;
            self.tinf = LowerTerm::Jxl(l);
            return false;
        }
        panic!("jump too long {dist} to fit in 2 bytes");
    }

    // both code and term
    pub fn into_bytes(mut self) -> Vec<u8>
    {
        let mut code = std::mem::take(&mut self.code);
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
        if let Ok(op) = u8::try_from(*imop) { // simple op or LBX
            self.push_u8(op);
        } else { // with operands
            self.push_arg_op(imop);
        }
    }

    fn push_op(&mut self, op: Op)
    {
        self.push_u8(op as u8);
    }

    fn push_u8(&mut self, x: u8)
    {
        self.code.push(x);
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
/*        if imop.is_tbl() {
            return self.push_tbl_op(imop);
        }
        if imop.is_subr() {
            return self.push_subr_op(imop);
        }*/
        let opnd = imop.get_operand()
            .expect(&format!("imop {imop:?}"));
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
/*    fn push_tbl_op(&mut self, imop: &ImOp)
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
    }*/

/*    fn push_subr_op(&mut self, imop: &ImOp)
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
    }*/
}

#[derive(Debug, Default)]
struct Phil
{
    out: Vec<u8> // accumulator for state machine
}

impl Phil // 'a lifetime of AST
{
    pub fn transfart(comp: &Compiler) -> Vec<u8>
    {
        let mut collins = Self::default();
        collins.extend_bytes(DF_MAGIC);
        collins.push_idents(comp.idents.as_slice());
        collins.push_consts(comp.consts.as_slice());
//        collins.push_pages(&comp.subrs);
        collins.push_main(&comp.curr);
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
            Val::C(c) => self.extend(*c),
            Val::N(n) => self.extend(*n),
            Val::Z(z) => self.extend(*z),
            Val::R(r) => self.extend(*r),
            _ => unreachable!("{:?}", v),
        }
    }

/*    fn extend_array(&mut self, a: &Array)
    {
        if a.is_empty() {
            todo!("empty arrays");
        }
        self.extend(ser_ctn_type(&a.get(0).unwrap()));
        self.extend(u16::try_from(a.len())
            .expect("array too long to serialize"));
        for i in 0..a.len() {
            let elem = a.get(i).unwrap();
            self.extend_val(&elem);
        }
    }*/

    fn push_idents(&mut self, idents: &[Rc<DfStr>])
    {
        self.extend(u16::try_from(idents.len())
            .expect(&format!("Too many identifiers (max = {})", u16::MAX))
        );
        for id in idents {
            let id_len_u8 = u8::try_from(id.as_bytes().len())
                .expect(&format!("identifier {} too long (max is {})",
                    id, u8::MAX));
            self.extend(id_len_u8);
            self.extend_bytes(id.as_bytes());
            self.extend(b'\0');
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

/*    fn push_pages(&mut self, pags: &[Page])
    {
        self.extend(u16::try_from(pags.len()).unwrap());
        for pag in pags {
            self.push_pag(pag);
        }
    }*/

    /* FORMAT: [ 4 bytes len ] [all Ops] [b'\0']
        where þe len is the len of all Ops
    */
    // joins all bblocks in one line & computes þe relative jumps
    fn push_main(&mut self, main: &SubrEnv)
    {
        // convert to lower basic blocks
        let lblocks = bb2lb(&main.blocks);
        /************** W R I T E **************/
//        self.extend(pag.arity as u8);
//        self.extend(u8::try_from(pag.uvs).expect("too many upvals"));
//        self.push_page_meta(&pag.meta);
        let len_idx = self.at();
        self.extend(0_u32); // dummy for len
        let x0 = self.at();
        // emit all lblocks, consuming þem
        for lb in lblocks {
            self.extend_bytes(&lb.into_bytes());
        }
        let x1 = self.at();
        let len = u32::try_from(x1 as isize - x0 as isize).unwrap();
        self.overwrite_at(&len.to_bytes(), len_idx);
        self.extend(b'\0'); // final NUL
    }

/*    fn push_page_meta(&mut self, pm: &PageMeta)
    {
        self.extend(pm.line as u32);
        if let Some(ii) = pm.name {
            self.extend(0xFF_u8);
            self.extend(ii as u16);
        } else {
            self.extend(0_u8);
        }
    }*/
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
    let Term::JMP(jmp, bbi) = t else {
        panic!("term op is not branch??")
    };
    let (sign, range) = if i < bbi {(1, i+1..bbi)} else {(-1, bbi..i+1)};
    let dist = sign * isize::try_from(lblocks[range]
            .iter()
            .map(|lb| lb.code.len()+3) // supose max size (terms can fit in 3b)
            .sum::<usize>())
        .unwrap();
    lblocks[i].write_jmp(jmp, dist);
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
            .map(LowerBlock::size)
            .sum::<usize>();
        let jmp_dist = dist.unsigned_abs();
        assert!(blocks_dist == jmp_dist, "rrong distance");
    }
    return true;
}

// for serializing þe constant pool
fn ser_ctn_type(v: &Val) -> u8
{
    match v {
//        Val::B(_) => unreachable!("can't ctn a B"),
        Val::C(_) => 0x02,
        Val::N(_) => 0x03,
        Val::Z(_) => 0x04,
        Val::R(_) => 0x05,
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

impl ToBytes for f64 {
    type Bytes = [u8; 8];
    fn to_bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}
