/* dflib/tables/dfstd/mod.rs */

use crate::asterix::Val;

pub fn get(k: &str) -> Option<Val>
{
    match k {
        "io" => Some(Val::new_nat_tb(&"STD$io")),
        _ => None,
    }
}

pub fn has(k: &str) -> bool
{
    match k {
        "io" => true,
        _ => false,
    }
}

pub mod io
{
    use crate::asterix::Val;

    pub fn get(k: &str) -> Option<Val>
    {
        match k {
            "put"   => Some(Val::new_nat_proc(&"put")),
            "putLn" => Some(Val::new_nat_proc(&"putLn")),
            _ => None,
        }
    }

    pub fn has(k: &str) -> bool
    {
        match k {
            "put"   |
            "putLn" => true,
            _ => false,
        }
    }
}
