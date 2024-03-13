/* src/dflib/mod.rs */

use crate::asterix::Val;

//pub mod funcs;
pub mod procs;
pub mod tables;

pub fn get(name: &str) -> Option<Val>
{
    match name {
        "STD"   => Some(make_tb("STD")),
        _ => panic!("unknown proc"),
    }
}

#[inline]
fn make_tb(name: &'static str) -> Val
{
    Val::new_nat_tb(name)
}
