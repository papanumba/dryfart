/* dflib/tables/mod.rs */

use crate::asterix::Val;
mod dfstd;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NatTb
{
    STD,
    IO,
    A,
}

impl NatTb
{
    pub fn get(&self, k: &str) -> Option<Val>
    {
        match self {
            Self::STD => dfstd::get(k),
            Self::IO => dfstd::io::get(k),
            Self::A  => dfstd::a::get(k),
        }
    }

    pub fn name(&self) -> &'static str
    {
        match self {
            Self::STD => "STD",
            Self::IO => "STD$io",
            Self::A => "STD$a",
        }
    }
}
