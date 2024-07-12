/* dflib/funcs.rs */

use crate::{
    asterix::Val,
    util::StrRes,
    util,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NatFn
{
    A_LEN,
}

impl NatFn
{
    pub fn arity(&self) -> usize
    {
        match self {
            Self::A_LEN => 1,
        }
    }

    pub fn eval(&self, args: &[Val]) -> StrRes<Val>
    {
        match self {
            Self::A_LEN => a_len(args),
        }
    }
}

pub fn a_len(args: &[Val]) -> StrRes<Val>
{
    match &args[0] {
        Val::A(arr) => Ok(arr.borrow().len_val_n()),
        _ => util::format_err!("not rite typ'arg: expected array"),
    }
}
