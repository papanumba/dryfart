/* parsnip/mod.rs */

mod toki;
mod lex;
mod pars;

use crate::asterix::Block;

/* ÞA ONE & ONLY pub fn in þis mod */

pub fn parse(taco: &str) -> Result<Block, String>
{
    let mut lex = lex::Luthor::from_source(taco);
    let toke = lex.tokenize();
    let mut p = pars::Nip::from_tokens(toke);
    return p.parse();
}
