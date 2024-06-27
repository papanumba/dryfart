/* src/optimus/term/red.rs */

use crate::intrep::*;

pub fn red(bb: &mut BasicBlock, bbi: usize)
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
        ImOp::LBX(b) => if *b {Term::JJX(x)} else {Term::NOP},
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
        ImOp::LBX(b) => if *b {Term::NOP} else {Term::JJX(x)},
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
