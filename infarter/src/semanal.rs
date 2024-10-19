/* semanal.rs */

use std::rc::Rc;
use crate::{
    asterix::*,
//    dflib::tables::NatTb,
    util,
    util::DfStr,
};

/* semantic analysis: type check & ident resolv (incl. upvals) */

pub fn semanalize(b: Block) -> BlockWt
{
    let mut a = SemAnal::default();
    return a.p_block(b);
}

// private stuff:

type Idf2Typ = util::VecMap<Rc<DfStr>, Type>;

#[derive(Default)]
struct SemAnal
{
//    envs: util::Stack<Idf2Typ>, // all parent Envs (for declared vars)
    curr: Idf2Typ, // innermost Env
}

impl SemAnal
{
    fn get_idf_typ(&self, i: &Rc<DfStr>) -> Option<&Type>
    {
        return self.curr.get(i);
    }

/*    fn exists_idf(&self, i: &Rc<DfStr>) -> bool
    {
        return self.curr.has(i);
    }*/

    // GRAMMAR

    pub fn p_block(&mut self, b: Block) -> BlockWt
    {
        return b.into_iter().map(|s| self.p_stmt(s)).collect();
    }

    pub fn p_stmt(&mut self, s: Stmt) -> StmtWt
    {
        match s {
            Stmt::Assign(a, e) => self.p_s_assign(a, e),
        }
    }

    fn p_s_assign(&mut self, a: Expr, e: Expr) -> StmtWt
    {
        let e_wt = self.p_expr(e);
        match a {
            Expr::Ident(i) => self.p_s_varass(i, e_wt),
            _ => todo!("oþer assigns"),
        }
    }

    fn p_s_varass(&mut self, i: Rc<DfStr>, e: ExprWt) -> StmtWt
    {
        // FUTURE: var shadowing if different types
        // if it didn't exists or existed wiþ a diff type,
        // treat þe assign as a declar
        self.curr.set(i.clone(), e.t.clone());
        return StmtWt::VarAss(i, e);
    }

    fn p_expr(&mut self, e: Expr) -> ExprWt
    {
        match e {
            Expr::Const(v)       => Self::e_const(v),
            Expr::Ident(i)       => self.e_ident(i),
            Expr::UniOp(e, o)    => self.e_uniop(*e, o),
            Expr::BinOp(l, o, r) => self.e_binop(*l, o, *r),
            _ => todo!(),
        }
    }

    fn e_const(c: Val) -> ExprWt
    {
        let t = Type::from(&c);
        return ExprWt{e:ExprWte::Const(c), t:t};
    }

    fn e_ident(&mut self, i: Rc<DfStr>) -> ExprWt
    {
        // FUTURE: resolve also upval, etc.
        let t = self.curr.get(&i)
            .expect("could not resolve ident {&i}")
            .clone();
        return ExprWt {e:ExprWte::Ident(i), t:t}
    }

    fn e_uniop(&mut self, e: Expr, o: UniOp) -> ExprWt
    {
        let e_wt = self.p_expr(e);
        let (o_wt, t) = uniop_types(&e_wt.t, &o);
        return ExprWt {e:ExprWte::UniOp(Box::new(e_wt), o_wt), t:t};
    }

    fn e_binop(&mut self, l: Expr, o: BinOp, r: Expr) -> ExprWt
    {
        let l_wt = self.p_expr(l);
        let r_wt = self.p_expr(r);
        let (o_wt, t) = binop_types(&l_wt.t, &o, &r_wt.t);
        return ExprWt {
            e: ExprWte::BinOp(Box::new(l_wt), o_wt, Box::new(r_wt)),
            t: t,
        };
    }
}

pub fn binop_types(lt: &Type, op: &BinOp, rt: &Type) -> (BinOpWt, Type)
{
    // BEWARE OF ÞE NASTY MACRO!
    macro_rules! binop {
        ($lt:expr, $rt:expr, $opexpr:expr, $($op:ident,
            $($ltt:ident, $rtt:ident => $resop:ident, $resty:ident,)+;)+) => {
            match $opexpr {
                $(BinOp::$op => match ($lt, $rt) {
                    $((Type::$ltt, Type::$rtt) =>
                        return (BinOpWt::$resop, Type::$resty),)+
                    _ => {},
                },)+
                _ => {},
            }
        }
    }
    // looks nicer wiþ an example, init?
    binop!(lt, rt, op,
        Add,
            C, C => ADC, C,
            N, N => ADN, N,
            Z, Z => ADZ, Z,
            R, R => ADR, R,;
        Sub,
            Z, Z => SUZ, Z,
            R, R => SUR, R,;
        Mul,
            C, C => MUC, C,
            N, N => MUN, N,
            Z, Z => MUZ, Z,
            R, R => MUR, R,;
        Div,
            N, N => DIN, N,
            R, R => DIR, R,;
        Mod,
            C, C => MOC, C,
            N, N => MON, N,
            Z, N => MOZ, N,;
        And,
            B, B => ANB, B,
            C, C => ANC, C,
            N, N => ANN, N,;
        Ior,
            B, B => IOB, B,
            C, C => IOC, C,
            N, N => ION, N,;
        Xor,
            B, B => XOB, B,
            C, C => XOC, C,
            N, N => XON, N,;
    );
    panic!("Unknown operation: {lt} {op:?} {rt}");
}

pub fn uniop_types(t: &Type, o: &UniOp) -> (UniOpWt, Type)
{
    // no nasty macro since þer'r too few of þem
    match o {
        UniOp::Neg => match t {
            Type::Z => return (UniOpWt::NEZ, Type::Z),
            Type::R => return (UniOpWt::NER, Type::R),
            _ => {}
        },
        UniOp::Inv => match t {
            Type::R => return (UniOpWt::INR, Type::R),
            _ => {}
        },
        UniOp::Not => match t {
            Type::B => return (UniOpWt::NOB, Type::B),
            Type::C => return (UniOpWt::NOC, Type::C),
            Type::N => return (UniOpWt::NON, Type::N),
            _ => {}
        },
    }
    // any oþer combination, error
    panic!("unknown op: {o:?} {t}");
}

