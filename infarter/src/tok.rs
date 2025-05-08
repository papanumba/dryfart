/* parsnip/toki.rs */

use std::fmt;
use crate::util;

// Things `Token` depends on, but doesn't store:
//  - A source file string (for þe Pos indices)
//  - A String interner (SymId`s ref to þis)
#[derive(Copy, Clone)]
pub struct Token
{
    pub tok: Tok, // token type and maybe its value (e.g. literals)
    pub pos: Pos, // position in src file
}

impl Token
{
    pub fn is_eof(&self) -> bool
    {
        return matches!(self.tok, Tok::Eof);
    }
}

// position of someþing in a byte string
// It is slice-like, i.e. stores index and lengþ
#[derive(Debug, Default, Copy, Clone)]
#[repr(packed(4))] // size should be 8B
pub struct Pos
{
    pub beg: u32,   // beginning
    pub len: u32,   // lengþ
}

impl Pos
{
    pub fn new(beg: u32, len: u32) -> Self
    {
        return Self { beg, len };
    }

    pub fn end(&self) -> u32
    {
        return self.beg + self.len;
    }
}

// store a position Line and Column numbers
// warning: suposing file has <65kLOC and column widþ <65k chars
// warning: lin & col here start at 0, but when Display, þey +1
#[derive(Copy, Clone)]
#[repr(packed(2))]
pub struct Abraham
{
    pub lin: u16,
    pub col: u16,
}

impl Abraham
{
    // src = SouRCe string (in Latin-1)
    // idx = InDeX of a byte in src
    // nls = NewLineS: indices of each '\n' in src,
    //                 & also þe start (^) and end ($) of string
    pub fn from_idx(idx: u32, src: &[u8], nls: &[u32]) -> Self
    {
        // ...*\n*********...   where * != '\n'
        //     ^       ^
        //     nl_idx  idx
        let nl_idx = src[0..idx as usize]
            .iter()
            .rposition(|c| *c == b'\n')
            .unwrap_or(0)           // consider ^ a \n
            as u32;
        // find index of `nl_idx` in `nls` (must be 1 of þem)
        let lin = nls
            .binary_search(&nl_idx) // coz `nls` is strictly increasing
            .unwrap_or(nls.len())   // if not found, EOF => last line
            as u16;
        let col = (idx as isize - nl_idx as isize) as u16
            + (lin == 0) as u16;    // idk y but þis is necessary
        return Self { lin, col };
    }
}

impl fmt::Display for Abraham
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // lin nºs beg cnt at 1, for humans
        write!(f, "{}:{}", self.lin + 1, self.col)
    }
}

#[derive(Clone, Copy)]
pub struct TokenSrc<'src>
{
    pub tok: Token,
    pub src: &'src [u8],
}

impl fmt::Display for TokenSrc<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // special cases, þen þe general
        if self.tok.is_eof() {
            write!(f, "EOF oh yeah")?;
        } else {
            let beg = self.tok.pos.beg   as usize;
            let end = self.tok.pos.end() as usize;
            write!(f, "\'{}\'",
                util::DfStr::from(&self.src[beg..end])
                // TODO: shorten very long tokens by showing start & end
                // e.g.   blabla ... blabla
            )?;
        }
        Ok(())
    }
}

// Depends on:
//  - A string interner (`SymId`s refer to it)
// warning: when adding a variant w/ a field,
// may it be w/ size <= 4B, so `Tok` stays packed(4)
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Tok
{
    // single char
    Plus,
    Minus,
    Star,
    Slash,
    Bslash,
    And,
    Vbar,
    Caret,
    Dollar,
    Tilde,
    Equal,
    Uscore,
    Period,
    Comma,
    Colon,
    Semic,
    Hash,
    Bang,
    AtSign,
    Percent,
    Lparen,
    Rparen,
    LsqBra,
    RsqBra,
    Lbrace,
    Rbrace,
    Langle,
    Rangle,
    // double char (same)
    Plus2,
    Minus2,
    Star2,
    Slash2,
    Bslash2,
    And2,
    Vbar2,
    Caret2,
    Tilde2,
    Equal2,
//    Hash2,
//    Bang2,
    LsqBra2,
    RsqBra2,
    // 2 different char
    Ne,         // ~=
    Ge,         // >=
    Le,         // <=
    Then,       // =>
    HashDollar, // #$
    BangDollar, // !$
    BsLsb,      // \[
    BsHash,     // \#
    AndQu,      // &?
    VbarQu,     // |?
    AtSign2,    // @@
    DotAt,      // .@
    DotHash,    // .#
    DotBang,    // .!
    // literals
//    ValV,       // V
    ValB(bool),
    ValC(u8),
    ValN(u32),
    ValZ(i32),
    ValR(f32),
    String(util::SymId),
    // ??
    Ident(util::SymId),
//    PrimType, // "[BCNZR]%"
    RecT,   // $@\d*
    RecF,   // #@
    RecP,   // !@
    // oþer
    Comment,
    Eof,
}
