/* src/parsnip/lex.rs */

use super::toki::{Token, PrimType};

macro_rules! if_next {
    ($this:ident, $c:expr, $tt:ident) => {
        if $this.matches::<0>($c) {
            $this.advance();
            return Token::$tt;
        }
    };
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
            match t {
                Token::Comment(_) => continue,
                _ => res.push((t, self.line)),
            }
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
        let c = match self.read_char() {
            Some(ch) => ch,
            None => return None,
        };
        Some(match c {
            b'_' => Token::Uscore,
            b'.' => Token::Period,
            b',' => Token::Comma,
            b';' => Token::Semic,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'^' => Token::Caret,
            b'+' | b'-' | b'*' | b'/' | b'@' | b'[' | b']' |
            b'&' | b'|' => self.maybe_2ble(*c),
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
            b'"' => self.get_string(),
            b'\'' => self.comment(),
             _ => Token::Unknown(*c),
        })
    }

    // gets called when some of +-*/&|~
    pub fn maybe_2ble(&mut self, c: u8) -> Token<'src>
    {
        if !self.is_at_2ble(c) {
            return Token::try_from(&c).unwrap();
        }
        if let Ok(t) = Token::try_2ble_from(c) {
            self.advance();
            return t;
        }
        panic!("not a double {0}{0}", char::from(c));
    }

    // $, $@[0-9]*
    #[inline]
    fn from_dollar(&mut self) -> Token<'src>
    {
        let Some(c) = self.peek::<0>() else {
            return Token::Dollar;
        };
        if *c != b'@' {
            return Token::Dollar;
        }
        if self.has_digit_next() {
            self.advance(); // @
            self.adv_while(|c| c.is_ascii_digit());
            let level = std::str::from_utf8(&self.lexeme()[2..])
                .unwrap().parse::<u32>().unwrap();
            return Token::RecT(level);
        } else { // default level
            self.advance(); // @
            return Token::RecT(0);
        }
    }

    // ~, ~~, ~=
    #[inline]
    fn from_tilde(&mut self) -> Token<'src>
    {
        if_next!(self, b'~', Tilde2);
        if_next!(self, b'=', Ne);
        return Token::Tilde;
    }

    // =, ==, =>
    #[inline]
    fn from_equal(&mut self) -> Token<'src>
    {
        if_next!(self, b'=', Equal2);
        if_next!(self, b'>', Then);
        return Token::Equal;
    }

    // <, <=
    #[inline]
    fn from_langle(&mut self) -> Token<'src>
    {
        if_next!(self, b'=', Le);
        return Token::Langle;
    }

    // >, >=
    #[inline]
    fn from_rangle(&mut self) -> Token<'src>
    {
        if_next!(self, b'=', Ge);
        return Token::Rangle;
    }

    // !, !!, !@, !$
    #[inline]
    fn from_bang(&mut self) -> Token<'src>
    {
        if_next!(self, b'!', Bang2);
        if_next!(self, b'@', RecP);
        if_next!(self, b'$', BangDollar);
        return Token::Bang;
    }

    // #, ##, #@, #$
    #[inline]
    fn from_hash(&mut self) -> Token<'src>
    {
        if_next!(self, b'#', Hash2);
        if_next!(self, b'@', RecF);
        if_next!(self, b'$', HashDollar);
        return Token::Hash;
    }

    // \, FUTURE: \\, \#, \[
    #[inline]
    fn from_bslash(&mut self) -> Token<'src>
    {
        return Token::Bslash;
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
                b'V' => return Token::ValV,
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

    // called when "
    fn get_string(&mut self) -> Token<'src>
    {
        let mut ended_string = false;
        while let Some(c) = self.peek::<0>() {
            match *c {
                b'"' => { ended_string = true; break; },
                b'`' => if self.peek::<1>().is_none() {
                    panic!("unterminated escape chars at line {}", self.line);
                } else {
                    self.advance(); self.advance();
                },
                _ => self.advance(),
            }
        }
        if !ended_string {
            panic!("unterminated string at line {}", self.line);
        }
        let raw = &self.lexeme()[1..];
        self.advance(); // skip final quote
        return Token::String(raw);
    }

    // called when '
    fn comment(&mut self) -> Token<'src>
    {
        self.advance(); // '
        while !self.matches::<0>(b'\n') {
            if self.is_at_end() {
                break;
            }
            self.advance();
        }
        return Token::Comment(self.lexeme());
    }
}
