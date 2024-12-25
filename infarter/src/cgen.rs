/* cgen.rs */

use num_enum::TryFromPrimitive;
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
    res.push_str(&declar_vars(&g.tmp));
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
                let s = if *op == TacBinOp::Bin(BinOpWt::MOZ) {
                    // mod, not C's rem %
                    let b = tb;
                    let c = tc;
                    format!("t{ta} = (t{b}%t{c}+t{c})%t{c};\n")
                } else {
                    format!("t{} = t{} {} t{};\n", ta, tb, tacbinop2c(op), tc)
                };
                res.push_str(&s);
            },
            Tac::LAB(i) => {
                res.push_str(&format!("L{i}:\n"));
            },
            Tac::JMP(i) => {
                res.push_str(&format!("goto L{i};\n"));
            },
            Tac::JIF(li, ti, b) => {
                res.push_str(&format!("if ({}t{}) goto L{};\n",
                    if *b {""} else {"!"}, ti, li));
            },
            _ => todo!(),
        }
    }
    res.push_str("\nreturn 0;\n}");
    return res;
}

// private types

// groups t indexes by type e.g. [B,N,B] -> {B:[0,2], N:[1]}
fn declar_vars(tmp: &[Type]) -> String
{
    // group by type
    let mut groups = std::collections::HashMap::<u8, Vec<u16>>::new();
    for (i, t) in tmp.iter().enumerate() {
        let t = *t as u8;
        if !groups.contains_key(&t) {
            groups.insert(t, vec![]);
        }
        let g = groups.get_mut(&t).unwrap().push(i as u16);
    }
    // to string
    let mut res = String::new();
    for (typ, grp) in groups.iter() {
        res.push_str(types_convert(&Type::try_from_primitive(*typ).unwrap()));
        res.push(' ');
        for idx in grp {
            res.push_str(&format!("t{idx},"));
        }
        res.pop(); // last comma ','
        res.push_str(";\n");
    }
    return res;
}

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

fn tacbinop2c(op: &TacBinOp) -> &'static str
{
    match op {
        TacBinOp::Bin(b) => binop2c(b),
        TacBinOp::Cmp(b) => cmpop2c(b),
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

fn cmpop2c(op: &CmpOpWt) -> &'static str
{
    match op {
        CmpOpWt::Equ(EquOpWt(b, _)) => if *b {"=="} else {"!="}, // TODO what for == in bools
        CmpOpWt::Ord(OrdOpWt(o, _)) => match o {
            OrdOp::Lt => "<",
            OrdOp::Le => "<=",
            OrdOp::Gt => ">",
            OrdOp::Ge => ">=",
        },
    }
}
