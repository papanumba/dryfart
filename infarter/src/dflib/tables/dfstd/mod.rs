/* dflib/tables/dfstd/mod.rs */

use crate::asterix::Val;
//use crate::dflib::procs::NatPc;

pub fn get(k: &str) -> Option<Val>
{
    match k {
        "io" => Some(Val::new_nat_tb("STD$io")),
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
