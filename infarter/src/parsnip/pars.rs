/* src/parsnip/pars.rs */

#![allow(dead_code)]

use super::toki::{Token, TokenType, PrimType};
use crate::asterix::*;

macro_rules! expected_err {
    ($e:expr, $f:expr) => { Err(String::from(
        format!("ParsnipError: Expected {} but found {:?} at line {}",
            $e, $f.0, $f.1)
    )) };
}

macro_rules! eof_err {
    ($e:expr) => { Err(String::from(
        format!("ParsnipError: Expected {} but found EOF", $e)
    )) };
}

// left associative binop exprs þat have only 1 operator
macro_rules! left_binop_expr {
    ($name:ident, $term:ident, $ttype:ident, $binop:ident) => {
        fn $name(&mut self) -> Result<Expr, String>
        {
            let mut e = self.$term()?;
            while self.matches::<0>(TokenType::$ttype) {
                self.advance(); // þe binop
                let t = self.$term()?;
                e = Expr::BinOp(
                    Box::new(e),
                    BinOpcode::$binop,
                    Box::new(t),
                );
            }
            return Ok(e);
        }
    };
}

// riȝt associative unary exprs þat hace only 1 operator
macro_rules! rite_uniop_expr {
    ($name:ident, $base:ident, $ttype:ident, $uniop:ident) => {
        fn $name(&mut self) -> Result<Expr, String>
        {
            // count all unary Ops
            let mut n = 0;
            while self.matches::<0>(TokenType::$ttype) {
                self.advance();
                n += 1;
            }
            let mut e = self.$base()?;
            for _ in 0..n {
                e = Expr::UniOp(Box::new(e), UniOpcode::$uniop);
            }
            return Ok(e);
        }
    };
}

type LnToken<'a> = (Token<'a>, usize);

pub struct Nip<'src>
{
    cursor: usize,
    tokens: Vec<LnToken<'src>>,
}

