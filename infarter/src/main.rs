/* main.rs */

#![allow(warnings)]

//use std::io::Write;

pub mod util;
pub mod tok;
pub mod lex;
pub mod ast;
pub mod pars;

fn main()
{
    let argv: Vec<_> = std::env::args().collect();
    let argc: usize = argv.len();
    if argc == 2 {
        parse_file(&argv[1]);
    } else if argc == 3 {
        match argv[1].as_str() {
//            "t"  => transfart(&argv[2], false),
//            "to" => transfart(&argv[2], true),
            "c"  => parse_file(&argv[2]),
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
    let dftaco = util::DfString::fast_from_string(taco);
    let tacodfstr = (&dftaco).into();
    let lexres = lex::tokenize(&tacodfstr);
    let tacou8: &[u8] = tacodfstr.as_ref();
    let lr = match lexres {
        Ok(lr) => lr,
        Err((e, n)) =>  {
            eprintln!("{}", lex::ErrorSrc::new(e, tacou8, &n));
            return;
        }
    };
    let asterix = pars::parse(&lr.tokens)
        .map_err(|e| e.exp)
        .unwrap();
    dbg!(&asterix);
/*    let mut ast = match parsnip::parse(taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };*/
//    let mut ast = semanal::semanalize(ast);
//    dbg!(&ast);
/*    tarzan::exec_main(&ast);*/
}

/*
pub fn transfart(ifname: &str, opt: bool)
{
/*    let taco: String = read_file_to_string(ifname);
    let mut ofname: String = ifname.to_owned();
    ofname.push('c');
    let mut ast = match parsnip::parse(taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    let mut ast = semanal::semanalize(ast);
    let mut cfg = intrep::Compiler::from_asterix(&ast);
    if opt {
        optimus::opt_bblocks(&mut cfg);
    }
//     dbg!(&cfg);
    let mut ofile = std::fs::File::create(&ofname)
        .expect("could not create file");
    match ofile.write_all(&genesis::comp_into_bytes(&cfg)) {
        Ok(()) => println!("Successfully transfarted{} to {}",
            if opt {" optimized"} else {""},
            ofname,
        ),
        Err(e) => eprintln!("Could not write to binary file because:\n {e}"),
    }*/
}

pub fn df2c(ifname: &str)
{
    let taco: String = read_file_to_string(ifname);
/*    let mut ofname: String = ifname.to_owned();
    ofname.push('c');
    let mut ast = match parsnip::parse(taco) {
        Ok(b) => b,
        Err(e) => {eprintln!("{e}"); return;},
    };
    let mut ast = semanal::semanalize(ast);*/
/*    let ir = ssa_ir::Gen::from_prog(&ast);
    dbg!(&ir);
    let c_code = cgen::into_c(&ir);
    println!("{}", &c_code);
    // sample.df -> sample.c
    let mut ofname = String::from(&ifname[..ifname.len()-3]);
    ofname.push_str(".c");
    let mut ofile = std::fs::File::create(&ofname)
        .expect("could not create file");
    match ofile.write_all(c_code.as_bytes()) {
        Ok(()) => println!("Successfully transfarted {ifname} to {ofname}"),
        Err(e) => eprintln!("Could not write to binary file because:\n {e}"),
    }*/
    panic!("wkejr");
}
*/

#[inline]
pub fn read_file_to_string(fname: &str) -> String
{
    return std::fs::read_to_string(fname)
            .expect("Should have been able to read the file");
}
