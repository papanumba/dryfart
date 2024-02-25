/* src/dflib/mod.rs */

use crate::asterix::Val;

//pub mod funcs;
pub mod procs;

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
    Val::new_nat_proc(name)
}
