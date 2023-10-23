/* src/lib_procs.rs */

use crate::asterix::Val;
use crate::twalker::Scope;

pub fn do_lib_pccall(scope: &mut Scope, name: &str, raw_args: &Vec<Val>)
{
    match name {
        "put" => tp::put(raw_args),
        _ => panic!("unknown proc"),
    };
}

mod tp
{
    use crate::asterix::Val;

    pub fn put(args: &Vec<Val>)
    {
        match args.len() {
            0 => println!(""),
            1 => match &args[0] {
                Val::B(b) => if *b {print!("T");} else {print!("F");},
                Val::C(c) => print!("{c}"),
                Val::N(n) => print!("{n}"),
                Val::Z(z) => print!("{z}"),
                Val::R(r) => print!("{r}"),
                Val::A(a) => print!("{a}"),
                _ => panic!("cannot print"),
            },
            _ => panic!("not rite numba of args calling show!"),
        };
    }
}
