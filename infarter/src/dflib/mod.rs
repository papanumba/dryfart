/* src/dflib/mod.rs */

use std::rc::Rc;
use crate::asterix::Val;

//mod funcs;
mod procs;

pub fn get(name: &str) -> Option<Val>
{
    match name {
        "put"   => Some(make_pc("put")),
        "putLn" => Some(make_pc("putLn")),
        _ => panic!("unknown proc"),
    }
}

#[inline]
fn make_pc(name: &'static str) -> Val
{
    Val::P(Rc::new(procs::NatPc::new(name)))
}
