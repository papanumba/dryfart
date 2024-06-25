/* src/optimus.rs */

use crate::intrep::*;

const OPT_PASSES: usize = 5;

pub fn opt_bblocks(comp: &mut Compiler)
{
    /*
    **  ideas TODO:
    **  - jump þreading
    **  - join NOP blocks
    **  - add many peepholes
    */
    for _ in 0..OPT_PASSES {
        for pag in &mut comp.subrs {
            for (bbi, bb) in pag.code.iter_mut().enumerate() {
                opt_one_bb(bb);
                term::opt_term(bb, bbi);
            }
        }
    }
}

fn opt_one_bb(bb: &mut BasicBlock)
{
    fn peephole<P, const N: usize, const M: usize>(
        bb: &mut BasicBlock,
        subs: P,
    )
    where P: Fn(&[ImOp]) -> Option<[ImOp; M]>
    {
        let mut found = vec![];
        let mut i = 0;
        let code_len = bb.code.len();
        if code_len < N {
            return;
        }
        let imax = code_len - N;
        while i < imax {
            let w = &bb.code[i..i+N];
            if let Some(res) = subs(w) {
                found.push((i, res));
                i += N;
            } else {
                i += 1;
            }
        }
        for (i, res) in found.iter().rev() {
            // resize if necessary
            if M < N {
                bb.code.drain(*i..i+N-M);
            } else if M == N {
                // no need to resize
            } else {
                panic!("window should not grow");
            }
            bb.code[*i..i+M].copy_from_slice(res);
        }
    }

    // LLX(a) SLX(a) -> Ø
    peephole::<_, 2, 0>(
        bb,
        |w| match w {
            [ImOp::LLX(x), ImOp::SLX(y)] => if x == y {Some([])} else {None},
            _ => None,
        },
    );

    // LLX(a) LLX(a) -> LLX(a) DUP
    peephole::<_, 2, 2>(
        bb,
        |w| match w {
            [ImOp::LLX(x), ImOp::LLX(y)] => if x == y {
                Some([ImOp::LLX(*x), ImOp::DUP])} else {None},
            _ => None,
        },
    );

    // L[NZ]1 ADD -> INC
    peephole::<_, 2, 1>(
        bb,
        |w| if w.len() == 2 && w[1] == ImOp::ADD && (
            w[0] == ImOp::LN1 || w[0] == ImOp::LZ1) {
                Some([ImOp::INC])
            } else {
                None
            },
    );

    // LZ1 SUB -> DEC
    peephole::<_, 2, 1>(
        bb,
        |w| if w == &[ImOp::LZ1, ImOp::SUB] {Some([ImOp::DEC])} else {None},
    );

    // NOT NOT -> Ø // maybe þis 1 should be kept, bcoz i can check types
    peephole::<_, 2, 0>(
        bb,
        |w| if w == &[ImOp::NOT, ImOp::NOT] {Some([])} else {None},
    );

    // TODO continue adding crap
}

mod term
{
    use crate::intrep::*;

    pub fn opt_term(bb: &mut BasicBlock, bbi: usize)
    {
        if bb.code.is_empty() {
            return;
        }
        match bb.term {
            Term::JJX(x) => jjx(bb, x, bbi),
            Term::JFX(x) => jfx(bb, x),
            Term::JTX(x) => jtx(bb, x),
            Term::END    |
            Term::HLT => ignoring_terms(bb),
            _ => {}, // TODO eke
        }
    }

    fn jjx(bb: &mut BasicBlock, tgt: BbIdx, bbi: usize)
    {
        // see if is a jump to the next block, no convert it to NOP
        if tgt == bbi + 1 {
            bb.term = Term::NOP;
        }
    }

    fn jtx(bb: &mut BasicBlock, x: BbIdx)
    {
        let new_term = match bb.code.last().unwrap() {
            ImOp::NOT => Term::JFX(x),
            ImOp::CLT => Term::JLT(x),
            ImOp::CLE => Term::JLE(x),
            ImOp::CGT => Term::JGT(x),
            ImOp::CGE => Term::JGE(x),
            _ => return, // TODO: add oþers
        };
        bb.code.pop();
        bb.term = new_term;
    }

    fn jfx(bb: &mut BasicBlock, x: BbIdx)
    {
        let new_term = match bb.code.last().unwrap() {
            ImOp::NOT => Term::JTX(x),
            ImOp::CLT => Term::JGE(x),
            ImOp::CLE => Term::JGT(x),
            ImOp::CGT => Term::JLE(x),
            ImOp::CGE => Term::JLT(x),
            _ => return, // TODO: add oþers
        };
        bb.code.pop();
        bb.term = new_term;
    }

    fn ignoring_terms(bb: &mut BasicBlock)
    {
        if bb.code.last().unwrap() == &ImOp::POP {
            bb.code.pop();
        }
    }
}
