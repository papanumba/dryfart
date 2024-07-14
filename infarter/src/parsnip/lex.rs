/* parsnip/lex.rs */

use super::toki::{new_tok, Token, LnToken, TokTyp, PrimType};
use crate::{asterix, asterix::Val, util};

macro_rules! if_next {
    ($zelf:expr, $c:expr, $tt:ident) => {
        if $zelf.matches($c) {
            $zelf.advance();
            return Token::simple(TokTyp::$tt, $zelf.lexeme());
        }
    }
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

pub struct Luthor<'src>
{
    input: &'src [u8],
    line:     usize,  // current line
    base_pos: usize,  // first position from which trying to get a token
    next_pos: usize,  // final position of the trying current token
}

impl<'src> Luthor<'src>
{
    pub fn tokenize(s: &'src util::DfStr) -> Vec<LnToken<'src>>
    {
        let mut lxr = Self {
            input: s.as_bytes(),
            line: 1,
            base_pos: 0,
            next_pos: 0,
        };
        let mut res = vec![];
        while let Some(t) = lxr.next_token() {
            if t.as_comment().is_none() { // sþ oþer þan a comment
                res.push((t, lxr.line));
            }
        }
        res.push((Token::new_eof(), lxr.line));
        return res;
    }

    fn is_at_end(&self) -> bool
    {
        self.next_pos == self.input.len()
    }

    fn advance(&mut self)
    {
        if !self.is_at_end() {
            self.next_pos += 1;
        }
    }

    fn adv_while<COND>(&mut self, cond: COND)
    where COND: Fn(&u8) -> bool
    {
        while let Some(c) = self.peek() {
            if cond(&c) {
                self.advance();
            } else {
                 break;
            }
        }
    }

    // skips whitespaces and updates self.line when finding '\n'
    fn skip_whites(&mut self)
    {
        while let Some(w) = self.peek() {
            if !w.is_ascii_whitespace() {
                break;
            }
            if w == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
    }

    fn peek(&self) -> Option<u8>
    {
        self.peekn::<0>()
    }

    // LA: lookahead, 0 -> peek, 1 -> peek next
    fn peekn<const LA: usize>(&self) -> Option<u8>
    {
        self.input.get(self.next_pos + LA).copied()
    }

    fn lexeme(&self) -> &'src [u8]
    {
        &self.input[self.base_pos..self.next_pos]
    }

    fn matches(&self, m: u8) -> bool
    {
        self.peek().map(|c| c == m).unwrap_or(false)
    }

    fn read_char(&mut self) -> Option<u8>
    {
        let tmp = self.peek();
        self.advance();
        return tmp;
    }

    fn is_at_digit(&self) -> bool
    {
        self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }

    fn has_digit_next(&self) -> bool
    {
        self.peekn::<1>().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }

    fn next_token(&mut self) -> Option<Token<'src>>
    {
        self.skip_whites();
        self.base_pos = self.next_pos;
        let c = self.read_char()?;
        Some(match c {
            b'_' => lex_new_tok!(self, Uscore),
            b'.' => lex_new_tok!(self, Period),
            b',' => lex_new_tok!(self, Comma),
            b';' => lex_new_tok!(self, Semic),
            b'(' => lex_new_tok!(self, Lparen),
            b')' => lex_new_tok!(self, Rparen),
            b'{' => lex_new_tok!(self, Lbrace),
            b'}' => lex_new_tok!(self, Rbrace),
            b'+' | b'-' | b'*' | b'/' | b'@' | b'[' | b']' | b'^'
                 => self.maybe_2ble(c),
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

    // gets called when some of +-*/@[]^
    pub fn maybe_2ble(&mut self, c: u8) -> Token<'src>
    {
        if !self.matches(c) {
            return Token::try_1gle_from(self.lexeme()).unwrap();
        }
        self.advance();
        return Token::try_2ble_from(self.lexeme()).unwrap();
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
        b'\\'=> Bslash2,
        b'[' => BsLsb,
        b'#' => BsHash,
    }

    // $, $@[0-9]*
    fn from_dollar(&mut self) -> Token<'src>
    {
        if !self.matches(b'@') {
            return lex_new_tok!(self, Dollar);
        };
        let mut level = 0; // default level
        self.advance(); // @
        if self.is_at_digit() {
            self.adv_while(u8::is_ascii_digit);
            level = unsafe {
                std::str::from_utf8_unchecked(&self.lexeme()[2..])
                    .parse::<u32>()
                    .unwrap()
            };
        }
        return Token::new_rect(level, self.lexeme());
    }

    // gets called when at digit
    fn get_num(&mut self) -> Token<'src>
    {
        self.adv_while(u8::is_ascii_digit);
        if self.matches(b'U') || self.matches(b'u') {
            let n = Token::parse_valn(self.lexeme());
            self.advance(); // [Uu]
            return n;
        }
        // til here we'll have a "\d+" number
        // þen check weþr it's a R% "\d+\.\d+"
        if !(self.matches(b'.') && self.has_digit_next()) {
            return Token::parse_valz(self.lexeme());
        }
        self.advance(); // .
        self.adv_while(u8::is_ascii_digit);
        return Token::parse_valr(self.lexeme());
    }

    // gets called when at letter
    // result can be Token::{Ident, PrimType}
    fn get_ident(&mut self) -> Token<'src>
    {
        if let Some(pt) = self.try_prim_type() {
            self.advance();
            return Token::new_primtype(pt, self.lexeme());
        }
        self.adv_while(u8::is_ascii_alphanumeric);
        let lex = self.lexeme();
        return match lex {
            b"V" => new_tok!(ValV, lex),
            b"T" => Token::new_valb(true,  lex),
            b"F" => Token::new_valb(false, lex),
            _    => Token::new_ident(lex),
        };
    }

    // gets called when parsing an Ident
    // if þe current lexeme is a PrimType,
    // returns Some(PrimType) but does not advance()
    fn try_prim_type(&self) -> Option<PrimType>
    {
        if !self.matches(b'%') {
            return None;
        }
        let c = self.input[self.base_pos];
        return PrimType::try_from(&c).ok();
    }

    // called when '
    fn get_string(&mut self) -> Token<'src>
    {
        let mut ended_string = false;
        while let Some(c) = self.read_char() {
            if c == b'\'' {
                ended_string = true;
                break;
            }
            if c == asterix::ESC_CH && self.read_char().is_none() {
                panic!("expected escape char but found EOF at line {}",
                    self.line);
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
        if c == asterix::ESC_CH { // escapes
            let Some(d) = self.read_char() else {
                panic!("unterminated escaped C% at EOF");
            };
            let Ok(e) = Val::escape_char(d) else {
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
        return Token::new_valc(c, self.lexeme());
    }

    // called when `
    fn comment(&mut self) -> Token<'src>
    {
        self.advance(); // `
        while !self.matches(b'\n') && !self.is_at_end() {
            self.advance();
        }
        return Token::new_comment(self.lexeme());
    }
}
