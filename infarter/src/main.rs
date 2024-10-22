/* main.rs */

#![allow(warnings)]

use std::io::Write;

pub mod parsnip;
pub mod asterix;
pub mod semanal;
pub mod intrep;
pub mod genesis;
/*pub mod tarzan;
pub mod dflib;
pub mod optimus;*/
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
    let mut ast = match parsnip::parse(taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    let mut ast = semanal::semanalize(ast);
    dbg!(&ast);
/*    tarzan::exec_main(&ast);*/
}

pub fn transfart(ifname: &str, opt: bool)
{
    let taco: String = read_file_to_string(ifname);
    let mut ofname: String = ifname.to_owned();
    ofname.push('c');
    let mut ast = match parsnip::parse(taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    let mut ast = semanal::semanalize(ast);
    let mut cfg = intrep::Compiler::from_asterix(&ast);
/*    if opt {
        optimus::opt_bblocks(&mut cfg);
    }*/
    let mut ofile = std::fs::File::create(&ofname)
        .expect("could not create file");
    match ofile.write_all(&genesis::comp_into_bytes(&cfg)) {
        Ok(()) => println!("Successfully transfarted{} to {}",
            if opt {" optimized"} else {""},
            ofname,
        ),
        Err(e) => eprintln!("Could not write to binary file because:\n {e}"),
    }
}

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}
