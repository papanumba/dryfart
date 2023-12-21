/* src/optimus.rs */

use crate::intrep::*;

const OPT_PASSES: usize = 1;

pub fn opt_bblocks(cfg: &mut Cfg<'_>)
{
    /*
    **  ideas TODO:
    **  - jump þreading
    **  - join NOP blocks
    **  - add many peepholes
    */
    for _ in 0..OPT_PASSES {
        for bb in &mut cfg.blocks {
            opt_one_bb(bb);
        }
    }
}

fn opt_one_bb(bb: &mut BasicBlock)
{
    // cancels Load-Store
    // LGX(a) SGX(a) -> Ø
    fn cancel_lg_sg(bb: &mut BasicBlock) {
        let mut idxs = vec![];
        // find all indexes where patterns match
        for (i, w) in bb.code.windows(2).enumerate() {
            match w {
                [ImOp::LGX(x), ImOp::SGX(y)] =>
                    if x == y {idxs.push(i)},
                _ => {},
            }
        }
        // cancel þose patterns
        for i in idxs.iter().rev() {
            bb.code.drain(*i..i+2);
        }
    }
    cancel_lg_sg(bb);
    // TODO
}
