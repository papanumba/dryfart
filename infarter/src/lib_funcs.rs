/* src/lib_funcs.rs */

use crate::asterix::Val;

pub fn do_lib_fncall(name: &str, raw_args: &Vec<Val>) -> Val
{
    return match name {
        "len" => tp::len(raw_args),
        _ => panic!("unknown func {name}"),
    };
}

mod tp
{
    use crate::asterix::Val;

    pub fn len(args: &Vec<Val>) -> Val
    {
        let a = match args.len() {
            1 => &args[0],
            _ => panic!("not rite numbav args to len#, must be 1"),
        };
        return match a {
            Val::A(arr) => arr.len(),
            _ => panic!("ERROR@len#: not rite typ'arg: expected array"),
        };
    }
}

