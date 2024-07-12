/* optimus/mod.rs */

mod peep; // peephole
mod term; // BB's terminating op opt

use crate::intrep::*;

const OPT_PASSES: usize = 5;

pub fn opt_bblocks(comp: &mut Compiler)
{
    for _ in 0..OPT_PASSES {
        for pag in &mut comp.subrs {
            for (bbi, bb) in pag.code.iter_mut().enumerate() {
                peep::opt_one_bb(bb);
                term::reduce(bb, bbi);
            }
            term::thread(&mut pag.code);
        }
    }
}
