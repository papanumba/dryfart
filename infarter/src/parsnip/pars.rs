/* src/parsnip/pars.rs */

#![allow(dead_code)]

use std::{
    rc::Rc,
    cell::RefCell,
};
use super::toki::{Token, TokTyp, PrimType};
use crate::asterix::*;
use crate::util;

macro_rules! expected_err {
    ($e:expr, $f:expr) => { util::format_err!(
        "ParsnipError: Expected {} but found {} at line {}",
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
            while self.matches::<0>(TokTyp::$ttype) {
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
            while self.matches::<0>(TokTyp::$ttype) {
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

macro_rules! valx_fn {
    ($name:ident, $X:ident, $t:ty) => {
        // called when peek: 0 -> $X
        #[inline]
        fn $name(&mut self, x: $t) -> Expr
        {
            let val = Expr::Const(Val::$X(x));
            self.advance();
            return val;
        }
    }
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
            eof_err!(format!("{}", self.peek::<0>().unwrap().0))
        };
    }

    #[inline]
    fn peek<const LA: usize>(&self) -> Option<LnToken<'src>>
    {
        return self.tokens.get(self.cursor + LA).copied();
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
    fn matches<const LA: usize>(&self, m: TokTyp) -> bool
    {
        match self.peek::<LA>() {
            Some(t) => t.0.typ() == m,
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
    fn exp_adv(&mut self, t: TokTyp) -> Result<(), String>
    {
        if self.matches::<0>(t) {
            self.advance();
            Ok(())
        } else {
            expected_err!(format!("{:?}", t), self.peek::<0>().unwrap())
        }
    }

    #[allow(dead_code)] // debug only
    #[inline]
    fn print_peek(&self)
    {
        match self.peek::<0>() {
            Some((t, ln)) => println!("{t} at line {ln}"),
            None => println!("None"),
        }
    }

    /******** G R A M M A R ********/

    fn block(&mut self) -> Result<Block, String>
    {
        let mut stmts: Vec<Stmt> = vec![];
        loop {
            match self.stmt() {
                Some(st) => stmts.push(st?),
                None => return Ok(stmts),
            }
        }
    }

    #[inline]
    fn stmt(&mut self) -> Option<Result<Stmt, String>>
    {
        let Some(t) = self.peek::<0>() else {
            return None;
        };
        match t.0.typ() {
            // one of þe few 2-lookahead
            TokTyp::LsqBra  => Some(self.if_stmt()),
            TokTyp::AtSign  => Some(self.loop_stmt()),
            TokTyp::AtSign2 => Some(self.break_stmt()),
            TokTyp::Hash2   => Some(self.return_stmt()),
            TokTyp::Bang2   => Some(self.pc_end()),
            _ => self.other_stmt(),
        }
    }

    // þese are assigns, operons or pccalls
    fn other_stmt(&mut self) -> Option<Result<Stmt, String>>
    {
        const MSG: &'static str =
            "=, !, !$, ++, --, **, //, \\\\, && or ||";
        let Ok(lhs) = self.expr() else {
            return None;
        };
        let Some(t) = self.peek::<0>() else {
            return Some(eof_err!(MSG));
        };
        return Some(match t.0.typ() {
            TokTyp::Equal => self.assign(lhs),
            TokTyp::Bang => self.pccall(lhs),
            TokTyp::BangDollar => self.tbpcal(lhs),
            TokTyp::Plus2     |
            TokTyp::Minus2    |
            TokTyp::Asterisk2 |
            TokTyp::Slash2    |
            TokTyp::Bslash2   |
            TokTyp::And2      |
            TokTyp::Vbar2     |
            TokTyp::Caret2    => self.operon(lhs, t.0),
            _ => expected_err!(MSG, t),
        });
    }

    #[inline]
    fn assign(&mut self, lhs: Expr) -> Result<Stmt, String>
    {
        self.advance(); // =
        let e = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        Ok(Stmt::Assign(lhs, e))
    }

    #[inline]
    fn operon(&mut self, lhs: Expr, op: Token<'_>) -> Result<Stmt, String>
    {
        self.advance(); // op
        let binop = BinOpcode::try_from(op.typ())?;
        let ex = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::OperOn(lhs, binop, ex));
    }

    #[inline]
    fn pccall(&mut self, lhs: Expr) -> Result<Stmt, String>
    {
        self.advance(); // !
        let args = self.comma_ex(TokTyp::Period)?;
        return Ok(Stmt::PcCall(lhs, args));
    }

    #[inline]
    fn tbpcal(&mut self, lhs: Expr) -> Result<Stmt, String>
    {
        self.advance(); // !$
        let name = self.consume_ident()?;
        self.exp_adv(TokTyp::Bang)?; // !
        let args = self.comma_ex(TokTyp::Period)?;
        let name = Rc::new(name.try_into().unwrap());
        return Ok(Stmt::TbPCal(lhs, name, args));
    }

    // called when: peek 0 -> LsqBra
    fn if_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // [
        // parse mandatory 1st case of þe if
        let cond = self.expr()?;
        self.exp_adv(TokTyp::Then)?;
        let if_block = self.block()?;
        let if0 = IfCase::new(cond, if_block);
        // check if end
        if self.matches::<0>(TokTyp::RsqBra) {
            self.advance(); // ]
            return Ok(Stmt::IfElse(if0, vec![], None));
        }
        // loop until matching a "]" xor "| =>" (else case)
        let mut elseifs = vec![];
        loop {
            const MSG: &'static str = "] or |";
            let Some(tok) = self.peek::<0>() else {
                return eof_err!(MSG);
            };
            let tt0 = tok.0.typ();
            if tt0 == TokTyp::RsqBra {
                self.advance(); // ]
                return Ok(Stmt::IfElse(if0, elseifs, None));
            }
            // now must be an Elseif or an Else
            if tt0 != TokTyp::Vbar {
                return expected_err!(MSG, tok);
            }
            self.advance(); // |
            if self.matches::<0>(TokTyp::Then) { // Else
                self.advance(); // =>
                let eb = self.block()?;
                self.exp_adv(TokTyp::RsqBra)?;
                return Ok(Stmt::IfElse(if0, elseifs, Some(eb)));
            }
            // now must be an Elseif
            let cond = self.expr()?;
            self.exp_adv(TokTyp::Then)?;
            let blok = self.block()?;
            elseifs.push(IfCase::new(cond, blok));
        }
    }

    // called when peek: 0 -> @
    fn loop_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @
        let pre = self.block()?; // maybe empty
        if !self.matches::<0>(TokTyp::LsqBra2) { // infinite loop
            self.exp_adv(TokTyp::Period)?;
            return Ok(Stmt::LoopIf(Loop::Inf(pre)));
        }
        // now, þer should be þe condition
        self.exp_adv(TokTyp::LsqBra2)?;
        let cond = self.expr()?;
        self.exp_adv(TokTyp::RsqBra2)?;
        let post = self.block()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::LoopIf(Loop::Cdt(pre, cond, post)));
    }

    // called when peek: 0 -> @@
    fn break_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // @@
        let mut level: u32 = 0;
        let Some(t) = self.peek::<0>() else {
            return eof_err!("ValN");
        };
        match t.0.typ() {
            TokTyp::ValN => {
                level = t.0.as_valn().unwrap();
                self.advance();
            },
            TokTyp::Period  => {}, // implicit level 1
            _ => return expected_err!("ValN or .", t),
        }
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::BreakL(level));
    }

    // called when peek: 0 -> ##
    fn return_stmt(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // ##
        let ret = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::Return(ret));
    }

    // called when !!
    fn pc_end(&mut self) -> Result<Stmt, String>
    {
        self.advance(); // !!
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::PcExit);
    }

    fn expr(&mut self) -> Result<Expr, String>
    {
        return self.cor_expr();
    }

    left_binop_expr!( cor_expr, cand_expr, VbarQu,  Cor);
    left_binop_expr!(cand_expr,  cmp_expr,  AndQu, Cand);

    fn cmp_expr(&mut self) -> Result<Expr, String>
    {
        let first = self.or_expr()?;
        let mut others: Vec<(BinOpcode, Expr)> = vec![];
        while let Some(pop) = self.peek::<0>() {
            if !pop.0.is_cmp() {
                break;
            }
            let op = BinOpcode::try_from(pop.0.typ()).unwrap();
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
        while self.matches::<0>(TokTyp::Plus)
           || self.matches::<0>(TokTyp::Minus) {
            let op = self.read_token().unwrap().0; // +, -
            let rhs = self.neg_expr()?;
            let op = match op.typ() {
                TokTyp::Plus  => BinOpcode::Add,
                TokTyp::Minus => BinOpcode::Sub,
                _ => unreachable!(),
            };
            ae = Expr::BinOp(Box::new(ae), op, Box::new(rhs));
        }
        return Ok(ae);
    }

    rite_uniop_expr!(neg_expr, mul_expr, Minus, Neg);

    fn mul_expr(&mut self) -> Result<Expr, String>
    {
        let mut me = self.inv_expr()?;
        while self.matches::<0>(TokTyp::Asterisk)
           || self.matches::<0>(TokTyp::Slash)
           || self.matches::<0>(TokTyp::Bslash) {
            let op = self.read_token().unwrap().0; // *, /, \
            let rhs = self.inv_expr()?;
            let op = match op.typ() {
                TokTyp::Asterisk => BinOpcode::Mul,
                TokTyp::Slash    => BinOpcode::Div,
                TokTyp::Bslash   => BinOpcode::Mod,
                _ => unreachable!(),
            };
            me = Expr::BinOp(Box::new(me), op, Box::new(rhs));
        }
        return Ok(me);
    }

    rite_uniop_expr!(inv_expr,  not_expr,  Slash, Inv);
    rite_uniop_expr!(not_expr,  idx_expr,  Tilde, Not);
    left_binop_expr!(idx_expr, cast_expr, Uscore, Idx);

    fn cast_expr(&mut self) -> Result<Expr, String>
    {
        let Some(t) = self.peek::<0>() else {
            return eof_err!("type%, ident or literal");
        };
        if t.0.typ() != TokTyp::PrimType {
            return self.fn_acc_ex();
        }
        self.advance(); // þe primtype
        let casted = self.cast_expr()?;
        return Ok(Expr::Tcast(
            t.0.as_primtype().unwrap().into(),
            Box::new(casted)
        ));
    }

    fn fn_acc_ex(&mut self) -> Result<Expr, String>
    {
        let mut e = self.nucle()?;
        loop {
            let Some(t) = self.peek::<0>() else {
                break;
            };
            match t.0.typ() {
                TokTyp::Dollar => {
                    self.advance(); // $
                    let i = self.consume_ident()?;
                    e = Expr::TblFd(Box::new(e),
                        Rc::new(i.try_into().unwrap()),
                    );
                },
                TokTyp::Hash => {
                    self.advance(); // #
                    let args = self.comma_ex(TokTyp::Semic)?;
                    e = Expr::Fcall(Box::new(e), args);
                },
                TokTyp::HashDollar => {
                    self.advance(); // #$
                    let i = self.consume_ident()?;
                    self.exp_adv(TokTyp::Hash)?; // #
                    let args = self.comma_ex(TokTyp::Semic)?;
                    e = Expr::TbFcl(Box::new(e),
                        Rc::new(i.try_into().unwrap()),
                        args);
                },
                _ => break,
            }
        }
        return Ok(e);
    }

    fn nucle(&mut self) -> Result<Expr, String>
    {
        const MSG: &'static str = "(, #, !, _, $, \\[, \\#, ident or literal";
        let Some(tok) = self.peek::<0>() else {
            return eof_err!(MSG);
        };
        match tok.0.typ() {
            TokTyp::Lparen => self.parented(),
            TokTyp::Hash => self.func(tok.1),
            TokTyp::BsLsb => self.if_expr(),
            TokTyp::BsHash => self.short_fn(tok.1),
            TokTyp::RecF => {
                self.advance();
                Ok(Expr::RecFn)
            },
            TokTyp::Bang => self.proc(tok.1),
            TokTyp::RecP => {
                self.advance();
                Ok(Expr::RecPc)
            },
            TokTyp::Uscore =>     self.arrlit(),
            TokTyp::Dollar =>     self.tbllit(),
            TokTyp::RecT => {
                self.advance();
                Ok(Expr::RecsT(tok.0.as_rect().unwrap()))
            },
            TokTyp::Ident => {
                self.advance();
                let id = tok.0.as_ident().unwrap();
                return Ok(Expr::Ident(Rc::new(
                    id.try_into().unwrap()
                )));
            },
            // literals
            TokTyp::ValV => {self.advance(); Ok(Expr::Const(Val::V))},
            TokTyp::ValB => Ok(self.valb(tok.0.as_valb().unwrap())),
            TokTyp::ValC => Ok(self.valc(tok.0.as_valc().unwrap())),
            TokTyp::ValN => Ok(self.valn(tok.0.as_valn().unwrap())),
            TokTyp::ValZ => Ok(self.valz(tok.0.as_valz().unwrap())),
            TokTyp::ValR => Ok(self.valr(tok.0.as_valr().unwrap())),
            TokTyp::String =>  self.string(tok.0.as_string().unwrap()),
            _ => expected_err!(MSG, tok),
        }
    }

    valx_fn!(valb, B, bool);
    valx_fn!(valc, C, u8);
    valx_fn!(valn, N, u32);
    valx_fn!(valz, Z, i32);
    valx_fn!(valr, R, f32);

    // parses comma separated exprs which end in a specific token
    // it also consumes þe end token, so no need to exp_adv after
    fn comma_ex(&mut self, end: TokTyp) -> Result<Vec<Expr>, String>
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
            let Some(tok) = self.peek::<0>() else {
                return eof_err!(comma_or_end);
            };
            let tt = tok.0.typ();
            if tt == end {
                self.advance(); // consume end
                return Ok(exs);
            }
            if tt != TokTyp::Comma {
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
        self.exp_adv(TokTyp::Rparen)?;
        return Ok(e);
    }

    // called when _
    fn arrlit(&mut self) -> Result<Expr, String>
    {
        self.advance(); // _
        let arr_e = self.comma_ex(TokTyp::Semic)?;
        return Ok(Expr::Array(arr_e));
    }

    // called when $
    fn tbllit(&mut self) -> Result<Expr, String>
    {
        const MSG: &'static str = "Ident or ;";
        self.advance(); // $
        let mut tbl_e = vec![];
        loop {
            let Some(t) = self.peek::<0>() else {
                return eof_err!(MSG);
            };
            match t.0.typ() {
                TokTyp::Ident => {}, // ok, continue reading
                TokTyp::Semic => break,
                _ => return expected_err!(MSG, t),
            }
            let i = self.consume_ident()?;
            self.exp_adv(TokTyp::Equal)?;
            let e = self.expr()?;
            self.exp_adv(TokTyp::Period)?;
            let i = Rc::new(i.try_into().unwrap());
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
        let name = match self.peek::<0>() {
            Some((t, _)) => if let Some(s) = t.as_string() {
                Some(Rc::new(s.try_into().unwrap()))
            } else { None },
            _ => None,
        };
        if name.is_some() {
            self.advance(); // string
        }
        let end_tok = match st {
            SubrType::F => TokTyp::Semic,
            SubrType::P => TokTyp::Period,
        };
        let pars: Vec<Rc<DfStr>> = self.pars(end_tok)?
            .iter()
            .map(|b| Rc::new(b.try_into().unwrap()))
            .collect();
        let bloq = self.block()?;
        self.exp_adv(TokTyp::Period)?;
        let meta = SubrMeta { line: line, name: name };
        let subr = Subr {
            meta: meta,
            upvs: vec![],
            pars: pars,
            body: bloq
        };
        let mrs = Rc::new(RefCell::new(subr));
        return Ok(match st {
            SubrType::F => Expr::FnDef(mrs),
            SubrType::P => Expr::PcDef(mrs),
        });
    }

    // matches (Ident (Comma Ident)*)? END
    fn pars(&mut self, end: TokTyp) -> Result<Vec<&[u8]>, String>
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
            self.exp_adv(TokTyp::Comma)?;
            let id = self.consume_ident()?;
            res.push(id);
        }
        self.advance(); // END
        return Ok(res);
    }

    // called when \#
    #[inline]
    fn short_fn(&mut self, line: usize) -> Result<Expr, String>
    {
        self.advance(); // \#
        // TODO: maybe put actual name of short functions?
        let pars: Vec<Rc<DfStr>> = self.pars(TokTyp::Semic)?
            .iter()
            .map(|b| Rc::new(b.try_into().unwrap()))
            .collect();
        let ret_expr = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        let meta = SubrMeta { line: line, name: None };
        let subr = Subr {
            meta: meta,
            upvs: vec![],
            pars: pars,
            body: vec![Stmt::Return(ret_expr)],
        };
        let mrs = Rc::new(RefCell::new(subr));
        return Ok(Expr::FnDef(mrs));
    }

    // called when \[
    #[inline]
    fn if_expr(&mut self) -> Result<Expr, String>
    {
        self.advance(); // \[
        let mut cases = vec![];
        loop {
            if self.matches::<0>(TokTyp::Then) {
                todo!("final else =>");
            }
            let e = self.expr()?;
            if !cases.is_empty() && self.matches::<0>(TokTyp::RsqBra) {
                self.advance(); // ]
                return Ok(Expr::IfExp(cases, Box::new(e)));
            }
            if self.exp_adv(TokTyp::Then).is_err() {
                let msg = if !cases.is_empty() {"=> or ]"} else {"=>"};
                return expected_err!(msg, self.peek::<0>().unwrap());
            }
            let f = self.expr()?;
            self.exp_adv(TokTyp::Semic)?;
            cases.push((e, f));
        }
    }

    fn consume_ident(&mut self) -> Result<&'src [u8], String>
    {
        let Some(tok) = self.peek::<0>() else {
            return eof_err!("Ident");
        };
        let Some(i) = tok.0.as_ident() else {
            return expected_err!("Ident", tok);
        };
        self.advance(); // ident
        return Ok(i);
    }

    // called when curr tok is String
    fn string(&mut self, b: &[u8]) -> Result<Expr, String>
    {
        let a = Array::try_from(b)?;
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

impl TryFrom<TokTyp> for BinOpcode
{
    type Error = String;
    fn try_from(t: TokTyp) -> Result<Self, Self::Error>
    {
        match t {
            TokTyp::Equal2 => Ok(BinOpcode::Eq),
            TokTyp::Ne     => Ok(BinOpcode::Ne),
            TokTyp::Langle => Ok(BinOpcode::Lt),
            TokTyp::Le     => Ok(BinOpcode::Le),
            TokTyp::Rangle => Ok(BinOpcode::Gt),
            TokTyp::Ge     => Ok(BinOpcode::Ge),
            // for Operons
            TokTyp::Plus2     => Ok(BinOpcode::Add),
            TokTyp::Minus2    => Ok(BinOpcode::Sub),
            TokTyp::Asterisk2 => Ok(BinOpcode::Mul),
            TokTyp::Slash2    => Ok(BinOpcode::Div),
            TokTyp::Bslash2   => Ok(BinOpcode::Mod),
            TokTyp::And2      => Ok(BinOpcode::And),
            TokTyp::Vbar2     => Ok(BinOpcode::Or),
            TokTyp::Caret2    => Ok(BinOpcode::Xor),
            _ => unreachable!("cannot convert token {:?} into a BinOp", t),
        }
    }
}
