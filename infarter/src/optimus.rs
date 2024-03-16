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
            for bb in &mut pag.code {
                opt_one_bb(bb);
                opt_term(bb);
            }
        }
    }
}

fn opt_one_bb(bb: &mut BasicBlock)
{
    fn peephole<P, const N: usize, const M: usize>(
        bb: &mut BasicBlock,
        patt: P,
        subs: [ImOp; M])
    where P: Fn(&[ImOp]) -> bool // should be [ImOp; N] but window is a slice
    {
        let mut idxs = vec![];
        for (i, w) in bb.code.windows(2).enumerate() {
            if patt(w) {
                idxs.push(i);
            }
        }
        for i in idxs.iter().rev() {
            // resize if necessary
            if M < N {
                let d = N - M;
                bb.code.drain(*i..i+d);
            } else if M > N {
                unreachable!(); // opt shouldn't enlarge a block
            } else {
                // noþing
            }
            bb.code[*i..i+M].copy_from_slice(&subs);
        }
    }

    // cancels global Load-Store
    // LGX(a) SGX(a) -> Ø
    peephole::<_, 2, 0>(
        bb,
        |w| match w {
            [ImOp::LGX(x), ImOp::SGX(y)] => x == y,
            _ => false,
        },
        []
    );

    // L[NZ]1 ADD -> INC
    peephole::<_, 2, 1>(
        bb,
        |w| match w {
            [ImOp::LN1, ImOp::ADD] |
            [ImOp::LZ1, ImOp::ADD] => true,
            _ => false,
        },
        [ImOp::INC]
    );

    // LZ1 SUB -> DEC
    peephole::<_, 2, 1>(
        bb,
        |w| match w {
            [ImOp::LZ1, ImOp::SUB] => true,
            _ => false,
        },
        [ImOp::DEC]
    );

    // TODO continue adding crap
}

fn opt_term(bb: &mut BasicBlock)
{
    if bb.code.is_empty() {
        return;
    }
    if let Term::JFX(x) = bb.term {
        match bb.code.last().unwrap() {
            ImOp::CLT => {
                bb.code.pop();
                bb.term = Term::JGE(x);
            },
            ImOp::CLE => {
                bb.code.pop();
                bb.term = Term::JGT(x);
            },
            ImOp::CGT => {
                bb.code.pop();
                bb.term = Term::JLE(x);
            },
            ImOp::CGE => {
                bb.code.pop();
                bb.term = Term::JLT(x);
            },
            _ => {}, // TODO: add oþers
        }
    }
}