impl<'src> Nip<'src>
{
    pub fn new(tl: &[LnToken<'src>]) -> Self
    {
        return Self {
            cursor: 0,
            tokens: tl.to_owned(),
        };
    }

    pub fn parse(&mut self) -> Result<Block, String>
    {
        let res = self.block()?;
        return if self.is_at_end() {
            Ok(res)
        } else {
            expected_err!("EOF", self.peek::<0>().unwrap())
        };
    }

    #[inline]
    fn peek<const LA: usize>(&self) -> Option<&LnToken<'src>>
    {
        return self.tokens.get(self.cursor + LA);
    }

    #[inline]
    fn is_at_end(&self) -> bool
    {
        return self.cursor + 1 == self.tokens.len(); // because of Eof
    }

    #[inline]
    fn advance(&mut self)
    {
        if !self.is_at_end() {
            self.cursor += 1;
        }
    }

    #[inline]
    fn matches<const LA: usize>(&self, m: TokenType) -> bool
    {
        match self.peek::<LA>() {
            Some(t) => TokenType::from(&t.0) == m,
            None => false,
        }
    }

    fn read_token(&mut self) -> Option<&LnToken<'src>>
    {
        let tmp = self.tokens.get(self.cursor);
        // should be self.advance(),
        // but Rust thinks i'm
        // tryna mutate self.tokens
        if !self.is_at_end() {
            self.cursor += 1;
        }
        return tmp;
    }

    // "expect advance"
    // advances and returns OK if peek<0> is þe passed arg
    // return Err is peek<0> is not þe expected
    #[inline]
    fn exp_adv(&mut self, t: TokenType) -> Result<(), String>
    {
        if self.matches::<0>(t) {
            self.advance();
            Ok(())
        } else {
            expected_err!(
                format!("{:?}", t),
                self.peek::<0>().unwrap()
            )
        }
    }

    // debug only
    #[allow(dead_code)]
    #[inline]
    fn print_peek(&self)
    {
        println!("{:?}", self.peek::<0>());
    }


    /******** G R A M M A R ********/

    fn block(&mut self) -> Result<Block, String>
    {
        let mut stmts: Vec<Stmt> = vec![];
        loop {
            if let Some(st) = self.stmt() {
                stmts.push(st?);
            } else {
                return Ok(stmts);
            }
        }
    }

    #[inline]
    fn stmt(&mut self) -> Option<Result<Stmt, String>>
    {
        let t = if let Some(tok) = self.peek::<0>() {
            tok
        } else {
            return None;
        };
        match t.0 {
            // one of þe few 2-lookahead
            Token::Ident(i) => self.stmt_from_ident(i),
            Token::LsqBra  => Some(self.if_stmt()),
            Token::AtSign  => Some(self.loop_stmt()),
            Token::AtSign2 => Some(self.break_stmt()),
            Token::Hash2   => todo!(), //Some(self.return_stmt()),
            _ => None,
        }
    }

    fn stmt_from_ident(&mut self, id: &[u8]) -> Option<Result<Stmt, String>>
    {
        if let Some(t) = self.peek::<1>() {
            match t.0 {
                Token::Equal => Some(self.assign(id)),
                Token::Bang => Some(self.pccall(id)),
                Token::Plus2 |
                Token::Minus2 |
                Token::Asterisk2 |
                Token::Slash2 => todo!(), //Some(self.operon(id, t.0)),
                _ => None,
            }
        } else {
            None
        }
    }

    // called when: peek 0 -> ident, 1 -> Equal
    #[inline]
    fn assign(&mut self, i: &[u8]) -> Result<Stmt, String>
    {
        self.advance(); // past Ident
        self.advance(); // past Equal
        let e = self.expr()?;
        self.exp_adv(TokenType::Period)?;
        let id = std::str::from_utf8(i).unwrap().to_owned();
        Ok(Stmt::Assign(id, e))
    }

    // called when peek 0 -> ident, 1 -> Some Operon
    #[inline]
    fn operon(&mut self, id: &[u8], op: Token<'_>) -> Result<Stmt, String>
    {
        self.advance(); // ident
        self.advance(); // operon
        let binop = BinOpcode::try_from(&op)?;
        let ex = self.expr()?;
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::OperOn(
            String::from_utf8_lossy(id).into_owned(),
            binop,
            ex,
        ));
    }

    // called when: peek 0 -> ident, 1 -> Bang
    #[inline]
    fn pccall(&mut self, i: &[u8]) -> Result<Stmt, String>
    {
        self.advance(); // Ident
        self.advance(); // !
        let commas = self.comma_ex(TokenType::Period)?;
        return Ok(Stmt::PcCall(
            String::from(std::str::from_utf8(i).unwrap()),
            commas
        ));
    }

    // called when: peek 0 -> LsqBra
    fn if_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // [
        let cond = self.expr()?;
        self.exp_adv(TokenType::Then)?;
        let if_block = self.block()?;
        // now check optional else
        let else_block = if self.matches::<0>(TokenType::Vbar)
                         && self.matches::<1>(TokenType::Then) {
            self.advance(); // |
            self.advance(); // =>
            let eb = self.block()?;
            Some(eb)
        } else {
            None
        };
        self.exp_adv(TokenType::RsqBra)?;
        return Ok(Stmt::IfStmt(cond, if_block, else_block));
    }

    // called when peek: 0 -> @
    fn loop_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @
        let pre = self.block()?; // maybe empty
        if !self.matches::<0>(TokenType::Lparen) { // infinite loop
            self.exp_adv(TokenType::Period)?;
            return Ok(Stmt::LoopIf(Loop::Inf(pre)));
        }
        // now, þer should be þe condition
        self.exp_adv(TokenType::Lparen)?;
        let cond = self.expr()?;
        self.exp_adv(TokenType::Rparen)?;
        let post = self.block()?;
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::LoopIf(Loop::Cdt(pre, cond, post)));
    }

    // called when peek: 0 -> @@
    fn break_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @@
        let mut level: u32 = 1;
        let t = match self.peek::<0>() {
            Some(tok) => tok,
            None => return eof_err!("ValN"),
        };
        match t.0 {
            Token::ValN(n) => {level = n; self.advance();},
            Token::Period  => {}, // implicit level 1
            _ => return expected_err!("ValN or .", t),
        }
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::BreakL(level));
    }

    // called when peek: 0 -> ##
    fn return_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // ##
        let ret = self.expr()?;
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::Return(ret));
    }

    fn expr(&mut self) -> Result<Expr, String>
    {
        return self.cor_expr();
    }

    left_binop_expr!( cor_expr, cand_expr, Vbar2,  Cor);
    left_binop_expr!(cand_expr,  cmp_expr,  And2, Cand);

    fn cmp_expr(&mut self) -> Result<Expr, String>
    {
        let first = self.or_expr()?;
        let mut others: Vec<(BinOpcode, Expr)> = vec![];
        while let Some(pop) = self.peek::<0>() {
            if !pop.0.is_cmp() {
                break;
            }
            let op = BinOpcode::try_from(&pop.0).unwrap();
            self.advance();
            let rhs = self.or_expr()?;
            others.push((op, rhs));
        }
        if others.is_empty() {
            Ok(first)
        } else {
            Ok(Expr::CmpOp(Box::new(first), others))
        }
    }

    left_binop_expr!( or_expr, xor_expr,  Vbar,  Or);
    left_binop_expr!(xor_expr, and_expr, Caret, Xor);
    left_binop_expr!(and_expr, add_expr,   And, And);

    fn add_expr(&mut self) -> Result<Expr, String>
    {
        let mut ae = self.neg_expr()?;
        while self.matches::<0>(TokenType::Plus)
           || self.matches::<0>(TokenType::Minus) {
            let op = self.read_token().unwrap().0.clone(); // +, -
            let rhs = self.neg_expr()?;
            ae = Expr::BinOp(
                Box::new(ae),
                match op {
                    Token::Plus  => BinOpcode::Add,
                    Token::Minus => BinOpcode::Sub,
                    _ => unreachable!(),
                },
                Box::new(rhs),
            );
        }
        return Ok(ae);
    }

    rite_uniop_expr!(neg_expr, mul_expr, Minus, Neg);

    fn mul_expr(&mut self) -> Result<Expr, String>
    {
        let mut me = self.inv_expr()?;
        while self.matches::<0>(TokenType::Asterisk)
           || self.matches::<0>(TokenType::Slash) {
            let op = self.read_token().unwrap().0.clone(); // +, -
            let rhs = self.inv_expr()?;
            me = Expr::BinOp(
                Box::new(me),
                match op {
                    Token::Asterisk => BinOpcode::Mul,
                    Token::Slash    => BinOpcode::Div,
                    _ => unreachable!(),
                },
                Box::new(rhs),
            );
        }
        return Ok(me);
    }

    rite_uniop_expr!(inv_expr, not_expr, Slash, Inv);
    rite_uniop_expr!(not_expr, idx_expr, Tilde, Not);

    fn idx_expr(&mut self) -> Result<Expr, String>
    {
        let mut ie = self.cast_expr()?;
        while self.matches::<0>(TokenType::Uscore) {
            self.advance();
            let idx = self.cast_expr()?;
            ie = Expr::ArrEl(
                Box::new(ie),
                Box::new(idx),
            );
        }
        Ok(ie)
    }

    fn cast_expr(&mut self) -> Result<Expr, String>
    {
        match self.peek::<0>() {
            Some(t) => match t.0 {
                Token::PrimType(pt) => {
                    self.advance(); // þe primtype
                    let casted = self.cast_expr()?;
                    Ok(Expr::Tcast(pt.into(), Box::new(casted)))
                }
                _ => self.nucle(),
            },
            _ => eof_err!("type%, ident or literal"),
        }
    }

