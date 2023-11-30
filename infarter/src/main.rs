/* src/main.rs */

#![allow(dead_code, unused_variables)]

pub mod parsnip;
pub mod asterix;
pub mod codegen;
pub mod tarzan;
pub mod dflib;
pub mod util;

fn main()
{
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 2 {
        parse_file(&argv[1]);
    } else if argc == 3 { // `infarter t program.df` will give a program.dfc
        if argv[1] != "t" {
            panic!("missing -t flag");
        }
        transfart(&argv[2]);
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

pub fn transfart(ifname: &str)
{
    let taco: String = read_file_to_string(ifname);
    let mut ofname: String = ifname.to_owned();
    ofname.push('c');
    match parsnip::parse(&taco) {
        Ok(b) => codegen::transfart(&b, &ofname),
        Err(e) => println!("{e}"),
    }
}
