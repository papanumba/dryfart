/* cgen.rs */

use crate::{
    asterix::*,
    ssa_ir::*,
};

pub fn into_c(g: &Gen) -> String
{
    let mut res = String::new();
    res.push_str("#include <stdint.h>\n");
    res.push_str("\nint main(void)\n{\n");
    // declare'em all at begin
    for (i, t) in g.tmp.iter().enumerate() {
        res.push_str(types_convert(&g.tmp[i]));
        res.push_str(&format!(" t{i};\n"));
    }
    for c in &g.out {
        match c {
            Tac::CTN(a, ci) => {
                res.push_str(&format!("t{a} = "));
                res.push_str(&literal_in_c(&g.ctn.as_slice()[*ci as usize]));
                res.push_str(";\n");
            },
            Tac::CPY(ta, tb) => {
                res.push_str(&format!("t{ta} = t{tb};\n"));
            },
            Tac::UNO(ta, tb, op) => {
                res.push_str(&format!("t{} = {} t{};\n",
                    ta, uniop2c(op), tb));
            },
            Tac::BIO(ta, tb, op, tc) => {
                let s = if *op == BinOpWt::MOZ { // mod, not C's rem %
                    let b = tb;
                    let c = tc;
                    format!("t{ta} = (t{b}%t{c}+t{c})%t{c};\n")
                } else {
                    format!("t{} = t{} {} t{};\n", ta, tb, binop2c(op), tc)
                };
                res.push_str(&s);
            },
            _ => todo!(),
        }
    }
    res.push_str("\nreturn 0;\n}");
    return res;
}

// private types

fn types_convert(t: &Type) -> &'static str
{
    match t {
        Type::B => "int",
        Type::C => "uint8_t",
        Type::N => "uint32_t",
        Type::Z => "int32_t",
        Type::R => "double",
    }
}

fn literal_in_c(v: &Val) -> String
{
    match v {
        Val::B(b) => format!("{}", *b as i32), // 1, 0
        Val::C(c) => format!("{c}"),
        Val::N(n) => format!("{n}"),
        Val::Z(z) => format!("{z}"),
        Val::R(r) => format!("{r}"),
    }
}

fn uniop2c(op: &UniOpWt) -> &'static str
{
    match op {
        UniOpWt::NEZ |
        UniOpWt::NER => "-",
        UniOpWt::INR => "1.0 / ",
        UniOpWt::NOB => "!",
        UniOpWt::NOC |
        UniOpWt::NON => "~",
    }
}

fn binop2c(op: &BinOpWt) -> &'static str
{
    match op {
        BinOpWt::ADC | BinOpWt::ADN | BinOpWt::ADZ | BinOpWt::ADR => "+",
        BinOpWt::SUZ | BinOpWt::SUR => "-",
        BinOpWt::MUC | BinOpWt::MUN | BinOpWt::MUZ | BinOpWt::MUR => "*",
        BinOpWt::DIN | BinOpWt::DIR => "/",
        BinOpWt::MOC | BinOpWt::MON => "%",
        BinOpWt::MOZ => unreachable!(), // special case
        BinOpWt::ANB | BinOpWt::ANC | BinOpWt::ANN => "&",
        BinOpWt::IOB | BinOpWt::IOC | BinOpWt::ION => "|",
        BinOpWt::XOB | BinOpWt::XOC | BinOpWt::XON => "^",
    }
}
