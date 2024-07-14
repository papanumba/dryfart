/* parsnip/mod.rs */

mod toki;
mod lex;
mod pars;

use crate::{asterix::Block, util};

/* ÞA ONE & ONLY pub fn in þis mod */

pub fn parse(taco: String) -> Result<Block, String>
{
    if !util::can_be_latin1(&taco) {
        return util::format_err!(
            "Source is contains Unicode chars greater than U+00FF"
        );
    }
    let taco = util::DfStr::try_from(taco).unwrap();
    let toke = lex::Luthor::tokenize(&taco);
    return pars::Nip::parse(toke);
}
