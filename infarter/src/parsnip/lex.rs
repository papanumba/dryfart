/* src/parsnip/lex.rs */

use super::toki::new_tok;
use super::toki::{Token, TokTyp, PrimType};
use crate::asterix::Val;
use crate::asterix;

macro_rules! if_next {
    ($zelf:expr, $c:expr, $tt:ident) => {
        if $zelf.matches::<0>($c) {
            $zelf.advance();
            return Token::simple(TokTyp::$tt, $zelf.lexeme());
        }
    };
}

macro_rules! lex_new_tok {
    ($zelf:expr, $tt:ident) => {
        new_tok!($tt, $zelf.lexeme())
    }
}

macro_rules! from_char_fn {
    ($name:ident, $base:ident, $($ch:expr => $t:ident,)+) => {
        #[inline]
        fn $name(&mut self) -> Token<'src>
        {
            $(if_next!(self, $ch, $t);)+
            return lex_new_tok!(self, $base);
        }
    }
}

#[derive(Debug)]
pub struct Luthor<'src>
{
    input: &'src [u8],
    line:     usize,  // current line
    base_pos: usize,  // first position from which trying to get a token
    next_pos: usize,  // final position of the trying token
}

impl<'src> Luthor<'src>
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
            match t.as_comment() {
                Some(_) => continue,
                _ => res.push((t, self.line)),
            }
        }
        res.push((Token::new_eof(), self.line));
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

    #[inline]
    fn adv_while<COND>(&mut self, cond: COND)
    where COND: Fn(u8) -> bool
    {
        while let Some(c) = self.peek::<0>() {
            if cond(*c) {
                self.advance();
            } else {
                 break;
            }
        }
    }

    // skips whitespaces and updates self.line when finding '\n'
    #[inline]
    fn skip_whites(&mut self)
    {
        while let Some(w) = self.peek::<0>() {
            if !w.is_ascii_whitespace() {
                break;
            }
            if *w == b'\n' {
                self.line += 1;
            }
            self.advance();
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
        let Some(c) = self.read_char() else {
           return None;
        };
        Some(match c {
            b'_' => lex_new_tok!(self, Uscore),
            b'.' => lex_new_tok!(self, Period),
            b',' => lex_new_tok!(self, Comma),
            b';' => lex_new_tok!(self, Semic),
            b'(' => lex_new_tok!(self, Lparen),
            b')' => lex_new_tok!(self, Rparen),
            b'{' => lex_new_tok!(self, Lbrace),
            b'}' => lex_new_tok!(self, Rbrace),
            b'^' => lex_new_tok!(self, Caret),
            b'+' | b'-' | b'*' | b'/' | b'@' | b'[' | b']'
                 => self.maybe_2ble(*c),
            b'&' => self.from_and(),
            b'|' => self.from_vbar(),
            b'#' => self.from_hash(),
            b'!' => self.from_bang(),
            b'$' => self.from_dollar(),
            b'~' => self.from_tilde(),  // ~, ~~, ~=
            b'=' => self.from_equal(),  // =, ==, =>
            b'<' => self.from_langle(), // <, <=
            b'>' => self.from_rangle(), // >, >=
            b'\\'=> self.from_bslash(),
            b'0'..=b'9' => self.get_num(), // N, Z or R
            b'a'..=b'z' | b'A'..=b'Z' => self.get_ident(),
            b'\'' => self.get_string(),
            b'"' => self.get_char(),
            b'`' => self.comment(),
             _ => Token::new_unknown(self.lexeme()),
        })
    }

    // gets called when some of +-*/&|~
    pub fn maybe_2ble(&mut self, c: u8) -> Token<'src>
    {
        if !self.is_at_2ble(c) {
            return Token::try_1gle_from(self.lexeme()).unwrap();
        }
        self.advance();
        if let Ok(t) = Token::try_2ble_from(self.lexeme()) {
            return t;
        }
        panic!("not a double {0}{0}", char::from(c));
    }

    from_char_fn!{from_and, And,
        b'&' => And2,
        b'?' => AndQu,
    }

    from_char_fn!{from_vbar, Vbar,
        b'|' => Vbar2,
        b'?' => VbarQu,
    }

    from_char_fn!{from_tilde, Tilde,
        b'~' => Tilde2,
        b'=' => Ne,
    }

    from_char_fn!{from_equal, Equal,
        b'=' => Equal2,
        b'>' => Then,
    }

    from_char_fn!{from_langle, Langle,
        b'=' => Le,
    }

    from_char_fn!{from_rangle, Rangle,
        b'=' => Ge,
    }

    from_char_fn!{from_bang, Bang,
        b'!' => Bang2,
        b'@' => RecP,
        b'$' => BangDollar,
    }

    from_char_fn!{from_hash, Hash,
        b'#' => Hash2,
        b'@' => RecF,
        b'$' => HashDollar,
    }

    from_char_fn!{from_bslash, Bslash,
        b'[' => BsLsb,
        b'#' => BsHash,
    }

    // $, $@[0-9]*
    #[inline]
    fn from_dollar(&mut self) -> Token<'src>
    {
        let Some(c) = self.peek::<0>() else {
            return lex_new_tok!(self, Dollar);
        };
        if *c != b'@' {
            return lex_new_tok!(self, Dollar);
        }
        if self.has_digit_next() {
            self.advance(); // @
            self.adv_while(|c| c.is_ascii_digit());
            let level = std::str::from_utf8(&self.lexeme()[2..])
                .unwrap().parse::<u32>().unwrap();
            return Token::new_rect(level, self.lexeme());
        } else { // default level
            self.advance(); // @
            return Token::new_rect(0, self.lexeme());
        }
    }

    // gets called when current char is a digit
    fn get_num(&mut self) -> Token<'src>
    {
        self.adv_while(|c| c.is_ascii_digit());
        if self.matches::<0>(b'U') || self.matches::<0>(b'u') {
            let n = Token::parse_valn(self.lexeme());
            self.advance(); // [Uu]
            return n;
        }
        // til here we'll have a "\d+" number
        // þen check weþr it's a R% "\d+\.\d+"
        if !(self.matches::<0>(b'.') && self.has_digit_next()) {
            return Token::parse_valz(self.lexeme());
        }
        self.advance(); // get past þe dot '.'
        self.adv_while(|c| c.is_ascii_digit());
        return Token::parse_valr(self.lexeme());
    }

    // gets called when current char is a letter
    // result can be Token::{Ident, PrimType}
    fn get_ident(&mut self) -> Token<'src>
    {
        if let Some(pt) = self.try_prim_type() {
            self.advance();
            return Token::new_primtype(pt, self.lexeme());
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
                b'V' => return new_tok!(ValV, lex),
                b'T' => return Token::new_valb(true,  lex),
                b'F' => return Token::new_valb(false, lex),
                _ => {},
            }
        }
        return Token::new_ident(lex);
    }

    // gets called when parsing an Ident
    // if þe current lexeme is a PrimType,
    // returns Some(PrimType) but does not advance()
    fn try_prim_type(&self) -> Option<PrimType>
    {
        if !self.matches::<0>(b'%') {
            return None;
        }
        let c = self.input[self.base_pos];
        return if let Ok(pt) = PrimType::try_from(&c) {
            Some(pt)
        } else {
            None
        };
    }

    // called when '
    fn get_string(&mut self) -> Token<'src>
    {
        let mut ended_string = false;
        while let Some(c) = self.read_char() {
            if *c == b'\'' {
                ended_string = true;
                break;
            }
            if *c == asterix::ESC_CH {
                if self.read_char().is_none() {
                    panic!(
                        "expected escape char but found EOF at line {}",
                        self.line);
                }
                // will check later if þe escapes are valid
            }
        }
        if !ended_string {
            panic!("unterminated string at line {}", self.line);
        }
        let lxm = self.lexeme();
        let raw = &lxm[1..lxm.len()-1];
        return Token::new_string(raw);
    }

    // called when "
    fn get_char(&mut self) -> Token<'src>
    {
        let Some(c) = self.read_char() else {
            panic!("unterminated C% literal at EOF");
        };
        if *c == asterix::ESC_CH { // escapes
            let Some(d) = self.read_char() else {
                panic!("unterminated escaped C% at EOF");
            };
            let Ok(e) = Val::escape_char(*d) else {
                panic!("unknown escape char \"{d}");
            };
            let Some(b'"') = self.read_char() else {
                panic!("unterminated C% literal, at line {}", self.line);
            };
            return Token::new_valc(e, self.lexeme());
        }
        // normal chars
        let Some(b'"') = self.read_char() else {
            panic!("unterminated C% literal, at line {}", self.line);
        };
        return Token::new_valc(*c, self.lexeme());
    }

    // called when `
    fn comment(&mut self) -> Token<'src>
    {
        self.advance(); // `
        while !self.matches::<0>(b'\n') {
            if self.is_at_end() {
                break;
            }
            self.advance();
        }
        return Token::new_comment(self.lexeme());
    }
}
