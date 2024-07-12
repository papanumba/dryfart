/* dflib/procs.rs */

use crate::asterix::Val;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NatPc
{
    IO_PUT,
    IO_PUTLN,
    GC,
    A_EKE,
}

impl NatPc
{
    pub fn arity(&self) -> usize
    {
        match self {
            Self::IO_PUT   => 1,
            Self::IO_PUTLN => 1,
            Self::GC       => 0,
            Self::A_EKE    => 2,
        }
    }

    pub fn exec(&self, args: &[Val])
    {
        match self {
            Self::IO_PUT   => put(args),
            Self::IO_PUTLN => put_ln(args),
            Self::GC       => {}, // tarzan's Rc
            Self::A_EKE    => a_eke(args),
        }
    }
}

fn put(args: &[Val])
{
    print!("{}", args[0]);
}

fn put_ln(args: &[Val])
{
    put(args);
    println!();
}

fn a_eke(args: &[Val])
{
    if args.len() != 2 {
        panic!("STD$a$eke must recieve 2 args");
    }
    if let Val::A(a) = &args[0] {
        a.borrow_mut().try_push(&args[1]).unwrap();
    } else {
        panic!("passed arg0 to STD$a$eke must be _%");
    }
}
