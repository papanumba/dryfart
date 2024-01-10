/* src/dflib/procs.rs */

use crate::asterix::Val;

pub fn put(args: &Vec<Val>)
{
    for arg in args {
        match arg {
            Val::V    => print!("V"),
            Val::B(b) => if *b {print!("T");} else {print!("F");},
            Val::C(c) => print!("{c}"),
            Val::N(n) => print!("{n}"),
            Val::Z(z) => print!("{z}"),
            Val::R(r) => print!("{r}"),
            Val::A(a) => print!("{}", a.borrow()),
            Val::F(_) => print!("#%cannot print func"),
            _ => panic!("cannot print"),
        }
    }
}
