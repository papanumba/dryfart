/* src/parsnip/turnip.rs */

use super::toki::{Token, TokenType, PrimType};
use crate::asterix::*;

macro_rules! expected_err {
    ($e:expr, $f:expr) => { Err(String::from(
        format!("ParsnipError: Expected {} but found {:?} at line {}",
            $e, $f.0, $f.1)
    )) };
}

/*macro_rules! expected_err_peek {
    ($e:expr) => { expected_err!(e, self.peek::<0>().unwrap()) };
}*/

type LnToken<'a> = (Token<'a>, usize);

pub struct Parsnip<'src>
{
    cursor: usize,
    tokens: Vec<LnToken<'src>>,
}

impl<'src> Parsnip<'src>
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
            Token::Ident(i) => {
                if self.matches::<1>(TokenType::Equal) {
                    Some(self.assign(i))
                } else if self.matches::<1>(TokenType::Bang) {
                    Some(self.pccall(i))
                } else {
                    None
                }
            },
            Token::LsqBra  => Some(self.if_stmt()),
            Token::AtSign  => Some(self.loop_stmt()),
            Token::AtSign2 => Some(self.break_stmt()),
            Token::Hash2   => Some(self.return_stmt()),
            _ => None,
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
        return if pre.is_empty() { // even if post is empty
            Ok(Stmt::LoopIf(Loop::Ini(cond, post)))
        } else if post.is_empty() {
            Ok(Stmt::LoopIf(Loop::Fin(pre, cond)))
        } else {
            Ok(Stmt::LoopIf(Loop::Mid(pre, cond, post)))
        }
    }

    // called when peek: 0 -> @@
    fn break_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @@
        let mut level: u32 = 1;
        if let Some(t) = self.peek::<0>() {
            match t.0 {
                Token::ValN(n) => {level = n; self.advance();},
                Token::Period  => {}, // implicit level 1
                _ => return expected_err!("ValN or .", t),
            }
        } else {
            return expected_err!("ValN", ("EOF", "end"));
        };
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
        let mut mt = self.cast_expr()?;
        for i in 0..n {
            mt = Expr::UniOp(Box::new(mt), UniOpcode::Inv);
        }
        return Ok(mt);
    }

    fn cast_expr(&mut self) -> Result<Expr, String>
    {
        if let Some(t) = self.peek::<0>() {
            match t.0 {
                Token::PrimType(pt) => {
                    self.advance();
                    let casted = self.cast_expr()?;
                    Ok(Expr::Tcast(pt.into(), Box::new(casted)))
                }
                _ => self.idx_expr(),
            }
        } else {
            expected_err!(
                "prim type, fn call, anon fn, ident or literal",
                ("EOF", "end"))
        }
    }

    fn idx_expr(&mut self) -> Result<Expr, String>
    {
        let root = self.idx_term()?;
        if self.matches::<0>(TokenType::Uscore) {
            todo!()
        }
        return Ok(root);
    }

    fn idx_term(&mut self) -> Result<Expr, String>
    {
        let mut nucle = self.nucle()?;
        while self.matches::<0>(TokenType::Hash) {
            self.advance(); // #
            let args = self.comma_ex(TokenType::Semic)?;
            nucle = Expr::Fcall(Box::new(nucle), args);
        }
        return Ok(nucle);
    }

    fn nucle(&mut self) -> Result<Expr, String>
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
                Token::Ident(id) => {
                    self.advance();
                    return Ok(Expr::Ident(std::str::from_utf8(id)
                        .unwrap().to_owned()));
                },
                Token::AtSign => { // recurse ident
                    self.advance();
                    return Ok(Expr::Ident("@".to_string()));
                },
                Token::String(s) => self.string(s),
                Token::Lparen => self.paren_expr(),
                Token::Hash => self.anon_fn(),
                _ => expected_err!("#, (, ident or literal", t),
            }
        } else {
            expected_err!("ValN", (Token::Eof, 0))
        }
    }

    // not as in grammar, þis can return an empty vec so þis parses `<CommaEx>?`
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
        self.advance(); // (
        let e = self.expr()?;
        self.exp_adv(TokenType::Rparen)?;
        return Ok(e);
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
        if let Some(t) = self.peek::<0>() {
            if let Token::Ident(i) = t.0 {
                self.advance();
                return Ok(i);
            } else {
                expected_err!("Ident", t)
            }
        } else {
            Err(String::from("expected Ident, found EOF"))
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

fn cmp_tok_to_binop(t: &Token) -> BinOpcode
{
    match t {
        Token::Equal2 => BinOpcode::Eq,
        Token::Ne     => BinOpcode::Ne,
        Token::Langle => BinOpcode::Lt,
        Token::Le     => BinOpcode::Le,
        Token::Rangle => BinOpcode::Gt,
        Token::Ge     => BinOpcode::Ge,
        _ => panic!("trying to use cmp_tok_to_binop on a non-cmp token {:?}",
            t),
    }
}
