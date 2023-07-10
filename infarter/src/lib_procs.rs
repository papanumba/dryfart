/* src/lib_procs.rs */

use crate::asterix::{Scope, Proc, Expr};

pub fn do_lib_pccall(scope: &mut Scope, name: &str, raw_args: &Vec<Box<Expr>>)
{
    match name {
        "show" => do_lib_show(scope, raw_args),
        _ => panic!("unknown proc"),
    };
}

fn do_lib_show(scope: &mut Scope, raw_args: &Vec<Box<Expr>>)
{
    match raw_args.len() {
        0 => println!("called show!"),
        _ => panic!("not rite numba of args calling show!"),
    };
}