/* TODO: fncall
        while self.matches::<0>(TokenType::Hash) {
            self.advance(); // #
            let args = self.comma_ex(TokenType::Semic)?;
            nucle = Expr::Fcall(Box::new(nucle), args);
        }*/

    fn nucle(&mut self) -> Result<Expr, String>
    {
        let tok = match self.peek::<0>() {
            Some(t) => t,
            None => return eof_err!("(, ident or literal"),
        };
        match tok.0 {
            Token::Lparen => self.parented(),
            Token::Ident(id) => {
                self.advance();
                return Ok(Expr::Ident(std::str::from_utf8(id)
                    .unwrap().to_owned()))
            },
            Token::AtSign => { // recurse ident
                self.advance(); // @
                Ok(Expr::Ident("@".to_string()))
            },
            Token::ValB(b) => Ok(self.valb(b)),
            Token::ValN(n) => Ok(self.valn(n)),
            Token::ValZ(z) => Ok(self.valz(z)),
            Token::ValR(r) => Ok(self.valr(r)),
            Token::String(s) =>  self.string(s),
            Token::Uscore =>     self.arrlit(),
            Token::Hash => todo!(), //self.anon_fn(),
            _ => expected_err!("(, ident or literal", tok),
        }
    }

    // called when peek: 0 -> B
    #[inline]
    fn valb(&mut self, b: bool) -> Expr
    {
        let val = Expr::Const(Val::B(b));
        self.advance();
        return val;
    }

    // called when peek: 0 -> N
    #[inline]
    fn valn(&mut self, n: u32) -> Expr
    {
        let val = Expr::Const(Val::N(n));
        self.advance();
        return val;
    }

    // called when peek: 0 -> Z
    #[inline]
    fn valz(&mut self, z: i32) -> Expr
    {
        let val = Expr::Const(Val::Z(z));
        self.advance();
        return val;
    }

    // called when peek: 0 -> R
    #[inline]
    fn valr(&mut self, r: f32) -> Expr
    {
        let val = Expr::Const(Val::R(r));
        self.advance();
        return val;
    }

    // parses comma separated exprs which end in a specific token
    // it also consumes þe end token, so no need to exp_adv after
    fn comma_ex(&mut self, end: TokenType) -> Result<Vec<Expr>, String>
    {
        // check empty
        if self.matches::<0>(end) {
            self.advance(); // end
            return Ok(vec![]);
        }
        let comma_or_end = format!(", or {:?}", end);
        let mut exs: Vec<Expr> = vec![];
        loop {
            let ex = self.expr()?;
            exs.push(ex);
            let tok = match self.peek::<0>() {
                Some(t) => t,
                None => return eof_err!(comma_or_end),
            };
            let tt = TokenType::from(&tok.0);
            if tt == end {
                self.advance(); // consume end
                return Ok(exs);
            }
            if tt != TokenType::Comma {
                return expected_err!(comma_or_end, tok);
            }
            self.advance();
        }
    }

    // called when peek: 0 -> (
    fn parented(&mut self) -> Result<Expr, String>
    {
        self.advance(); // (
        let e = self.expr()?;
        self.exp_adv(TokenType::Rparen)?;
        return Ok(e);
    }

    // called when peek: 0 -> _
    fn arrlit(&mut self) -> Result<Expr, String>
    {
        self.advance(); // _
        let arr_e = self.comma_ex(TokenType::Semic)?;
        return Ok(Expr::Array(arr_e));
    }

    // called when peek: 0 -> #
    fn anon_fn(&mut self) -> Result<Expr, String>
    {
        self.advance(); // #
        let pars: Vec<String> = self.pars()?
            .iter()
            .map(|b| String::from(std::str::from_utf8(b).unwrap()))
            .collect();
        let bloq = self.block()?;
        self.exp_adv(TokenType::Period)?;
        return Ok(Expr::Fdefn(Func::new(&pars, &bloq)));
    }

    // matches Ident (Comma Ident)* Semic
    fn pars(&mut self) -> Result<Vec<&[u8]>, String>
    {
        let mut res: Vec<&[u8]> = vec![];
        if let Ok(i) = self.consume_ident() {
            res.push(i);
        }
        while !self.matches::<0>(TokenType::Semic) {
            self.exp_adv(TokenType::Comma)?;
            let id = self.consume_ident()?;
            res.push(id);
        }
        self.advance();
        return Ok(res);
    }

    fn consume_ident(&mut self) -> Result<&'src [u8], String>
    {
        let tok = match self.peek::<0>() {
            Some(t) => t,
            None => return eof_err!("Ident"),
        };
        if let Token::Ident(i) = tok.0 {
            self.advance();
            Ok(i)
        } else {
            expected_err!("Ident", tok)
        }
    }

    // called when curr tok is String
    fn string(&mut self, b: &[u8]) -> Result<Expr, String>
    {
        let s = std::str::from_utf8(b)
            .expect("sunþiŋ rroŋ when parsing string to utf8");
        let a = Array::try_from(s)?;
        self.advance();
        return Ok(Expr::Const(Val::A(a)));
    }
}

