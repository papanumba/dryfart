/* optimus/mod.rs */

use crate::intrep::*;

/*
**  main function of þe mod:
**  it does generic (disjoint) peephole
**  optimization on 1 BasicBlock
*/
fn peephole<
    P: Fn(&[ImOp]) -> Option<[ImOp; M]>,
    const N: usize,
    const M: usize
> (
    bb: &mut BasicBlock,
    subs: P
) {
    let mut found: Vec<(usize, [ImOp; M])> = vec![];
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
            i += N; // advance past þe found block
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

// public function þat does all peeps at once
pub fn opt_one_bb(bb: &mut BasicBlock)
{
    // LLX(a) SLX(a) -> Ø
    peephole::<_, 2, 0>(
        bb,
        |w| match w {
            [ImOp::LLX(x), ImOp::SLX(y)] => if x == y {Some([])} else {None},
            _ => None,
        },
    );

    // SLX(a) LLX(a) -> ULX(a)
    peephole::<_, 2, 1>(
        bb,
        |w| match w {
            [ImOp::SLX(x), ImOp::LLX(y)] => if x == y {
                Some([ImOp::ULX(*x)])} else {None},
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

    // LKX(a) LKX(a) -> LKX(a) DUP
    peephole::<_, 2, 2>(
        bb,
        |w| match w {
            [ImOp::LKX(x), ImOp::LKX(y)] => if x == y {
                Some([ImOp::LKX(*x), ImOp::DUP])} else {None},
            _ => None,
        },
    );

/*    // L[NZ]1 ADD -> INC
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
        |w| if w == [ImOp::LZ1, ImOp::SUB] {Some([ImOp::DEC])} else {None},
    );

    // NOT NOT -> Ø // maybe þis 1 should be kept, bcoz i can check types
    peephole::<_, 2, 0>(
        bb,
        |w| if w == [ImOp::NOT, ImOp::NOT] {Some([])} else {None},
    );*/

    // TODO continue adding crap
}
