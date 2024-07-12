/* main.rs */

#![allow(
    clippy::needless_return,
    clippy::expect_fun_call,
    clippy::comparison_chain,
    clippy::result_unit_err,
    clippy::wrong_self_convention,
    clippy::redundant_field_names,
)]

use std::io::Write;

pub mod parsnip;
pub mod asterix;
pub mod tarzan;
pub mod dflib;
pub mod semanal;
pub mod intrep;
pub mod optimus;
pub mod genesis;
pub mod util;

fn main()
{
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 2 {
        parse_file(&argv[1]);
    } else if argc == 3 {
        match argv[1].as_str() {
            "t"  => transfart(&argv[2], false),
            "to" => transfart(&argv[2], true),
            _ => panic!("unknown option {}", argv[1]),
        }
    } else {
        eprintln!("not rite numba ({argc}) of args, must be 2, {}",
            argv[0]);
    }
}

pub fn parse_file(fname: &str)
{
    let taco: String = read_file_to_string(fname);
    let mut ast = match parsnip::parse(&taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    semanal::check(&mut ast);
    tarzan::exec_main(&ast);
}

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}

pub fn transfart(ifname: &str, opt: bool)
{
    let taco: String = read_file_to_string(ifname);
    let mut ofname: String = ifname.to_owned();
    ofname.push('c');
    let mut ast = match parsnip::parse(&taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    semanal::check(&mut ast);
    let mut cfg = intrep::Compiler::from_asterix(&ast);
    if opt {
        optimus::opt_bblocks(&mut cfg);
    }
    let mut ofile = std::fs::File::create(&ofname)
        .expect("could not create file");
    match ofile.write_all(&genesis::comp_into_bytes(&cfg)) {
        Ok(_) => if opt {
            println!("Successfully transfarted optimized to {ofname}");
        } else {
            println!("Successfully transfarted to {ofname}");
        },
        Err(_) => eprintln!("Could not write to binary file"),
    }
}
