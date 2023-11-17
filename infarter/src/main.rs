/* src/main.rs */

#![allow(dead_code, unused_variables)]

//use regex;

pub mod parsnip;
pub mod asterix;
pub mod twalker;
pub mod dflib;
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

pub fn parse_file(fname: &str)
{
    let mut taco: String = read_file_to_string(fname);
    clear_comments(&mut taco);
    match parsnip::parse(&taco) {
        Ok(b) => twalker::anal_check(&b),
        Err(e) => println!("{e}"),
    }
}

fn clear_comments(s: &mut String)
{
//    let com_re = regex::Regex::new(r"'.*\n").unwrap();
//    let result = com_re.replace_all(s, "\n");
//    *s = result.to_string();
}

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}
