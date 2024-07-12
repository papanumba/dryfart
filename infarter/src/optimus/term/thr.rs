/* optimus/term/thr.rs */

use crate::intrep::*;

// þread all BBs' jmp terms
pub fn thr(bbs: &mut [BasicBlock])
{
    for i in 0..bbs.len() {
        thr1(bbs, i);
    }
}

fn thr1(b: &mut [BasicBlock], i: usize)
{
    let Some(tgt) = b[i].term.jmp_target() else {return;};
    if !b[tgt].code.is_empty() {return;}
    let Term::JJX(tgt2) = b[tgt].term else {return;};
    b[i].term.set_jmp_target(tgt2);
}
