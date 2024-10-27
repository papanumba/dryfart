/* optimus/term/red.rs */

use crate::intrep::*;

pub fn red(bb: &mut BasicBlock, bbi: usize)
{
    if bb.code.is_empty() {
        return;
    }
    match bb.term {
        Term::JMP(j, tgt) => jmp(bb, bbi, j, tgt),
//        Term::END    |
        Term::HLT => ignoring_terms(bb),
        _ => {}, // TODO eke
    }
}

fn jmp(bb: &mut BasicBlock, bbi: usize, jop: Jmp, tgt: BbIdx)
{
    match jop {
        Jmp::JX    => jjx(bb, tgt, bbi),
        Jmp::YX(b) => if b {todo!("jtx");} else {jfx(bb, tgt);},
        _ => {},
    }
}

fn jjx(bb: &mut BasicBlock, bbi: usize, tgt: BbIdx)
{
    // see if is a jump to the next block, no convert it to NOP
/*    if tgt == bbi + 1 {
        bb.term = Term::NOP;
    }*/
}

/*fn jtx(bb: &mut BasicBlock, x: BbIdx)
{
    let new_term = match bb.code.last().unwrap() {
        ImOp::LBX(b) => if *b {Term::JJX(x)} else {Term::NOP},
        ImOp::NOT => Term::JFX(x),
        ImOp::CEQ => Term::JEX(x),
        ImOp::CNE => Term::JNX(x),
        ImOp::CLT => Term::JLT(x),
        ImOp::CLE => Term::JLE(x),
        ImOp::CGT => Term::JGT(x),
        ImOp::CGE => Term::JGE(x),
        _ => return, // TODO: add oþers
    };
    bb.code.pop();
    bb.term = new_term;
}*/

fn jfx(bb: &mut BasicBlock, tgt: BbIdx)
{
    let new_term = match bb.code.last().unwrap() {
        ImOp::CMP(c) => Term::JMP(Jmp::CX(c.negated()), tgt),
        _ => return, // do noþing
    };
    bb.code.pop();
    bb.term = new_term;
//        ImOp::LBX(b) => if *b {Term::NOP} else {Term::JJX(x)},
//        ImOp::NOT => Term::JYX(x),
}

fn ignoring_terms(bb: &mut BasicBlock)
{
    if bb.code.last().unwrap() == &ImOp::POP {
        bb.code.pop();
    }
}
