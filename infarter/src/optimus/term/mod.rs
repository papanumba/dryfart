/* src/optimus/term/mod.rs */

mod red; // reduction
mod thr; // jump threading

use crate::intrep::*;

pub fn reduce(bb: &mut BasicBlock, bbi: usize)
{
    red::red(bb, bbi);
}

pub fn thread(b: &mut [BasicBlock])
{
    thr::thr(b);
}
