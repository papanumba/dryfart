/* dflib/tables/dfstd/mod.rs */

use crate::asterix::Val;
use super::NatTb;

pub fn get(k: &str) -> Option<Val>
{
    match k {
        "io" => Some(Val::from(NatTb::IO)),
        "a"  => Some(Val::from(NatTb::A)),
        _ => None,
    }
}

pub mod io
{
    use crate::asterix::Val;
    use crate::dflib::procs::NatPc;

    pub fn get(k: &str) -> Option<Val>
    {
        match k {
            "put"   => Some(Val::new_nat_proc(NatPc::IO_PUT)),
            "putLn" => Some(Val::new_nat_proc(NatPc::IO_PUTLN)),
            _ => None,
        }
    }
}

pub mod a
{
    use crate::asterix::Val;
    use crate::dflib::{
        procs::NatPc,
        funcs::NatFn,
    };

    pub fn get(k: &str) -> Option<Val>
    {
        match k {
            "eke"   => Some(Val::new_nat_proc(NatPc::A_EKE)),
            "len"   => Some(Val::from(NatFn::A_LEN)),
            _ => None,
        }
    }
}
