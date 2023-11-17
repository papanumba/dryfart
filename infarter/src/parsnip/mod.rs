/* src/parsnip/mod.rs */

mod luthor;
mod turnip;

use crate::twalker::Stmt;

/* ÞA ONE & ONLY pub fn in þis mod*/

pub fn parse(taco: &str) -> Result<Stmt, String>
{
    let mut lex = luthor::Lexer::new(&taco);
    let toke = lex.tokenize()?;
    let mut p = turnip::Parsnip::new(&toke);
    return p.parse();
}
