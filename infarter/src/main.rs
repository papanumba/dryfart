/* src/main.rs */

//#![allow(dead_code, unused_variables)]

use std::io::Write;

pub mod parsnip;
pub mod asterix;
pub mod intrep;
pub mod optimus;
pub mod genesis;
pub mod tarzan;
pub mod dflib;
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
        eprintln!("not rite numba ({argc}) of args, must be 2, {}", argv[0]);
    }
}

pub fn parse_file(fname: &str)
{
    let taco: String = read_file_to_string(fname);
    match parsnip::parse(&taco) {
        Ok(b) => tarzan::anal_check(&b),
        Err(e) => println!("{e}"),
    }
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
    let ast = parsnip::parse(&taco).unwrap(); // prints parsnip error
    let mut cfg = intrep::Cfg::from_asterix(&ast);
    if opt {
        optimus::opt_bblocks(&mut cfg);
    }
    let mut ofile = std::fs::File::create(&ofname)
        .expect("could not create file");
    match ofile.write_all(&genesis::cfg_into_bytes(&cfg)) {
        Ok(_) =>   println!("Successfully transfarted to {ofname}"),
        Err(_) => eprintln!("Could not write to binary file"),
    }
}
