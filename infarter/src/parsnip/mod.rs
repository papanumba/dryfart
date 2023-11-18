/* src/parsnip/mod.rs */

mod toki;
mod luthor;
mod turnip;

use crate::asterix::Block;

/* ÞA ONE & ONLY pub fn in þis mod*/

pub fn parse(taco: &str) -> Result<Block, String>
{
    let mut lex = luthor::Lexer::new(&taco);
    let toke = lex.tokenize()?;
    let mut p = turnip::Parsnip::new(&toke);
    return p.parse();
}
