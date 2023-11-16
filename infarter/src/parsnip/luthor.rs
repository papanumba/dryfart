/* src/lexer.rs */

#[derive(Debug)]
pub struct Lexer<'src>
{
    input: &'src [u8],
    line:     usize,  // current line
    base_pos: usize,  // first position from which trying to get a token
    next_pos: usize,  // final position of the trying token
}

impl<'src> Lexer<'src>
{
    pub fn new(s: &'src str) -> Self
    {
        if !s.is_ascii() {
            panic!("passed string is not ascii");
        }
        return Self {
            input: s.as_bytes(),
            line: 1,
            base_pos: 0,
            next_pos: 0,
        };
    }

    pub fn tokenize(&mut self) -> Result<Vec<(Token<'src>, usize)>, String>
    {
        self.init();
        let mut res = Vec::new();
        while let Some(t) = self.next_token() {
            res.push((t, self.line));
        }
        res.push((Token::Eof, self.line));
        return Ok(res);
    }

    fn init(&mut self)
    {
        self.line     = 1;
        self.base_pos = 0;
        self.next_pos = 0;
    }

    #[inline]
    fn is_at_end(&self) -> bool
    {
        return self.next_pos == self.input.len();
    }

    #[inline]
    fn advance(&mut self)
    {
        if !self.is_at_end() {
            self.next_pos += 1;
        }
    }

    // skips whitespaces and updates self.line when finding '\n'
    #[inline]
    fn skip_whites(&mut self)
    {
        while let Some(w) = self.peek::<0>() {
            if w.is_ascii_whitespace() {
                if *w == b'\n' {
                    self.line += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    // LA: lookahead, 0 -> peek current char, 1 -> peek next
    #[inline]
    fn peek<const LA: usize>(&self) -> Option<&'src u8>
    {
        return self.input.get(self.next_pos + LA);
    }

    #[inline]
    fn lexeme(&self) -> &'src [u8]
    {
        return &self.input[self.base_pos..self.next_pos];
    }

    // match a char at LA lookahead
    #[inline]
    fn matches<const LA: usize>(&self, m: u8) -> bool
    {
        return if let Some(c) = self.peek::<LA>() {
            *c == m
        } else {
            false
        };
    }

    #[inline]
    fn read_char(&mut self) -> Option<&'src u8>
    {
        let tmp = self.input.get(self.next_pos);
        self.advance();
        return tmp;
    }

    #[inline]
    fn has_digit_next(&self) -> bool
    {
        return if let Some(c) = self.peek::<1>() {
            c.is_ascii_digit()
        } else {
            false
        };
    }

    // checks if current char is == next char
    #[inline]
    fn is_at_2ble(&self, c0: u8) -> bool
    {
        return if let Some(c) = self.peek::<0>() {
            *c == c0
        } else {
            false
        };
    }

    fn next_token(&mut self) -> Option<Token<'src>>
    {
        self.skip_whites();
        self.base_pos = self.next_pos;
        return if let Some(c) = self.read_char() {
            Some(match c {
                b'_' => Token::Uscore,
                b'.' => Token::Period,
                b',' => Token::Comma,
                b'@' => Token::AtSign,
                b'(' => Token::Lparen,
                b')' => Token::Rparen,
                b'[' => Token::LsqBra,
                b']' => Token::RsqBra,
                b'{' => Token::Lbrace,
                b'}' => Token::Rbrace,
                b'+' | b'-' | b'*' | b'/' |
                b'&' | b'|' | b'#' | b'!' => self.maybe_2ble(*c),
                b'~' => self.from_tilde(),  // ~, ~~, ~=
                b'=' => self.from_equal(),  // =, ==, =>
                b'<' => self.from_langle(), // <, <=
                b'>' => self.from_rangle(), // >, >=
                b'0'..=b'9' => self.get_num(), // N or R
                b'a'..=b'z' | b'A'..=b'Z' => self.get_ident(),
                b'"' => self.get_string(),
                 _ => Token::Unknown(*c),
            })
        } else {
            None
        };
    }

    // gets called when some of +-*/&|~
    pub fn maybe_2ble(&mut self, c: u8) -> Token<'src>
    {
        if self.is_at_2ble(c) {
            if let Ok(t) = Token::try_2ble_from(c) {
                self.advance();
                return t;
            } else {
                panic!("not a double {0}{0}", char::from(c));
            }
        } else {
            return Token::try_from(&c).unwrap();
        }
    }

    // ~, ~~, ~=
    #[inline]
    fn from_tilde(&mut self) -> Token<'src>
    {
        if let Some(c) = self.peek::<0>() {
            if *c == b'~' {
                self.advance();
                return Token::Tilde2;
            }
            if *c == b'=' {
                self.advance();
                return Token::Ne;
            }
        }
        return Token::Tilde;
    }

    // =, ==, =>
    #[inline]
    fn from_equal(&mut self) -> Token<'src>
    {
        if let Some(c) = self.peek::<0>() {
            if *c == b'=' {
                self.advance();
                return Token::Equal2;
            }
            if *c == b'>' {
                self.advance();
                return Token::Then;
            }
        }
        return Token::Equal;
    }

    // <, <=
    #[inline]
    fn from_langle(&mut self) -> Token<'src>
    {
        if self.matches::<0>(b'=') {
            self.advance();
            return Token::Le;
        }
        return Token::Langle;
    }

    // >, >=
    #[inline]
    fn from_rangle(&mut self) -> Token<'src>
    {
        if self.matches::<0>(b'=') {
            self.advance();
            return Token::Ge;
        }
        return Token::Rangle;
    }

    // gets called when current char is a digit
    fn get_num(&mut self) -> Token<'src>
    {
        while let Some(c) = self.peek::<0>() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                 break;
            }
        }
        // til here we'll have a N% "\d+" number
        // þen check weþr it's a R% "\d+\.\d+"
        if !(self.matches::<0>(b'.') && self.has_digit_next()) {
            return Token::parse_valn(self.lexeme());
        }
        self.advance(); // get past þe dot '.'
        while let Some(c) = self.peek::<0>() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                 break;
            }
        }
        return Token::parse_valr(self.lexeme());
    }

    // gets called when current char is a letter
    // result can be Token::{Ident, PrimType}
    fn get_ident(&mut self) -> Token<'src>
    {
        if let Some(pt) = self.try_prim_type() {
            self.advance();
            return Token::PrimType(pt);
        }
        while let Some(c) = self.peek::<0>() {
            if c.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let lex = self.lexeme();
        if lex.len() == 1 { // try boolean keywords
            match lex[0] {
                b'T' => return Token::ValB(true),
                b'F' => return Token::ValB(false),
                _ => {},
            }
        }
        return Token::Ident(lex);
    }

    // gets called when parsing an Ident
    // if þe current lexeme is a PrimType,
    // returns Some(PrimType) but does not advance()
    fn try_prim_type(&self) -> Option<PrimType>
    {
        if self.matches::<0>(b'%') {
            let c = self.input[self.base_pos];
            if let Ok(pt) = PrimType::try_from(&c) {
                return Some(pt);
            }
        }
        return None;
    }

    // gets called when current is a double quote "
    fn get_string(&mut self) -> Token<'src>
    {
        let mut ended_string = false;
        while let Some(c) = self.peek::<0>() {
            if *c != b'"' {
                self.advance();
            } else {
                ended_string = true;
                break;
            }
        }
        if !ended_string {
            panic!("unterminated string at line {}", self.line);
        }
        let raw = &self.lexeme()[1..];
        self.advance(); // skip final quote
        return Token::String(raw);
    }
}

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
            _ => Err(String::from(
                format!("{}% is not a dftype", char::from(*b))
            )),
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
    Tilde,
    Equal,
    Uscore,
    Period,
    Comma,
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
    // 2 different char
    Ne,   // ~=
    Ge,   // >=
    Le,   // <=
    Then, // =>
    // literals
    ValB(bool),
    ValN(u32),
    ValR(f32),
    String(&'src [u8]),
    // ??
    Ident(&'src [u8]),
    PrimType(PrimType), // "[BCNZR]%"
    // oþer
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
            | Self::Rangle
            | Self::Langle
            | Self::Ne
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
            _ => Err(String::from(
                format!("unknown double char token {0}{0}", char::from(b))
            )),
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
/*            b'_' => Ok(Token::Uscore),
            b'.' => Ok(Token::Period),
            b',' => Ok(Token::Comma),
            b'#' => Ok(Token::Hash),*/
            _ => Err(String::from(
                format!("unknown single char token {}", char::from(*b))
            )),
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
    Tilde,
    Equal,
    Uscore,
    Period,
    Comma,
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
    // 2 different char
    Ne,   // ~=
    Ge,   // >=
    Le,   // <=
    Then, // =>
    // literals
    ValB,
    ValN,
    ValR,
    String,
    // ??
    Ident,
    PrimType, // "[BCNZR]%"
    // oþer
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
            Token::Tilde    => Self::Tilde,
            Token::Equal    => Self::Equal,
            Token::Uscore   => Self::Uscore,
            Token::Period   => Self::Period,
            Token::Comma    => Self::Comma,
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
            // 2 different char
            Token::Ne       => Self::Ne,
            Token::Ge       => Self::Ge,
            Token::Le       => Self::Le,
            Token::Then     => Self::Then,
            // literals
            Token::ValB(_)  => Self::ValB,
            Token::ValN(_)  => Self::ValN,
            Token::ValR(_)  => Self::ValR,
            Token::String(_)=> Self::String,
            // ??
            Token::Ident(_) => Self::Ident,
            Token::PrimType(_) => Self::PrimType, // "[BCNZR]%"
            // oþer
            Token::Unknown(_) => Self::Unknown,
            Token::Eof => Self::Eof,
        }
    }
}
