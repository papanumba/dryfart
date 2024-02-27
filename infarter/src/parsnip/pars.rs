/* src/parsnip/pars.rs */

#![allow(dead_code)]

use std::rc::Rc;
use super::toki::{Token, TokenType, PrimType};
use crate::asterix::*;
use crate::util;

macro_rules! expected_err {
    ($e:expr, $f:expr) => { util::format_err!(
        "ParsnipError: Expected {} but found {:?} at line {}",
        $e, $f.0, $f.1
    ) };
}

macro_rules! eof_err {
    ($e:expr) => {
        util::format_err!("ParsnipError: Expected {} but found EOF", $e)
    };
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
    pub fn new(tl: Vec<LnToken<'src>>) -> Self
    {
        return Self {
            cursor: 0,
            tokens: tl,
        };
    }

    pub fn parse(&mut self) -> Result<Block, String>
    {
        let res = self.block()?;
        return if self.is_at_end() {
            Ok(res)
        } else {
            eof_err!(format!("{:?}", self.peek::<0>().unwrap().0))
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
            Token::LsqBra  => Some(self.if_stmt()),
            Token::AtSign  => Some(self.loop_stmt()),
            Token::AtSign2 => Some(self.break_stmt()),
            Token::Hash2   => Some(self.return_stmt()),
            Token::Bang2   => Some(self.pc_end()),
            _ => self.other_stmt(),
        }
    }

    // þese are assigns, operons or pccalls
    fn other_stmt(&mut self) -> Option<Result<Stmt, String>>
    {
        const MSG: &'static str = "=, !, ++, --, ** or //";
        let lhs = match self.expr() {
            Ok(e) => e,
            _ => return None,
        };
        Some(match self.peek::<0>() {
            Some(t) => match t.0 {
                Token::Equal => self.assign(lhs),
                Token::Bang => self.pccall(lhs),
                Token::Plus2 |
                Token::Minus2 |
                Token::Asterisk2 |
                Token::Slash2 => todo!(), //self.operon(id, t.0),
                _ => expected_err!(MSG, t),
            },
            None => eof_err!(MSG),
        })
    }

    #[inline]
    fn assign(&mut self, lhs: Expr) -> Result<Stmt, String>
    {
        self.advance(); // =
        let e = self.expr()?;
        self.exp_adv(TokenType::Period)?;
        Ok(Stmt::Assign(lhs, e))
    }

/*    #[inline]
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
    }*/

    #[inline]
    fn pccall(&mut self, lhs: Expr) -> Result<Stmt, String>
    {
        self.advance(); // !
        let args = self.comma_ex(TokenType::Period)?;
        return Ok(Stmt::PcCall(lhs, args));
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
        if !self.matches::<0>(TokenType::LsqBra2) { // infinite loop
            self.exp_adv(TokenType::Period)?;
            return Ok(Stmt::LoopIf(Loop::Inf(pre)));
        }
        // now, þer should be þe condition
        self.exp_adv(TokenType::LsqBra2)?;
        let cond = self.expr()?;
        self.exp_adv(TokenType::RsqBra2)?;
        let post = self.block()?;
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::LoopIf(Loop::Cdt(pre, cond, post)));
    }

    // called when peek: 0 -> @@
    fn break_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @@
        let mut level: u32 = 0;
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

    // called when !!
    fn pc_end(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // !!
        self.exp_adv(TokenType::Period)?;
        return Ok(Stmt::PcExit);
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

    rite_uniop_expr!(inv_expr,  not_expr,  Slash, Inv);
    rite_uniop_expr!(not_expr,  idx_expr,  Tilde, Not);
    left_binop_expr!(idx_expr, cast_expr, Uscore, Idx);

    fn cast_expr(&mut self) -> Result<Expr, String>
    {
        match self.peek::<0>() {
            Some(t) => match t.0 {
                Token::PrimType(pt) => {
                    self.advance(); // þe primtype
                    let casted = self.cast_expr()?;
                    Ok(Expr::Tcast(pt.into(), Box::new(casted)))
                }
                _ => self.acc_expr(),
            },
            _ => eof_err!("type%, ident or literal"),
        }
    }

    fn acc_expr(&mut self) -> Result<Expr, String>
    {
        let mut e = self.fn_call()?;
        while self.matches::<0>(TokenType::Dollar) {
            self.advance(); // $
            let i = self.consume_ident()?;
            e = Expr::TblFd(Box::new(e),
                String::from(std::str::from_utf8(i).unwrap()),
            );
        }
        return Ok(e);
    }

    fn fn_call(&mut self) -> Result<Expr, String>
    {
        let mut e = self.nucle()?;
        while self.matches::<0>(TokenType::Hash) {
            self.advance(); // #
            let args = self.comma_ex(TokenType::Semic)?;
            e = Expr::Fcall(Box::new(e), args);
        }
        return Ok(e);
    }

    fn nucle(&mut self) -> Result<Expr, String>
    {
        const MSG: &'static str = "(, #, !, _, $, ident or literal";
        let tok = match self.peek::<0>() {
            Some(t) => t,
            None => return eof_err!(MSG),
        };
        match tok.0 {
            Token::Lparen => self.parented(),
            Token::Hash => self.func(tok.1),
            Token::RecF => {
                self.advance();
                Ok(Expr::RecFn)
            },
            Token::Bang => self.proc(tok.1),
            Token::RecP => {
                self.advance();
                Ok(Expr::RecPc)
            },
            Token::Uscore =>     self.arrlit(),
            Token::Dollar =>     self.tbllit(),
            Token::RecT(l) => {
                self.advance();
                Ok(Expr::RecsT(l))
            },
            Token::Ident(id) => {
                self.advance();
                return Ok(Expr::Ident(std::str::from_utf8(id)
                    .unwrap().to_owned()))
            },
            // literals
            Token::ValB(b) => Ok(self.valb(b)),
            Token::ValN(n) => Ok(self.valn(n)),
            Token::ValZ(z) => Ok(self.valz(z)),
            Token::ValR(r) => Ok(self.valr(r)),
            Token::String(s) =>  self.string(s),
            _ => expected_err!(MSG, tok),
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

    // called when (
    fn parented(&mut self) -> Result<Expr, String>
    {
        self.advance(); // (
        let e = self.expr()?;
        self.exp_adv(TokenType::Rparen)?;
        return Ok(e);
    }

    // called when _
    fn arrlit(&mut self) -> Result<Expr, String>
    {
        self.advance(); // _
        let arr_e = self.comma_ex(TokenType::Semic)?;
        return Ok(Expr::Array(arr_e));
    }

    // called when $
    fn tbllit(&mut self) -> Result<Expr, String>
    {
        const MSG: &'static str = "Ident or ;";
        self.advance(); // $
        let mut tbl_e: Vec<(String, Expr)> = vec![];
        loop {
            if let Some(t) = self.peek::<0>() {
                match t.0 {
                    Token::Ident(_) => {}, // ok, continue
                    Token::Semic => break,
                    _ => return expected_err!(MSG, t),
                }
            } else {
                return eof_err!(MSG);
            }
            let i = self.consume_ident()?;
            self.exp_adv(TokenType::Equal)?;
            let e = self.expr()?;
            self.exp_adv(TokenType::Period)?;
            let i = String::from(std::str::from_utf8(i).unwrap());
            tbl_e.push((i, e));
        }
        self.advance(); // ;
        Ok(Expr::Table(tbl_e))
    }

    // called when #
    fn func(&mut self, line: usize) -> Result<Expr, String>
    {
        self.subr(line, SubrType::F)
    }

    // called when !
    fn proc(&mut self, line: usize) -> Result<Expr, String>
    {
        self.subr(line, SubrType::P)
    }

    // helper for func & proc
    fn subr(&mut self, line: usize, st: SubrType) -> Result<Expr, String>
    {
        self.advance(); // # or !
        let name: Option<String> = match self.peek::<0>() {
            Some((Token::String(s), _)) =>
                Some(std::str::from_utf8(*s).unwrap().to_string()),
            _ => None,
        };
        if name.is_some() {
            self.advance(); // string
        }
        let end_tok = match st {
            SubrType::F => TokenType::Semic,
            SubrType::P => TokenType::Period,
        };
        let pars: Vec<String> = self.pars(end_tok)?
            .iter()
            .map(|b| String::from(std::str::from_utf8(b).unwrap()))
            .collect();
        let bloq = self.block()?;
        self.exp_adv(TokenType::Period)?;
        let meta = SubrMeta { line: line, name: name };
        let subr = Subr { meta: meta, pars: pars, body: bloq };
        let rced = Rc::new(subr);
        return Ok(match st{
            SubrType::F => Expr::FnDef(rced),
            SubrType::P => Expr::PcDef(rced),
        });
    }

    // matches (Ident (Comma Ident)*)? END
    fn pars(&mut self, end: TokenType) -> Result<Vec<&[u8]>, String>
    {
        let mut res: Vec<&[u8]> = vec![];
        if self.matches::<0>(end) {
            self.advance();
            return Ok(res);
        }
        if let Ok(i) = self.consume_ident() {
            res.push(i);
        }
        while !self.matches::<0>(end) {
            self.exp_adv(TokenType::Comma)?;
            let id = self.consume_ident()?;
            res.push(id);
        }
        self.advance(); // END
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
        return Ok(Expr::Const(Val::from_array(a)));
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
            _ => util::format_err!(
                "cannot convert token {:?} into a BinOp", t
            ),
        }
    }
}
