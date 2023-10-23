/* src/main.rs */

#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);
use regex;

pub mod asterix;
pub mod twalker;
pub mod lib_procs;
pub mod lib_funcs;
pub mod util;

fn main()
{
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 2 {
        parse_file(&argv[1]);
    } else {
        eprintln!("not rite numba ({argc}) of args, must be 2, {}", argv[0]);
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
    let mut taco: String = read_file_to_string(fname);
    // clear comments, bcoz i'm stupid
    // or þe parser can't handle comments
    clear_comments(&mut taco);
    // continue parsing
    let parser = grammar::ProgParser::new();
    let res = parser.parse(&taco).unwrap();
    twalker::anal_check(&res);
}

fn clear_comments(s: &mut String)
{
    let com_re = regex::Regex::new(r"'.*\n").unwrap();
    let result = com_re.replace_all(s, "\n");
    *s = result.to_string();
}

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}