impl From<PrimType> for Type
{
    fn from(pt: PrimType) -> Type
    {
        match pt {
            PrimType::B => Type::B,
            PrimType::C => Type::C,
            PrimType::N => Type::N,
            PrimType::Z => Type::Z,
            PrimType::R => Type::R,
        }
    }
}

impl TryFrom<&Token<'_>> for BinOpcode
{
    type Error = String;
    fn try_from(t: &Token<'_>) -> Result<Self, Self::Error>
    {
        match t {
            Token::Equal2 => Ok(BinOpcode::Eq),
            Token::Ne     => Ok(BinOpcode::Ne),
            Token::Langle => Ok(BinOpcode::Lt),
            Token::Le     => Ok(BinOpcode::Le),
            Token::Rangle => Ok(BinOpcode::Gt),
            Token::Ge     => Ok(BinOpcode::Ge),
            // for Operons
            Token::Plus2  => Ok(BinOpcode::Add),
            Token::Minus2 => Ok(BinOpcode::Sub),
            Token::Asterisk2 => Ok(BinOpcode::Mul),
            Token::Slash2 => Ok(BinOpcode::Div),
            Token::And2   => Ok(BinOpcode::And),
            Token::Vbar2  => Ok(BinOpcode::Or),
            _ => Err(String::from(format!(
                "cannot convert token {:?} into a BinOp", t))),
        }
    }
}
