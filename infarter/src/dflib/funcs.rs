/* src/dflib/funcs.rs */

use crate::asterix::Val;

pub fn len(args: &Vec<Val>) -> Val
{
    let a = match args.len() {
        1 => &args[0],
        _ => panic!("not rite numbav args to len#, must be 1"),
    };
    return match a {
        Val::A(arr) => arr.borrow().len_val_n(),
        _ => panic!("ERROR@len#: not rite typ'arg: expected array"),
    };
}

/* MATH */

// R%#{R%,}
pub fn sqrt(args: &Vec<Val>) -> Val
{
    let x = match args.len() {
        1 => &args[0],
        _ => panic!("not rite numbav args to sqrt#, must be 1"),
    };
    return match x {
        Val::R(r) => Val::R(f32::sqrt(*r)),
        _ => panic!("ERROR sqrt#: not rite typ'arg: expected R%"),
    };
}

// Z%#{R%,}
pub fn round(args: &Vec<Val>) -> Val
{
    let x = match args.len() {
        1 => &args[0],
        _ => panic!("not rite numbav args to round#, must be 1"),
    };
    return match x {
        Val::R(r) => Val::Z(f32::round(*r) as i32),
        _ => panic!("ERROR round#: not rite typ'arg: expected R%"),
    };
}

// N%#{Z%,} | R%#Z%;
pub fn abs(args: &Vec<Val>) -> Val
{
    let x = match args.len() {
        1 => &args[0],
        _ => panic!("not rite numbav args to absz#, must be 1"),
    };
    return match x {
        Val::Z(z) => Val::N(i32::abs(*z) as u32),
        Val::R(r) => Val::R(f32::abs(*r)),
        _ => panic!("ERROR absz#: not rite typ'arg: expected Z%"),
    };
}

// R%#{R%,R%,}
pub fn atan2(args: &Vec<Val>) -> Val
{
    let yx = match args.len() {
        2 => (&args[0], &args[1]),
        _ => panic!("not rite numbav args to atan2#, must be 2"),
    };
    return match yx {
        (Val::R(y), Val::R(x)) => Val::R(f32::atan2(*y, *x)),
        _ => panic!("ERROR atan2#: not rite typ'arg: expected {{R%,R%,}}"),
    };
}

// R%#{R%,}
pub fn exp(args: &Vec<Val>) -> Val
{
    let x = match args.len() {
        1 => &args[0],
        _ => panic!("not rite numbav args to exp#, must be 1"),
    };
    return match x {
        Val::R(r) => Val::R(f32::exp(*r)),
        _ => panic!("ERROR exp#: not rite typ'arg: expected R%"),
    };
}
