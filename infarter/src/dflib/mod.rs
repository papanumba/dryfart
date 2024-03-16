/* src/dflib/mod.rs */

use crate::asterix::Val;

pub mod funcs;
pub mod procs;
pub mod tables;

pub fn get(name: &str) -> Option<Val>
{
    match name {
        "STD" => Some(Val::from(tables::NatTb::STD)),
        _ => panic!("unknown"),
    }
}
