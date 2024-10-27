/* optimus/term/mod.rs */

mod red; // reduction
//mod thr; // jump threading

use crate::intrep::BasicBlock;

pub fn reduce(bb: &mut BasicBlock, bbi: usize)
{
    red::red(bb, bbi);
}

/*pub fn thread(b: &mut [BasicBlock])
{
    thr::thr(b);
}*/
