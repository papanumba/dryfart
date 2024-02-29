/* dflib/tables/mod.rs */

use crate::asterix::Val;
mod dfstd;

#[derive(Debug)]
pub struct NatTb
{
    name: &'static str,
}

impl NatTb
{
    pub fn new(s: &'static str) -> Self
    {
        Self { name: s }
    }

    pub fn get(&self, k: &str) -> Option<Val>
    {
        match self.name {
            "STD" => dfstd::get(k),
            "STD$io" => dfstd::io::get(k),
            _ => panic!("unknown nat table"),
        }
    }

    pub fn has(&self, k: &str) -> bool
    {
        match self.name {
            "STD" => dfstd::has(k),
            "STD$io" => dfstd::io::has(k),
            _ => panic!("unknown nat table"),
        }
    }
}
