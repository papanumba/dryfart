/* src/dflib/procs.rs */

use crate::asterix::Val;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NatPc
{
    IO_PUT,
    IO_PUTLN,
    GC,
}

impl NatPc
{
    pub fn try_from_name(s: &str) -> Result<Self, ()>
    {
        match s {
            "put"   => Ok(Self::IO_PUT),
            "putLn" => Ok(Self::IO_PUTLN),
            _ => Err(()),
        }
    }

    pub fn arity(&self) -> usize
    {
        match self {
            Self::IO_PUT   => 1,
            Self::IO_PUTLN => 1,
            Self::GC       => 0,
        }
    }

    pub fn exec(&self, args: &[Val])
    {
        match self {
            Self::IO_PUT   => put(args),
            Self::IO_PUTLN => put_ln(args),
            Self::GC       => {}, // tarzan's Rc
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
