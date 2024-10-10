/* parsnip/toki.rs */

use num_enum;
use crate::util;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[derive(num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum PrimType {
    B = b'B',
    C = b'C',
    N = b'N',
    Z = b'Z',
    R = b'R',
}

impl TryFrom<&u8> for PrimType
{
    type Error = String;
    fn try_from(b: &u8) -> Result<Self, Self::Error>
    {
        match Self::try_from(*b) {
            Ok(x) => Ok(x),
            _ => util::format_err!("{}% is not a dftype", char::from(*b)),
        }
    }
}

pub type LnToken<'a> = (Token<'a>, usize);

#[derive(Copy, Clone)]
pub struct Token<'src>
{
    typ: TokTyp,        // token class
    val: TokVal<'src>,  // extracted value
    lex: &'src [u8],    // original lexeme
}

macro_rules! new_tok {
    ($tt:ident, $src:expr) => {
        Token::simple(TokTyp::$tt, $src)
    }
}

pub(crate) use new_tok;

macro_rules! parse_fn {
    ($fn_name:ident, $val_type:ty, $toktyp:ident) => {
        pub fn $fn_name(s: &'src [u8]) -> Self {
            return Self {
                typ: TokTyp::$toktyp,
                val: TokVal {
                    $toktyp: std::str::from_utf8(s)
                        .unwrap()
                        .parse::<$val_type>()
                        .unwrap()
                },
                lex: s,
            };
        }
    };
}

macro_rules! return_match {
    ($s:expr, $($c:pat => $t:ident),+; _ => $dft:expr) => {
        return match $s[0] {
            $($c => Ok(Token::simple(TokTyp::$t, $s)),)+
            _ => $dft,
        }
    }
}

macro_rules! acc_fn {
    ($fn_name:ident, $toktyp:ident, $val_type:ty) => {
        pub fn $fn_name(&self) -> Option<$val_type> {
            match self.typ {
                TokTyp::$toktyp => Some(unsafe{self.val.$toktyp}),
                _ => None,
            }
        }
    }
}

macro_rules! kinda_from_str {
    ($fn_name:ident, $toktyp:ident, $src:lifetime) => {
        pub fn $fn_name(s: &$src [u8]) -> Token<$src> {
            return Self {
                typ: TokTyp::$toktyp,
                val: TokVal {$toktyp: s},
                lex: s,
            };
        }
    }
}

macro_rules! level_fn {
    ($fn_name:ident, $toktyp:ident, $src:lifetime) => {
        pub fn $fn_name(lev: u32, s: &'src [u8]) -> Self {
            return Self {
                typ: TokTyp::$toktyp,
                val: TokVal{$toktyp: lev},
                lex: s,
            };
        }
    }
}

impl<'src> Token<'src>
{
    // "constructors"

    // only used when noþing is in self.val
    pub fn simple(t: TokTyp, lex: &'src [u8]) -> Self
    {
        return Self {typ:t, val:TokVal::default(), lex:lex};
    }

    pub fn new_eof() -> Self
    {
        return Self::simple(TokTyp::Eof, b"EOF");
    }

    pub fn new_valb(b: bool, s: &'src [u8]) -> Self
    {
        return Self {typ:TokTyp::ValB, val: TokVal{ValB:b}, lex:s};
    }

    pub fn new_valc(c: u8, s: &'src [u8]) -> Self
    {
        return Self {typ:TokTyp::ValC, val: TokVal{ValC:c}, lex:s};
    }

    pub fn new_primtype(pt: PrimType, s: &'src [u8]) -> Self
    {
        return Self {
            typ: TokTyp::PrimType,
            val: TokVal{PrimType: pt},
            lex: s,
        };
    }

    parse_fn!(parse_valn, u32, ValN);
    parse_fn!(parse_valz, i32, ValZ);
    parse_fn!(parse_valr, f64, ValR);

    kinda_from_str!(new_comment, Comment, 'src);
    kinda_from_str!(new_ident,   Ident,   'src);
    kinda_from_str!(new_string,  String,  'src);

    level_fn!(new_rect,  RecT,  'src);

    pub fn try_1gle_from(s: &'src [u8]) -> Result<Self, String>
    {
        return_match!(s,
            b'+' => Plus,       b'-' => Minus,      b'*' => Asterisk,
            b'/' => Slash,      b'\\'=> Bslash,     b'&' => And,
            b'|' => Vbar,       b'~' => Tilde,      b'=' => Equal,
            b'#' => Hash,       b'!' => Bang,       b'@' => AtSign,
            b'[' => LsqBra,     b']' => RsqBra,     b'_' => Uscore,
            b'.' => Period,     b',' => Comma,      b':' => Colon;
            _ => util::format_err!(
                "unknown single char token {}", char::from(s[0])
            )
        );
    }

    pub fn try_2ble_from(s: &'src [u8]) -> Result<Self, String>
    {
        return_match!(s,
            b'+' => Plus2,      b'-' => Minus2,     b'*' => Asterisk2,
            b'/' => Slash2,     b'&' => And2,       b'|' => Vbar2,
            b'~' => Tilde2,     b'=' => Equal2,     b'[' => LsqBra2,
            b']' => RsqBra2,    b'\\'=> Bslash2,    b'^' => Caret2,
            b'@' => AtSign2;
            _ => util::format_err!(
                "unknown double char token {0}{0}", char::from(s[0])
            )
        );
    }

    pub fn new_unknown(s: &'src [u8]) -> Self {
        return Self {
            typ: TokTyp::Unknown,
            val: TokVal {Unknown: s[0]},
            lex: s,
        };
    }

    // "getters"

    pub fn typ(&self) -> TokTyp
    {
        return self.typ;
    }

    acc_fn!(as_valb,     ValB,     bool);
    acc_fn!(as_valc,     ValC,     u8);
    acc_fn!(as_valn,     ValN,     u32);
    acc_fn!(as_valz,     ValZ,     i32);
    acc_fn!(as_valr,     ValR,     f64);
    acc_fn!(as_string,   String,   &'src [u8]);
    acc_fn!(as_ident,    Ident,    &'src [u8]);
    acc_fn!(as_primtype, PrimType, PrimType);
    acc_fn!(as_rect,     RecT,     u32);
    acc_fn!(as_comment,  Comment,  &'src [u8]);
    //acc_fn!(as_unknown,  Unknown,  u8); // unused

    // "askers"

    pub fn is_cmp(&self) -> bool
    {
        matches!(self.typ,
            TokTyp::Equal2
            | TokTyp::Ne
            | TokTyp::Rangle
            | TokTyp::Langle
            | TokTyp::Le
            | TokTyp::Ge
        )
    }
}

impl<'src> std::fmt::Display for Token<'src>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", util::DfStr::from(&self.lex))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokTyp
{
    // single char
    Plus,
    Minus,
    Asterisk,
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
    ValV,
    ValB,
    ValC,
    ValN,
    ValZ,
    ValR,
    String,
    // ??
    Ident,
    PrimType, // "[BCNZR]%"
    RecT,   // $@\d*
    RecF,   // #@
    RecP,   // !@
    // oþer
    Comment,
    Unknown,
    Eof,
}

#[allow(non_snake_case)] // to reuse þe enum & union fields
#[derive(Clone, Copy)]
pub union TokVal<'src>
{
    Other:    (),
    ValB:     bool,
    ValC:     u8,
    ValN:     u32,
    ValZ:     i32,
    ValR:     f64,
    String:   &'src [u8],
    Ident:    &'src [u8],
    PrimType: PrimType,
    RecT:     u32,          // þe level $@N
    Comment:  &'src [u8],
    Unknown:  u8,           // u8 as char
}

impl Default for TokVal<'_>
{
    fn default() -> Self
    {
        Self {Other: ()}
    }
}
