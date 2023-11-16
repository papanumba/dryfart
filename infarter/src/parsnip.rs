/* src/parsnip.rs */

//#![allow(dead_code)]

use crate::luthor::{Token, TokenType, PrimType};
use crate::asterix::*;
use crate::twalker::*;

macro_rules! expected_err {
    ($e:expr, $f:expr) => { Err(String::from(
        format!("ParsnipError: Expected {} but found {:?} at line {}",
            $e, $f.0, $f.1)
    )) };
}


pub fn parse<'src>(
    tokens: &[(Token<'src>, usize)]
) -> Result<Stmt, String>
{
    let mut p = Parsnip::new(tokens);
    return p.parse();
}


struct Parsnip<'src>
{
    cursor: usize,
    tokens: Vec<(Token<'src>, usize)>,
}

impl<'src> Parsnip<'src>
{
    pub fn new(tl: &[(Token<'src>, usize)]) -> Self
    {
        return Self {
            cursor: 0,
            tokens: tl.to_owned(),
        };
    }

    pub fn parse(&mut self) -> Result<Stmt, String>
    {
        let res = self.stmt();
        return if self.is_at_end() {
            res
        } else {
            expected_err!("EOF", self.peek::<0>().unwrap())
        };
    }

    #[inline]
    fn peek<const LA: usize>(&self) -> Option<&(Token<'src>, usize)>
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

    fn read_token(&mut self) -> Option<&(Token<'src>, usize)>
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


    /******** G R A M M A R ********/

    fn stmt(&mut self) -> Result<Stmt, String>
    {
        if self.matches::<0>(TokenType::Ident) {
            // one of þe few 2-lookaheads
            if self.matches::<1>(TokenType::Equal) {
                return self.assign();
            } else if self.matches::<1>(TokenType::Bang) {
                return self.pccall();
            }
        }
        todo!();
    }

    // called when: peek 0 -> ident, 1 -> Equal
    fn assign(&mut self) -> Result<Stmt, String>
    {
        let i = match self.peek::<0>().unwrap().0 {
            Token::Ident(ii) => ii,
            _ => unreachable!(),
        };
        self.advance(); // past Ident
        self.advance(); // past Equal
        let e = self.expr()?;
        if self.matches::<0>(TokenType::Period) {
            self.advance();
            let id = std::str::from_utf8(i).unwrap().to_owned();
            Ok(Stmt::Assign(id, e))
        } else {
            expected_err!(".", self.peek::<0>().unwrap())
        }
    }

    // called when: peek 0 -> ident, 1 -> Bang
    fn pccall(&mut self) -> Result<Stmt, String>
    {
        let i = match self.peek::<0>().unwrap().0 {
            Token::Ident(ii) => ii,
            _ => unreachable!(),
        };
        self.advance(); // Ident
        self.advance(); // !
        let commas = self.comma_ex(TokenType::Period)?;
        return Ok(Stmt::PcCall(
            String::from(std::str::from_utf8(i).unwrap()),
            commas
        ));
    }

    fn expr(&mut self) -> Result<Expr, String>
    {
        return self.or_expr();
    }

    fn or_expr(&mut self) -> Result<Expr, String>
    {
        let mut oe = self.and_expr()?;
        while self.matches::<0>(TokenType::Vbar) {
            self.advance();
            let rhs = self.and_expr()?;
            oe = Expr::BinOp(
                Box::new(oe),
                BinOpcode::Or,
                Box::new(rhs),
            );
        }
        Ok(oe)
    }

    fn and_expr(&mut self) -> Result<Expr, String>
    {
        let mut ae = self.and_term()?;
        while self.matches::<0>(TokenType::And) {
            self.advance();
            let rhs = self.and_term()?;
            ae = Expr::BinOp(
                Box::new(ae),
                BinOpcode::And,
                Box::new(rhs),
            );
        }
        Ok(ae)
    }

    fn and_term(&mut self) -> Result<Expr, String>
    {
        // count all unary Negations before CmpExpr
        let mut n = 0;
        while self.matches::<0>(TokenType::Tilde) {
            self.advance();
            n += 1;
        }
        let mut at = self.cmp_expr()?;
        for i in 0..n {
            at = Expr::UniOp(Box::new(at), UniOpcode::Neg);
        }
        return Ok(at);
    }

    fn cmp_expr(&mut self) -> Result<Expr, String>
    {
        let lhs = self.add_expr()?;
        let op = if let Some(pop) = self.peek::<0>() {
            if pop.0.is_cmp() {
                Some(cmp_tok_to_binop(&pop.0))
            } else {
                None
            }
        } else {
            None
        };
        if let Some(sop) = op {
            self.advance();
            let rhs = self.add_expr()?;
            Ok(Expr::BinOp(
                Box::new(lhs),
                sop,
                Box::new(rhs),
            ))
        } else {
            Ok(lhs)
        }
    }

    fn add_expr(&mut self) -> Result<Expr, String>
    {
        let mut ae = self.add_term()?;
        while self.matches::<0>(TokenType::Plus)
           || self.matches::<0>(TokenType::Minus) {
            let op = self.read_token().unwrap().0.clone(); // +, -
            let rhs = self.add_term()?;
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

    fn add_term(&mut self) -> Result<Expr, String>
    {
        // count all unary Minuses before MulExpr
        let mut n = 0;
        while self.matches::<0>(TokenType::Minus) {
            self.advance();
            n += 1;
        }
        let mut at = self.mul_expr()?;
        for i in 0..n {
            at = Expr::UniOp(Box::new(at), UniOpcode::Sub);
        }
        return Ok(at);
    }

    fn mul_expr(&mut self) -> Result<Expr, String>
    {
        let mut me = self.mul_term()?;
        while self.matches::<0>(TokenType::Asterisk)
           || self.matches::<0>(TokenType::Slash) {
            let op = self.read_token().unwrap().0.clone(); // +, -
            let rhs = self.mul_term()?;
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

    fn mul_term(&mut self) -> Result<Expr, String>
    {
        // count all unary Slashes before AtomExpr
        let mut n = 0;
        while self.matches::<0>(TokenType::Slash) {
            self.advance();
            n += 1;
        }
        let mut mt = self.atom_expr()?;
        for i in 0..n {
            mt = Expr::UniOp(Box::new(mt), UniOpcode::Inv);
        }
        return Ok(mt);
    }

    fn atom_expr(&mut self) -> Result<Expr, String>
    {
        if let Some(t) = self.peek::<0>() {
            match t.0 {
                Token::ValB(b) => {
                    let ret = Expr::Const(Val::B(b));
                    self.advance();
                    return Ok(ret);
                },
                Token::ValN(n) => {
                    let ret = Expr::Const(Val::N(n));
                    self.advance();
                    return Ok(ret);
                },
                Token::ValR(r) => {
                    let ret = Expr::Const(Val::R(r));
                    self.advance();
                    return Ok(ret);
                },
                Token::PrimType(pt) => {
                    self.advance();
                    let casted = self.atom_expr()?;
                    return Ok(Expr::Tcast(pt.into(), Box::new(casted)));
                },
                Token::Ident(id) =>
                    return Ok(Expr::Ident(std::str::from_utf8(id)
                        .unwrap().to_owned())),
                Token::Lparen => self.paren_expr(),
                _ => expected_err!("ValN", t),
            }
        } else {
            expected_err!("ValN", (Token::Eof, 0))
        }
    }

    // not as in grammar, þis can return an empty vec
    // so þis parses `<CommaEx>?`
    fn comma_ex(&mut self, end: TokenType) -> Result<Vec<Expr>, String>
    {
        // check empty
        if self.matches::<0>(end) {
            return Ok(vec![]);
        }
        let mut exs: Vec<Expr> = vec![];
        loop {
            let ex = self.expr()?;
            exs.push(ex);
            if let Some(t) = self.peek::<0>() {
                let tt = TokenType::from(&t.0);
                if tt == end {
                    self.advance();
                    return Ok(exs);
                }
                if tt != TokenType::Comma {
                    return expected_err!(format!(", or {:?}", end), t);
                }
            } else {
                return expected_err!(format!(", or {:?}", end),
                    self.peek::<0>().unwrap());
            }
            self.advance();
        }
    }

    // called when found Lparen
    fn paren_expr(&mut self) -> Result<Expr, String>
    {
        self.advance();
        let e = self.add_expr()?;
        if self.matches::<0>(TokenType::Rparen) {
            self.advance();
            return Ok(e);
        } else {
            return expected_err!(')', self.peek::<0>().unwrap());
        }
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

fn cmp_tok_to_binop(t: &Token) -> BinOpcode
{
    match t {
        Token::Equal2 => BinOpcode::Eq,
        Token::Ne     => BinOpcode::Eq,
        Token::Langle => BinOpcode::Lt,
        Token::Le     => BinOpcode::Le,
        Token::Rangle => BinOpcode::Gt,
        Token::Ge     => BinOpcode::Ge,
        _ => panic!("trying to use cmp_tok_to_binop on a non-cmp token {:?}",
            t),
    }
}
