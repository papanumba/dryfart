/* src/parsnip/toki.rs */

use crate::util;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PrimType { B, C, N, Z, R }

impl TryFrom<&u8> for PrimType
{
    type Error = String;
    fn try_from(b: &u8) -> Result<Self, Self::Error>
    {
        return match b {
            b'B' => return Ok(Self::B),
            b'C' => return Ok(Self::C),
            b'N' => return Ok(Self::N),
            b'Z' => return Ok(Self::Z),
            b'R' => return Ok(Self::R),
            _ => util::format_err!("{}% is not a dftype", char::from(*b)),
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Token<'src>
{
    // single char
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Vbar,
    Caret,
    Dollar,
    Tilde,
    Equal,
    Uscore,
    Period,
    Comma,
    Semic,
    Hash,
    Bang,
    AtSign,
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
    Asterisk2,
    Slash2,
    And2,
    Vbar2,
    Tilde2,
    Equal2,
    Hash2,
    Bang2,
    AtSign2,
    LsqBra2,
    RsqBra2,
    // 2 different char
    Ne,   // ~=
    Ge,   // >=
    Le,   // <=
    Then, // =>
    // literals
    ValB(bool),
    ValN(u32),
    ValZ(i32),
    ValR(f32),
    String(&'src [u8]),
    // ??
    Ident(&'src [u8]),
    PrimType(PrimType), // "[BCNZR]%"
    RecT(u32),
    RecF,
    RecP,
    // oþer
    Comment(&'src [u8]),
    Unknown(u8),
    Eof,
}

impl Token<'_>
{
    pub fn is_eof(&self) -> bool
    {
        return match self {
            Self::Eof => true,
            _ => false,
        }
    }

    pub fn is_cmp(&self) -> bool
    {
        return match self {
            Self::Equal2
            | Self::Ne
            | Self::Rangle
            | Self::Langle
            | Self::Le
            | Self::Ge => true,
            _ => false,
        }
    }

    pub fn parse_valn(s: &[u8]) -> Self
    {
        return Token::ValN(
            std::str::from_utf8(s)
            .unwrap()
            .parse::<u32>()
            .unwrap()
        );
    }

    pub fn parse_valz(s: &[u8]) -> Self
    {
        return Token::ValZ(
            std::str::from_utf8(s)
            .unwrap()
            .parse::<i32>()
            .unwrap()
        );
    }

    pub fn parse_valr(s: &[u8]) -> Self
    {
        return Token::ValR(
            std::str::from_utf8(s)
            .unwrap()
            .parse::<f32>()
            .unwrap()
        );
    }

    pub fn try_2ble_from(b: u8) -> Result<Self, String>
    {
        match b {
            b'+' => Ok(Token::Plus2),
            b'-' => Ok(Token::Minus2),
            b'*' => Ok(Token::Asterisk2),
            b'/' => Ok(Token::Slash2),
            b'&' => Ok(Token::And2),
            b'|' => Ok(Token::Vbar2),
            b'~' => Ok(Token::Tilde2),
            b'=' => Ok(Token::Equal2),
            b'#' => Ok(Token::Hash2),
            b'!' => Ok(Token::Bang2),
            b'@' => Ok(Token::AtSign2),
            b'[' => Ok(Token::LsqBra2),
            b']' => Ok(Token::RsqBra2),
            _ => util::format_err!(
                "unknown double char token {0}{0}", char::from(b)
            ),
        }
    }
}

impl<'src> TryFrom<&u8> for Token<'src>
{
    type Error = String;
    fn try_from(b: &u8) -> Result<Self, Self::Error>
    {
        match b {
            b'+' => Ok(Token::Plus),
            b'-' => Ok(Token::Minus),
            b'*' => Ok(Token::Asterisk),
            b'/' => Ok(Token::Slash),
            b'&' => Ok(Token::And),
            b'|' => Ok(Token::Vbar),
            b'~' => Ok(Token::Tilde),
            b'=' => Ok(Token::Equal),
            b'#' => Ok(Token::Hash),
            b'!' => Ok(Token::Bang),
            b'@' => Ok(Token::AtSign),
            b'[' => Ok(Token::LsqBra),
            b']' => Ok(Token::RsqBra),
            b'_' => Ok(Token::Uscore),
            b'.' => Ok(Token::Period),
            b',' => Ok(Token::Comma),
            _ => util::format_err!(
                "unknown single char token {}", char::from(*b)
            ),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType
{
    // single char
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Vbar,
    Caret,
    Dollar,
    Tilde,
    Equal,
    Uscore,
    Period,
    Comma,
    Semic,
    Hash,
    Bang,
    AtSign,
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
    Asterisk2,
    Slash2,
    And2,
    Vbar2,
    Tilde2,
    Equal2,
    Hash2,
    Bang2,
    AtSign2,
    LsqBra2,
    RsqBra2,
    // 2 different char
    Ne,   // ~=
    Ge,   // >=
    Le,   // <=
    Then, // =>
    // literals
    ValB,
    ValN,
    ValZ,
    ValR,
    String,
    // ??
    Ident,
    PrimType, // "[BCNZR]%"
    RecT,
    RecF,
    RecP,
    // oþer
    Comment,
    Unknown,
    Eof,
}

impl<'src> From<&Token<'src>> for TokenType
{
    fn from(t: &Token<'src>) -> Self
    {
        match t {
            Token::Plus     => Self::Plus,
            Token::Minus    => Self::Minus,
            Token::Asterisk => Self::Asterisk,
            Token::Slash    => Self::Slash,
            Token::And      => Self::And,
            Token::Vbar     => Self::Vbar,
            Token::Caret    => Self::Caret,
            Token::Dollar   => Self::Dollar,
            Token::Tilde    => Self::Tilde,
            Token::Equal    => Self::Equal,
            Token::Uscore   => Self::Uscore,
            Token::Period   => Self::Period,
            Token::Comma    => Self::Comma,
            Token::Semic    => Self::Semic,
            Token::Hash     => Self::Hash,
            Token::Bang     => Self::Bang,
            Token::AtSign   => Self::AtSign,
            Token::Lparen   => Self::Lparen,
            Token::Rparen   => Self::Rparen,
            Token::LsqBra   => Self::LsqBra,
            Token::RsqBra   => Self::RsqBra,
            Token::Lbrace   => Self::Lbrace,
            Token::Rbrace   => Self::Rbrace,
            Token::Langle   => Self::Langle,
            Token::Rangle   => Self::Rangle,
            // double char (same)
            Token::Plus2    => Self::Plus2,
            Token::Minus2   => Self::Minus2,
            Token::Asterisk2=> Self::Asterisk2,
            Token::Slash2   => Self::Slash2,
            Token::And2     => Self::And2,
            Token::Vbar2    => Self::Vbar2,
            Token::Tilde2   => Self::Tilde2,
            Token::Equal2   => Self::Equal2,
            Token::Hash2    => Self::Hash2,
            Token::Bang2    => Self::Bang2,
            Token::AtSign2  => Self::AtSign2,
            Token::LsqBra2  => Self::LsqBra2,
            Token::RsqBra2  => Self::RsqBra2,
            // 2 different char
            Token::Ne       => Self::Ne,
            Token::Ge       => Self::Ge,
            Token::Le       => Self::Le,
            Token::Then     => Self::Then,
            // literals
            Token::ValB(_)  => Self::ValB,
            Token::ValN(_)  => Self::ValN,
            Token::ValZ(_)  => Self::ValZ,
            Token::ValR(_)  => Self::ValR,
            Token::String(_)=> Self::String,
            // ??
            Token::Ident(_) => Self::Ident,
            Token::PrimType(_) => Self::PrimType, // "[BCNZR]%"
            Token::RecT(_) => Self::RecT,
            Token::RecF    => Self::RecF,
            Token::RecP    => Self::RecP,
            // oþer
            Token::Comment(_) => Self::Comment,
            Token::Unknown(_) => Self::Unknown,
            Token::Eof => Self::Eof,
        }
    }
}
