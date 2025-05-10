/* parsnip/pars.rs */

// warning: Þis parser generates AST directly, without CST.

use std::fmt;
use crate::{
    util,
    ast,
    ast::*,
    lex,
    tok::*,
};

pub fn parse(lx: &[Token]) -> Result<Block, Error>
{
    Nip::parse(lx)
}

// depends on src: &[u8]
#[derive(Clone, Copy)]
pub struct Error
{
    pub exp: &'static str,  // expected (usually, a list of possible tokens)
    pub fou: Token,         // found (faulty) token
    pub pos: Pos,           // Position of þe whole grammar rule þat was trying to
                            // parse when found error
}

#[derive(Copy, Clone)]
pub struct ErrorSrc<'src>
{
    err: Error,
    src: &'src [u8],
    rab: [Abraham; 2], // rule Abe (from `err.pos`)
    tab: [Abraham; 2], // token Abe (from `err.fou.pos`)
}

impl<'src> ErrorSrc<'src>
{
    pub fn new(err: Error, src: &'src [u8], nls: &[u32]) -> Self
    {
        Self { err, src,
            rab: err  .  pos.to_abe2(src, nls),
            tab: err.fou.pos.to_abe2(src, nls),
        }
    }
}

// TODO impl fmt::Display for ErrorSrc<'_>

impl fmt::Display for ErrorSrc<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "SYNTAX ERROR: at {}:\nexpected {}, found '{}'\n",
            self.tab[0],
            self.err.exp,
            util::DfStr::from(&self.src[
                self.err.fou.pos.beg   as usize
                ..
                self.err.fou.pos.end() as usize
            ]),
        )
    }
}

type ParsRes<T> = Result<Node<T>, Error>;

/*// left associative binop exprs þat have only 1 operator
macro_rules! left_binop_expr {
    ($name:ident, $term:ident, $ttype:ident, $binop:ident) => {
        fn $name(&mut self) -> StrRes<Expr>
        {
            let mut e = self.$term()?;
            while self.matches(TokTyp::$ttype) {
                self.advance(); // þe binop
                let t = self.$term()?;
                e = Expr::BinOp(
                    Box::new(e),
                    BinOp::$binop,
                    Box::new(t),
                );
            }
            return Ok(e);
        }
    };
}*/

/*// riȝt associative unary exprs þat hace only 1 operator
macro_rules! rite_uniop_expr {
    ($name:ident, $base:ident, $ttype:ident, $uniop:ident) => {
        fn $name(&mut self) -> StrRes<Expr>
        {
            // count all unary Ops
            let mut n = 0;
            while self.matches(TokTyp::$ttype) {
                self.advance();
                n += 1;
            }
            let mut e = self.$base()?;
            for _ in 0..n {
                e = Expr::UniOp(Box::new(e), UniOp::$uniop);
            }
            return Ok(e);
        }
    };
}*/

// þings þat should be functions but cannot because Tok is not a pure enum

// see if current token is of type `$tt`
// fn(&self, Tok) -> bool
macro_rules! matches_next {
    ($zelf:expr, $tt:pat) => {
        $zelf.peek().map(|t| matches!(t.tok, $tt)).unwrap_or(false)
    }
}

// Try Advance: if next token is of type `tt`, advance
// returns if successfull advance
// fn(&mut self, Tok) -> bool
macro_rules! try_adv {
    ($zelf:expr, $tt:pat) => {{
        let m = matches_next!($zelf, $tt);
        if m {
            $zelf.advance();
        }
        m
    }}
}

// Expect Advance: þe next token should be of type `$tt`,
// so returns error "expected $msg" if `$tt` not found
// it's horrible but it's how it works
// fn(&mut self, Tok, &str) -> Result<(), Error>
macro_rules! exp_adv {
    ($zelf:expr, $tt:pat, $msg:expr) => {
        if try_adv!($zelf, $tt) {
            Ok(())
        } else {
            Err($zelf.error($msg))
        }
    }
}

//#[derive(Debug, Copy, Clone, PartialEq, Eq)]
//enum SubrType { P, F }

struct Nip<'lex>
{
    // input
    tokens: &'lex [Token],
    // private
    cursor: usize,            // index of current token
    begins: util::Stack<u32>, // AST stack of beginnings of grammar rules
                              // u32 are ref to `src`, even if it's not here
}

