/* src/dflib/procs.rs */

use crate::asterix::Val;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NatPc
{
    name: &'static str,
}

impl NatPc
{
    pub fn new(s: &'static str) -> Self
    {
        Self { name: s }
    }

    pub fn arity(&self) -> usize
    {
        match self.name {
            "put" => 1,
            "putLn" => 1,
            _ => panic!("unknown native proc {}", self.name),
        }
    }

    pub fn exec(&self, args: &[Val])
    {
        match self.name {
            "put" => put(args),
            "putLn" => put_ln(args),
            _ => panic!("unknown native proc {}", self.name),
        }
    }
}

fn put(args: &[Val])
{
    match &args[0] {
        Val::V    => print!("V"),
        Val::B(b) => if *b {print!("T");} else {print!("F");},
        Val::C(c) => print!("{c}"),
        Val::N(n) => print!("{n}"),
        Val::Z(z) => print!("{z}"),
        Val::R(r) => print!("{r}"),
        Val::A(a) => print!("{}", a.borrow()),
        Val::P(p) => print!("{:?}", p),
        //Val::F(_) => print!("#%cannot print func"),
        _ => panic!("cannot print"),
    }
}

fn put_ln(args: &[Val])
{
    put(args);
    println!();
}
