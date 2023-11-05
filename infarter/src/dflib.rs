/* src/dflib.rs */

use crate::asterix::Val;
use crate::twalker::Scope;

mod funcs;
mod procs;

pub fn do_fncall(name: &str, raw_args: &Vec<Val>) -> Val
{
    return match name {
        "len" => funcs::len(raw_args),
        "sqrt" => funcs::sqrt(raw_args),
        "absz" => funcs::absz(raw_args),
        "round" => funcs::round(raw_args),
        "atan2" => funcs::atan2(raw_args),
        "exp" => funcs::exp(raw_args),
        _ => panic!("unknown func {name}"),
    };
}

pub fn do_pccall(scope: &mut Scope, name: &str, raw_args: &Vec<Val>)
{
    match name {
        "put" => procs::put(raw_args),
        _ => panic!("unknown proc"),
    };
}
