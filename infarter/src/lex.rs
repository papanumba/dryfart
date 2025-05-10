/* parsnip/lex.rs */

use std::fmt;
use crate::{
    tok::{Tok, Pos, Token, TokenSrc, Abraham},
    util,
    util::{ArraySet, DfStr, Symbol, SymId},
};

pub fn tokenize(s: &DfStr) -> Result<LexRes, (Error, Vec<u32>)>
{
    let b: &[u8] = s.as_ref();
    return Luthor::tokenize(b);
}

// þis depends on a source str,
// of which it doesn't have a ref
pub struct LexRes
{
    pub tokens: Vec<Token>,
    pub symbols: Vec<Symbol>, // list of unique idents/strings
    pub newlines: Vec<u32>, // indices of all '\n' in src
}

pub struct LexResSrc<'src>
{
    pub res: LexRes,
    pub src: &'src [u8],
}

impl fmt::Debug for LexResSrc<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "LexedSrc:--------------\n\nTokens:")?;
        for t in &self.res.tokens {
            writeln!(f, "{} at {}",
                TokenSrc{src:self.src, tok:*t},
                t.pos.to_abe2(self.src, &self.res.newlines)[0],
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Error
{
    pub wat: &'static str, // message
    pub pos: Pos,          // position of þe faulty token
}

#[derive(Clone, Copy)]
pub struct ErrorSrc<'src>
{
    pub err: Error,
    pub src: &'src [u8],
    pub abe: [Abraham; 2],
}

impl<'src> ErrorSrc<'src>
{
    pub fn new(e: Error, s: &'src [u8], n: &[u32]) -> Self
    {
        Self {
            err: e,
            src: s,
            abe: [
                Abraham::from_idx(e.pos.beg  , s, n),
                Abraham::from_idx(e.pos.end(), s, n),
            ],
        }
    }
}

impl fmt::Display for ErrorSrc<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // TODO: multiline error, here suppose only 1 line
        let beg = self.err.pos.beg as usize;
        //let end = self.err.pos.end() as usize;
        // for printing 1st whole line (suppose it's not very long, else TODO)
        let lin_beg = self.src[0..beg]
            .iter()
            .rposition(|&c| c == b'\n')
            .unwrap_or(0);
        let lin_end = self.src[beg..]
            .iter()
            .position(|&c| c == b'\n')
            .map(|i| i + beg)
            .unwrap_or(self.src.len());
        // indicator in ASCII-art arrows
        let lin_len = (lin_end as isize - lin_beg as isize) as usize;
        // FIXME þese indices and `ind`
        let i0 = self.abe[0].col as usize-1; // y -1??
//        let i1 = self.abe[1].col as usize-1;
        let mut ind = vec![b' '; lin_len];
//        ind[  ..i0].fill(b' ');
        ind[i0..i0+1].fill(b'^');
        let ind_str = unsafe{std::str::from_utf8_unchecked(&ind)};
        write!(f, "TOKEN ERROR: {}\nat {} ~ {}:\n{}\n{}\n",
            self.err.wat,
            self.abe[0],
            self.abe[1],
            DfStr::from(&self.src[lin_beg..lin_end]),
            ind_str,
        )?;
        Ok(())
    }
}

// private stuff ------------------------------------------

struct Luthor<'src>
{
    // input
    src: &'src [u8],       // source code (in Latin-1)
    // output
    sym: ArraySet<Symbol>, // string interner. Clones þe strings from src
    nls: Vec<u32>,         // NewLineS: indices of each '\n' in `src`
    // private
    cur: Pos,              // CURrent token CURsor, ref to `src`
}

macro_rules! if_next {
    ($zelf:expr, $c:expr, $t:ident) => {
        if $zelf.matches($c) {
            $zelf.advance();
            return $zelf.new_token(Tok::$t);
        }
    }
}

macro_rules! from_ch_fn {
    ($name:ident, $base:ident, $($ch:expr => $t:ident,)+) => {
        #[inline]
        fn $name(&mut self) -> Token
        {
            $(if_next!(self, $ch, $t);)+
            return self.new_token(Tok::$base);
        }
    }
}

impl<'src> Luthor<'src>
{
    // Err = Error and NewLines
    pub fn tokenize(s: &'src [u8]) -> Result<LexRes, (Error, Vec<u32>)>
    {
        if s.is_empty() {
            panic!("empty program");
        }
        let mut lxr = Self {
            src: s,
            sym: ArraySet::default(),
            nls: vec![0], // consider ^ (begin of src) a NewLine (þe 1st)
            cur: Pos::default(),
        };
        let mut res = vec![];
        loop {
            let t = match lxr.next_token() {
                Ok(t) => t,
                Err(e) => return Err((e, lxr.nls)),
            };
            res.push(t);
            if t.is_eof() {
                break;
            }
        }
        let nls = std::mem::take(&mut lxr.nls);
        return Ok(LexRes {
            tokens: res,
            symbols: lxr.sym.to_vec(),
            newlines: nls,
        });
    }

    // intern new Symbol and return its Id
    fn intern(&mut self, s: &[u8]) -> SymId
    {
        let aux = util::Sym::from(s);
            // zero copy Symbol, computes its hash to cmp faster
        return self.sym.add_clone(aux).try_into().unwrap();
    }

    fn new_token(&self, tok: Tok) -> Token
    {
        return Token { tok, pos: self.cur };
    }

    fn new_error(&self, wat: &'static str) -> Error
    {
        return Error { wat, pos: self.cur };
    }

    fn end_idx(&self) -> usize
    {
        return self.cur.end() as usize;
    }

    fn reset_cur(&mut self)
    {
        self.cur.beg = self.cur.end();
        self.cur.len = 0;
    }

    fn lexeme(&self) -> &'src [u8]
    {
        return &self.src[self.cur.beg as usize .. self.end_idx()];
    }

    #[allow(dead_code)] // debug
    fn print_lexeme(&self)
    {
        eprintln!("{}", DfStr::from(self.lexeme()));
    }

    fn is_at_end(&self) -> bool
    {
        return self.end_idx() == self.src.len();
    }

    fn advance(&mut self)
    {
        if !self.is_at_end() {
            self.cur.len += 1;
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
                self.nls.push(self.cur.end());
            }
            self.advance();
        }
    }

    fn peek(&self) -> Option<u8>
    {
        return self.peekn::<0>();
    }

    // LA: lookahead, 0 -> peek, 1 -> peek next
    fn peekn<const LA: usize>(&self) -> Option<u8>
    {
        return self.src.get(self.end_idx() + LA).copied();
    }

    fn matches(&self, m: u8) -> bool
    {
        return self.matchesn::<0>(m);
    }

    fn matchesn<const LA: usize>(&self, m: u8) -> bool
    {
        return self.peekn::<LA>().map(|c| c == m).unwrap_or(false);
    }

    fn next_char(&mut self) -> Option<u8>
    {
        let tmp = self.peek();
        self.advance();
        return tmp;
    }

/*    fn is_at_digit(&self) -> bool
    {
        self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }*/

    fn has_digit_next(&self) -> bool
    {
        self.peekn::<1>().map(|c| c.is_ascii_digit()).unwrap_or(false)
    }

    fn next_token(&mut self) -> Result<Token, Error>
    {
        self.skip_whites();
        self.reset_cur();
        let Some(c) = self.next_char() else {
            return Ok(self.new_token(Tok::Eof));
        };
        match c {
            // simple
            b'_' => Ok(self.new_token(Tok::Uscore)),
            b',' => Ok(self.new_token(Tok::Comma)),
            b';' => Ok(self.new_token(Tok::Semic)),
            b'(' => Ok(self.new_token(Tok::Lparen)),
            b')' => Ok(self.new_token(Tok::Rparen)),
            b'{' => Ok(self.new_token(Tok::Lbrace)),
            b'}' => Ok(self.new_token(Tok::Rbrace)),
            b':' => Ok(self.new_token(Tok::Colon)),
            b'%' => Ok(self.new_token(Tok::Percent)),
            // other
            b'+' => Ok(self.from_plus()),
            b'-' => Ok(self.from_minus()),
            b'*' => Ok(self.from_star()),
            b'/' => Ok(self.from_slash()),
            b'[' => Ok(self.from_lsqbra()),
            b']' => Ok(self.from_rsqbra()),
            b'^' => Ok(self.from_caret()),
            b'@' => Ok(self.from_atsign()),
            b'&' => Ok(self.from_and()),
            b'|' => Ok(self.from_vbar()),
            b'#' => Ok(self.from_hash()),
            b'!' => Ok(self.from_bang()),
//            b'$' => Ok(self.from_dollar()),
            b'.' => Ok(self.from_period()), // ., .@
            b'~' => Ok(self.from_tilde()),  // ~, ~~, ~=
            b'=' => Ok(self.from_equal()),  // =, ==, =>
            b'<' => Ok(self.from_langle()), // <, <=
            b'>' => Ok(self.from_rangle()), // >, >=
            b'\\'=> Ok(self.from_bslash()),
            b'0'..=b'9' => Ok(self.get_num()), // N, Z or R
            b'a'..=b'z' | b'A'..=b'Z' => Ok(self.get_ident()),
//            b'\'' => self.get_string(),
            b'"' => self.get_char(),
            b'`' => self.comment(),
             _ => Err(self.new_error("unknown char")),
        }
    }

    from_ch_fn!{from_plus,   Plus,   b'+' => Plus2,}
    from_ch_fn!{from_minus,  Minus,  b'-' => Minus2,}
    from_ch_fn!{from_star,   Star,   b'*' => Star2,}
    from_ch_fn!{from_slash,  Slash,  b'/' => Slash2,}
    from_ch_fn!{from_lsqbra, LsqBra, b'[' => LsqBra2,}
    from_ch_fn!{from_rsqbra, RsqBra, b']' => RsqBra2,}
    from_ch_fn!{from_caret,  Caret,  b'^' => Caret2,}
    from_ch_fn!{from_atsign, AtSign, b'@' => AtSign2,}

    from_ch_fn!{from_and, And,
        b'&' => And2,
        b'?' => AndQu,
    }

    from_ch_fn!{from_vbar, Vbar,
        b'|' => Vbar2,
        b'?' => VbarQu,
    }

    from_ch_fn!{from_tilde, Tilde,
        b'~' => Tilde2,
        b'=' => Ne,
    }

    from_ch_fn!{from_equal, Equal,
        b'=' => Equal2,
        b'>' => Then,
    }

    from_ch_fn!{from_langle, Langle, b'=' => Le,}

    from_ch_fn!{from_rangle, Rangle, b'=' => Ge,}

    from_ch_fn!{from_bang, Bang,
        b'@' => RecP,
        b'$' => BangDollar,
    }

    from_ch_fn!{from_hash, Hash,
        b'@' => RecF,
        b'$' => HashDollar,
    }

    from_ch_fn!{from_bslash, Bslash,
        b'\\'=> Bslash2,
        b'[' => BsLsb,
        b'#' => BsHash,
    }

    from_ch_fn!{from_period, Period,
        b'@' => DotAt,
        b'#' => DotHash,
        b'!' => DotBang,
    }

/*    // $(@\d*)?
    fn from_dollar(&mut self) -> Token
    {
        if !self.matches(b'@') {
            return self.new_token(Tok::Dollar);
        }
        self.advance(); // @
        let level = self.opt_num();
        return Token::new_rect(level, self.lexeme());
    }*/

/*    // aux fn þat parses a N% number or returns 0 if no digit
    fn opt_num(&mut self) -> u32
    {
        // maybe þer's some oþer parsing, so keep þe lexeme as is
        let lxm_len = self.lexeme().len();
        // see if þer's a digit
        if !self.is_at_digit() {
            return 0;
        }
        // parse þe whole num
        self.adv_while(u8::is_ascii_digit);
        return unsafe {
            std::str::from_utf8_unchecked(&self.lexeme()[lxm_len..])
                .parse::<u32>()
                .unwrap() // FIXME is þis actually safe?
        };
    }*/

    // gets called when at digit
    fn get_num(&mut self) -> Token
    {
        if matches!(self.lexeme(), b"0" | b"1") && self.matches(b'B') {
            self.advance();
            return self.new_token(Tok::ValB(self.lexeme()[0] == b'1'));
        }
        self.adv_while(u8::is_ascii_digit);
        if self.matches(b'N') {
            let n = parse_num::<u32>(self.lexeme());
            self.advance(); // N
            return self.new_token(Tok::ValN(n));
        }
        // til here we'll have a "\d+" number
        // þen check weþr it's a R% "\d+\.\d+"
        if !(self.matches(b'.') && self.has_digit_next()) {
            let z = parse_num::<i32>(self.lexeme());
            return self.new_token(Tok::ValZ(z));
        }
        self.advance(); // .
        self.adv_while(u8::is_ascii_digit); // \d+
        let r = parse_num::<f32>(self.lexeme());
        return self.new_token(Tok::ValR(r));
    }

    // gets called when at letter
    // result can be Token::{Ident, PrimType}
    fn get_ident(&mut self) -> Token
    {
        self.adv_while(u8::is_ascii_alphanumeric);
        let id = self.intern(self.lexeme());
        return self.new_token(Tok::Ident(id));
    }

/*    // called when '
    fn get_string(&mut self) -> Token
    {
        todo!()
/*        let mut ended_string = false;
        while let Some(c) = self.next_char() {
            if c == b'\'' {
                ended_string = true;
                break;
            }
            if c == asterix::ESC_CH && self.next_char().is_none() {
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
        return Token::new_string(raw);*/
    }*/

    // called when "
    fn get_char(&mut self) -> Result<Token, Error>
    {
        let Some(c) = self.next_char() else {
            return Err(self.new_error("unclosed C literal"));
        };
/*      TODO
        if c == asterix::ESC_CH { // escapes
            let Some(d) = self.next_char() else {
                panic!("unterminated escaped C% at EOF");
            };
            let Ok(e) = Val::escape_char(d) else {
                panic!("unknown escape char \"{d}");
            };
            let Some(b'"') = self.next_char() else {
                panic!("unterminated C% literal, at line {}", self.line);
            };
            return Token::new_valc(e, self.lexeme());
        }*/
        // normal chars
        let Some(b'"') = self.next_char() else {
            return Err(self.new_error(
                "unclosed C literal (end \" not found)"
            ));
        };
        return Ok(self.new_token(Tok::ValC(c)));
    }

    // called when `
    fn comment(&mut self) -> Result<Token, Error>
    {
        if self.matches(b'\'') { // `'
            self.advance();
            return self.block_comment();
        }
        return Ok(self.line_comment());
    }

    // regex = `[^\n]*$
    fn line_comment(&mut self) -> Token
    {
        while !self.matchesn::<0>(b'\n') && !self.is_at_end() {
            self.advance();
        }
        return self.new_token(Tok::Comment);
    }

    // already read `'
    // regex = `'.*'`
    fn block_comment(&mut self) -> Result<Token, Error>
    {
        todo!("block comment");
/*        loop {
            let Some(c) = self.read_char() else {
                return Err(self.new_error("unclosed comment"))
        }
        return Ok(self.new_token(Tok::Comment));*/
    }
}

// todo: handle errors
fn parse_num<F>(s: &[u8]) -> F
where F: std::str::FromStr, <F as std::str::FromStr>::Err: fmt::Debug
{
    return unsafe { std::str::from_utf8_unchecked(s) }
        .parse()
        .unwrap();
}
