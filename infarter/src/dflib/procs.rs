/* src/dflib/procs.rs */

use crate::asterix::Val;
//use crate::twalker::Scope;

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
            Val::F(_) => print!("#%"),
//            _ => panic!("cannot print"),
        },
        _ => panic!("not rite numba of args calling show!"),
    }
}