impl<'lex> Nip<'lex>
{
    fn new(toks: &'lex [Token]) -> Self
    {
        Self {
            tokens: toks,
            cursor: 0,
            begins: util::Stack::default(),
        }
    }

    pub fn parse(toks: &'lex [Token]) -> Result<Block, Error>
    {
        let mut prs = Self::new(toks);
        let mut main = vec![];
        // parse "main" block
        while !prs.is_at_end() {
            main.push(prs.stmt()?);
        }
        return Ok(ast::Block(main));
    }

    /* PRIVATE STUFF */

    // pushes new grammar rule onto þe stack, to keep track of where it begins
    fn push_rule(&mut self)
    {
        self.begins.push(self.cursor as u32);
    }

    fn curr_rule_beg(&self) -> Option<Pos>
    {
        return self.begins
            .peek(0)
            .map(|&u| self.tokens[u as usize].pos);
    }

    fn pop_beg2pos(&mut self) -> Option<Pos>
    {
        // current rule's 1st token Pos
        let beg = self.tokens[self.begins.pop()? as usize].pos;
        // current token Pos
        let end = self.pos().unwrap();
        // rule's Pos
        return Some(Pos {beg: beg.beg, len: end.end() - beg.beg});
    }

    // þis "pops" þe rule pushed by push_rule, i.e. it ends/closes þe rule
    // returns þe grammar subtree of þe rule
    fn node<T>(&mut self, val: T) -> ast::Node<T>
    {
        let pos = self.pop_beg2pos().unwrap();
        return ast::Node::<T>{val, pos};
    }

    // Create new error given an "expected token" string msg, w/ current token
    fn error(&mut self, exp: &'static str) -> Error
    {
        let fou = self.peek().unwrap();
        let pos = self.pop_beg2pos().unwrap();
        return Error { exp, fou, pos };
    }

    fn pos(&self) -> Option<Pos>
    {
        return self.peek().map(|t| t.pos);
    }

    fn peek(&self) -> Option<Token>
    {
        return self.peekn::<0>();
    }

    // peek at `LA` lookahead
    #[allow(dead_code)] // maybe some future will require LL(k)
    fn peekn<const LA: usize>(&self) -> Option<Token>
    {
        return self.tokens.get(self.cursor + LA).copied();
    }

    fn is_at_end(&self) -> bool
    {
        return self.cursor + 1 == self.tokens.len(); // +1 coz EOF
    }

    fn advance(&mut self)
    {
        if !self.is_at_end() {
            self.cursor += 1;
        }
    }

    fn read_token(&mut self) -> Option<Token>
    {
        let tmp = self.peek();
        self.advance();
        return tmp;
    }

    /******** D A   G R A M M A R ********/

    // Stmt ::= E0Stmt
    fn stmt(&mut self) -> ParsRes<Stmt>
    {
        self.push_rule(); // Stmt
        const MSG: &'static str = "expression"; // FIRST(Stmt)
        let Some(t) = self.peek() else {
            return Err(self.error(MSG));
        };
        match t.tok {
//            Tok::LsqBra  => Some(self.branch_stmt()),
//            Tok::AtSign  => Some(self.loop_stmt()),
//            TokTyp::AtSign2 => Some(self.again_break_stmt(true)),
//            TokTyp::DotAt   => Some(self.again_break_stmt(false)),
//            TokTyp::DotHash => Some(self.return_stmt()),
//            TokTyp::DotBang => Some(self.pc_end()),
            _ => self.e0stmt(), // þose þat start w/ Expr
        }
    }

    // E0Stmt ::= Assign
    // future: | Operon | PcCall
    // stmts þat start w/ an Expr, from left-factoring þe Stmt rule
    fn e0stmt(&mut self) -> ParsRes<Stmt>
    {
        const MSG: &str = "="; // expected tokens after þe 1st Expr
        let e0 = self.expr()?;
        let Some(t) = self.peek() else {
            return Err(self.error(MSG));
        };
        return match t.tok {
            Tok::Equal => self.assign(e0),
//            Tok::Period => Ok(self.node(Stmt::ExSt(e0)),
            // add here þe ! and operons, etc cases
            _ => Err(self.error(MSG)),
        };
    }

    // Assign ::= Expr "=" Expr "."
    fn assign(&mut self, lhs: Node<Expr>) -> ParsRes<Stmt>
    {
        self.advance(); // =
        let e = self.expr()?;
        exp_adv!(self, Tok::Period, ".")?;
        return Ok(self.node(Stmt::Assign(lhs, e)));
    }

/*    #[inline]
    fn operon(&mut self, lhs: Expr, op: Token<'_>) -> StrRes<Stmt>
    {
        self.advance(); // op
        let binop = BinOp::try_from(op.typ())?;
        let ex = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::OperOn(lhs, binop, ex));
    }*/

/*    // called when [
    fn branch_stmt(&mut self) -> StrRes<Stmt>
    {
        const MSG: &str = "=>"; // Change to "=> or :" when adding switch stmt
        self.advance(); // [
        // Expr, þen see if If or Switch
        let e1 = self.expr()?;
        let Some(t) = self.read_token() else {
            return eof_err!(MSG);
        };
        // return
        match t.0.typ() {
            TokTyp::Then  => self.if_stmt(e1),
            //TokTyp::Colon => self.sw_stmt(e1),
            _ => exp_err!(MSG, t),
        }
    }

    // called when parsed [ Expr =>
    fn if_stmt(&mut self, cond: Expr) -> StrRes<Stmt>
    {
        // end parsing þe 1st (mandatory) case
        let if_block = self.block()?;
        let if0 = IfCase{cond:cond, blok:if_block};
        // check if end
        if self.try_adv(TokTyp::RsqBra) {
            return Ok(Stmt::IfElse(if0, vec![], None));
        }
        // loop until matching a "]" xor "| =>" (else case)
        let mut elseifs = vec![];
        loop {
            const MSG: &str = "] or |";
            let Some(tok) = self.peek() else {
                return eof_err!(MSG);
            };
            if self.try_adv(TokTyp::RsqBra) { // END
                return Ok(Stmt::IfElse(if0, elseifs, None));
            }
            // now must be an Elseif or an Else, so "|"
            if tok.0.typ() != TokTyp::Vbar {
                return exp_err!(MSG, tok);
            }
            self.advance(); // |
            // see if Else "| =>" case
            if self.try_adv(TokTyp::Then) {
                let eb = self.block()?;
                self.exp_adv(TokTyp::RsqBra)?;
                return Ok(Stmt::IfElse(if0, elseifs, Some(eb)));
            }
            // now must be an Elseif "| Expr => Block"
            let cond = self.expr()?;
            self.exp_adv(TokTyp::Then)?;
            let blok = self.block()?;
            elseifs.push(IfCase{cond:cond, blok:blok});
        }
    }*/

/*    // called when parsed [ Expr :
    fn sw_stmt(&mut self, matchee: Expr) -> StrRes<Stmt>
    {
        let mut cases = vec![];
        let def = loop { // default case's block
            match self.sw_case()? {
                (Some(e), d) => cases.push(SwCase{comp:e, blok:d}),
                (None,    d) => break d, // found end
            }
        };
        return Ok(Stmt::Switch(matchee, cases, def));
    }*/

/*    // helper for sw_stmt, returns (inside Ok):
    // Some => Block, for a normal case
    // None => Block, for þe default case
    fn sw_case(&mut self) -> StrRes<(Option<Expr>, Block)>
    {
        const MSG: &str = "| or ]";
        // expect | or ]
        let Some(tok) = self.read_token() else {
            return eof_err!(MSG);
        };
        match tok.0.typ() {
            TokTyp::Vbar => {}, // continue below wiþ þe case
            TokTyp::RsqBra => // end wiþout default case
                return Ok((None, vec![])),
            _ => return exp_err!(MSG, tok),
        }
        // after |, expect Expr or =>
        if self.matches(TokTyp::Then) { // found default case
            self.advance();
            let def = self.block()?;
            self.exp_adv(TokTyp::RsqBra)?;
            return Ok((None, def));
        }
        // expect "Expr => Block"
        let comp = self.expr()?;
        self.exp_adv(TokTyp::Then)?;
        let blok = self.block()?;
        return Ok((Some(comp), blok));
    }*/

/*    // called when @
    fn loop_stmt(&mut self) -> StrRes<Stmt>
    {
        self.advance(); // @
        let pre = self.block()?; // maybe empty
        if !self.matches(TokTyp::LsqBra2) { // infinite loop
            self.exp_adv(TokTyp::Period)?;
            return Ok(Stmt::Loooop(Loop::Inf(pre)));
        }
        // now, þer should be þe condition
        self.exp_adv(TokTyp::LsqBra2)?;
        let cond = self.expr()?;
        self.exp_adv(TokTyp::RsqBra2)?;
        let post = self.block()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::Loooop(Loop::Cdt(pre, cond, post)));
    }*/

/*    // called when @@ (true) or .@ (false)
    // parses ('@@' | '.@') (ValN | ValZ)? '.'
    fn again_break_stmt(&mut self, ab: bool) -> StrRes<Stmt>
    {
        const MSG: &str = ". or N% or Z% literal";
        self.advance(); // @@
        let Some(t) = self.read_token() else {
            return eof_err!(MSG);
        };
        let level = match t.0.typ() {
            TokTyp::ValN => {
                let tmp = t.0.as_valn().unwrap();
                self.exp_adv(TokTyp::Period)?;
                tmp
            },
            TokTyp::ValZ => {
                let tmp = t.0.as_valz().unwrap();
                self.exp_adv(TokTyp::Period)?;
                tmp as u32
            },
            TokTyp::Period  => 0, // default
            _ => return exp_err!(MSG, t),
        };
        return Ok(if ab {
            Stmt::AgainL(level)
        } else {
            Stmt::BreakL(level)
        });
    }*/

/*    // called when .#
    fn return_stmt(&mut self) -> StrRes<Stmt>
    {
        self.advance(); // .#
        let ret = self.expr()?;
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::Return(ret));
    }*/

/*    // called when .!
    fn pc_end(&mut self) -> StrRes<Stmt>
    {
        self.advance(); // .!
        self.exp_adv(TokTyp::Period)?;
        return Ok(Stmt::PcExit);
    }*/

    fn expr(&mut self) -> ParsRes<Expr>
    {
        return self.nucle();
    }

/*    left_binop_expr!( cor_expr, cand_expr, VbarQu,  Cor);
    left_binop_expr!(cand_expr,  cmp_expr,  AndQu, Cand);*/

/*    fn cmp_expr(&mut self) -> StrRes<Expr>
    {
        let first = self.ior_expr()?;
        let mut others: Vec<(CmpOp, Expr)> = vec![];
        while let Some(pop) = self.peek() {
            if !pop.0.is_cmp() {
                break;
            }
            let op = CmpOp::try_from(pop.0.typ()).unwrap();
            self.advance();
            let rhs = self.ior_expr()?;
            others.push((op, rhs));
        }
        if others.is_empty() {
            Ok(first)
        } else {
            Ok(Expr::CmpOp(Box::new(first), others))
        }
    }

    left_binop_expr!(ior_expr, xor_expr,  Vbar, Ior);
    left_binop_expr!(xor_expr, and_expr, Caret, Xor);
    left_binop_expr!(and_expr, add_expr,   And, And);

    // AddExpr ::= MulExpr (("+" | "-") MulExpr)*   # w/ left assoc
    fn add_expr(&mut self) -> ParsRes<Expr>
    {
        let mut res = self.mul_expr()?;
        while matches_next!(self, Tok::Plus | Tok::Minus) {
            let op = self.read_token().unwrap(); // +, -
            let rhs = self.mul_expr()?;
            let op = match op.tok {
                Tok::Plus  => BinOp::Add,
                Tok::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            res = Expr::BinOp(Box::new(res), op, Box::new(rhs));
        }
        return Ok(ae);
    }

    rite_uniop_expr!(neg_expr, mul_expr, Minus, Neg);

    fn mul_expr(&mut self) -> StrRes<Expr>
    {
        let mut me = self.inv_expr()?;
        while self.matches(TokTyp::Asterisk)
           || self.matches(TokTyp::Slash)
           || self.matches(TokTyp::Bslash) {
            let op = self.read_token().unwrap().0; // *, /, \
            let rhs = self.inv_expr()?;
            let op = match op.typ() {
                TokTyp::Asterisk => BinOp::Mul,
                TokTyp::Slash    => BinOp::Div,
                TokTyp::Bslash   => BinOp::Mod,
                _ => unreachable!(),
            };
            me = Expr::BinOp(Box::new(me), op, Box::new(rhs));
        }
        return Ok(me);
    }

    rite_uniop_expr!(inv_expr,  not_expr,   Slash, Inv);
    rite_uniop_expr!(not_expr, cast_expr,   Tilde, Not);
//    left_binop_expr!(idx_expr,     nucle, Uscore, Idx);

    /*
    **  <CastEx> ::= <CastEx> % <Type>
    */
    fn cast_expr(&mut self) -> StrRes<Expr>
    {
        
    }*/

    /*
    **  <Type> ::= <Idf>
    ** // future: | (<Idf> "$")* <Idf>  // i.e. modules
    */

/*    fn fn_acc_ex(&mut self) -> StrRes<Expr>
    {
        let mut e = self.nucle()?;
        loop {
            let Some(t) = self.peek() else {
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
    }*/

    // Nucle ::= Literal | Ident | "(" Expr ")"
    fn nucle(&mut self) -> ParsRes<Expr>
    {
        self.push_rule(); // Nucle
        const MSG: &str = "(, identifier or literal"; // FIRST
        let Some(t) = self.read_token() else {
            return Err(self.error(MSG));
        };
        match t.tok {
//            TokTyp::Uscore =>     self.arrlit(),
//            TokTyp::Hash => self.func(tok.1),
//            TokTyp::BsLsb => self.if_expr(),
//            TokTyp::BsHash => self.short_fn(tok.1),
/*            TokTyp::RecF => {
                self.advance();
                Ok(Expr::RecFn)
            },*/
//            TokTyp::Bang => self.proc(tok.1),
            /*TokTyp::RecP => {
                self.advance();
                Ok(Expr::RecPc)
            },*/
/*            TokTyp::Dollar =>     self.tbllit(),
            TokTyp::RecT => {
                self.advance();
                Ok(Expr::RecsT(tok.0.as_rect().unwrap()))
            },*/
            Tok::Lparen => { // "(" Expr ")"
                let e = self.expr()?;
                exp_adv!(self, Tok::Rparen, ")")?;
                return Ok(self.node(Expr::Paren(Box::new(e))));
            },
            // Identifier
            Tok::Ident(id) => Ok(self.node(Expr::Ident(id))),
            // literals
            Tok::ValB(b) => Ok(self.node(Expr::Lit(Lit::B(b)))),
            Tok::ValC(c) => Ok(self.node(Expr::Lit(Lit::C(c)))),
            Tok::ValN(n) => Ok(self.node(Expr::Lit(Lit::N(n)))),
            Tok::ValZ(z) => Ok(self.node(Expr::Lit(Lit::Z(z)))),
            Tok::ValR(r) => Ok(self.node(Expr::Lit(Lit::R(r)))),
//            TokTyp::String =>  self.string(tok.0.as_string().unwrap()),
            _ => Err(self.error(MSG)),
        }
    }

/*    // parses comma separated exprs which end in a specific token
    // it also consumes þe end token, so no need to exp_adv after
    fn comma_ex(&mut self, end: TokTyp) -> StrRes<Vec<Expr>>
    {
        // check empty
        if self.try_adv(end) {
            return Ok(vec![]);
        }
        let comma_or_end = format!(", or {end:?}");
        let mut exs = vec![];
        loop {
            let ex = self.expr()?;
            exs.push(ex);
            let Some(tok) = self.peek() else {
                return eof_err!(comma_or_end);
            };
            if self.try_adv(end) {
                return Ok(exs);
            }
            if tok.0.typ() != TokTyp::Comma {
                return exp_err!(comma_or_end, tok);
            }
            self.advance();
        }
    }*/

/*    // called when _
    fn arrlit(&mut self) -> ParsRes<Expr>
    {
        self.advance(); // _
        let arr_e = self.comma_ex(TokTyp::Semic)?;
        return Ok(Expr::Array(arr_e));
    }*/

/*    // called when $
    fn tbllit(&mut self) -> StrRes<Expr>
    {
        const MSG: &str = "Ident or ;";
        self.advance(); // $
        let mut tbl_e = vec![];
        loop {
            let Some(t) = self.peek() else {
                return eof_err!(MSG);
            };
            match t.0.typ() {
                TokTyp::Ident => {}, // ok, continue reading
                TokTyp::Semic => break,
                _ => return exp_err!(MSG, t),
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
    }*/

/*    // called when #
    fn func(&mut self, line: usize) -> StrRes<Expr>
    {
        self.subr(line, SubrType::F)
    }*/

    // called when !
    /*
    **  <PcDef> ::= "!" <CommaEx(<Idf> "%" <Type>)>? "." <Block> "."
    */
/*    fn proc(&mut self, line: usize) -> StrRes<Expr>
    {
        self.subr(line, SubrType::P)
    }

    // helper for func & proc
    fn subr(&mut self, line: usize, st: SubrType) -> StrRes<Expr>
    {
        self.advance(); // # or !
        let name = match self.peek() { // FIXME: maybe use map?
            Some((t, _)) => t.as_string().map(
                |s| Rc::new(s.try_into().unwrap())
            ),
            None => None,
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
//            SubrType::F => Expr::FnDef(mrs),
            SubrType::P => Expr::PcDef(mrs),
            _ => todo!(),
        });
    }

    // matches (Ident (Comma Ident)*)? END
    fn pars(&mut self, end: TokTyp) -> StrRes<Vec<&[u8]>>
    {
        let mut res: Vec<&[u8]> = vec![];
        if self.matches(end) {
            self.advance();
            return Ok(res);
        }
        if let Ok(i) = self.consume_ident() {
            res.push(i);
        }
        while !self.matches(end) {
            self.exp_adv(TokTyp::Comma)?;
            let id = self.consume_ident()?;
            res.push(id);
        }
        self.advance(); // END
        return Ok(res);
    }*/

    // called when \#
/*    #[inline]
    fn short_fn(&mut self, line: usize) -> StrRes<Expr>
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
    }*/

    // called when \[
/*    #[inline]
    fn if_expr(&mut self) -> StrRes<Expr>
    {
        self.advance(); // \[
        let mut cases = vec![];
        loop {
            if self.matches(TokTyp::Then) {
                todo!("final else =>");
            }
            let e = self.expr()?;
            if !cases.is_empty() && self.matches(TokTyp::RsqBra) {
                self.advance(); // ]
                return Ok(Expr::IfExp(cases, Box::new(e)));
            }
            if self.exp_adv(TokTyp::Then).is_err() {
                let msg = if cases.is_empty() {"=>"} else {"=> or ]"};
                return exp_err!(msg, self.peek().unwrap());
            }
            let f = self.expr()?;
            self.exp_adv(TokTyp::Semic)?;
            cases.push((e, f));
        }
    }*/

/*    // expected identifier
    fn exp_ident(&mut self) -> ParsRes<SymId>
    {
        let Some(t) = self.peek() else {
            return self.error("identifier");
        };
        let Tok::Ident(i) = t.tok {
            return self.error("identifier");
        }
        self.advance(); // ident
        return Ok(i);
    }*/

/*    // called when curr tok is String
    fn string(&mut self, b: &[u8]) -> StrRes<Expr>
    {
        let a = Array::try_from(b)?;
        self.advance();
        return Ok(Expr::Const(Val::from_array(a)));
    }*/
}

/*impl TryFrom<Tok> for CmpOp
{
    type Error = ();
    fn try_from(t: Tok) -> Result<Self, ()>
    {
        match t {
            Tok::Equal2 => Ok(CmpOp::Equ(true)),
            Tok::Ne     => Ok(CmpOp::Equ(false)),
            Tok::Langle => Ok(CmpOp::Ord(OrdOp::Lt)),
            Tok::Le     => Ok(CmpOp::Ord(OrdOp::Le)),
            Tok::Rangle => Ok(CmpOp::Ord(OrdOp::Gt)),
            Tok::Ge     => Ok(CmpOp::Ord(OrdOp::Ge)),
            _ => Err(()),
        }
    }
}*/

/*impl TryFrom<TokTyp> for BinOp
{
    type Error = String;
    fn try_from(t: TokTyp) -> Result<Self, Self::Error>
    {
        match t {
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
*/
