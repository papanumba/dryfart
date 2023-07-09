/* src/main.rs */

#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

mod asterix;

fn main()
{
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 2 {
        parse_file(&argv[1]);
    } else {
        eprintln!("not rite numba of args");
    }
}

#[test]
fn test1()
{
    parse_file("./tests/test1.df");
}

#[test]
fn test2()
{
    parse_file("./tests/test2.df");
}

pub fn parse_file(fname: &str)
{
    let taco = read_file_to_string(fname);
    let parser = grammar::ProgParser::new();
    //let mut analizer = asterix::SemAnal::new();
    let res = parser.parse(&taco).unwrap();
    asterix::anal_check(&res);
}

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}
