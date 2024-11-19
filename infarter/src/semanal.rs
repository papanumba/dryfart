/* semanal.rs */

use std::{
    rc::Rc,
    mem,
};
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
    envs: util::Stack<Idf2Typ>, // all parent Envs (for declared vars)
    curr: Idf2Typ, // innermost Env
}

impl SemAnal
{
/*    fn get_idf_typ(&self, i: &Rc<DfStr>) -> Option<&Type>
    {
        match self.curr.get(i) {
            Some(t) => Some((0, t)),
            None => self.envs
                .iter()              // from top
                .find(|e| e.has(i))?
                .get(i),
        }
    }*/

/*    fn exists_idf(&self, i: &Rc<DfStr>) -> bool
    {
        return self.curr.has(i);
    }*/

    fn init_scope(&mut self)
    {
        self.envs.push(mem::take(&mut self.curr));
    }

    fn exit_scope(&mut self)
    {
        self.curr = self.envs.pop().unwrap();
    }

    // GRAMMAR

    pub fn no_env_block(&mut self, b: Block) -> BlockWt
    {
        return b
            .into_iter()
            .map(|s| self.p_stmt(s))
            .collect();
    }

    pub fn p_block(&mut self, b: Block) -> BlockWt
    {
        self.init_scope();
        let b_wt = self.no_env_block(b);
        self.exit_scope();
        return b_wt;
    }

    pub fn p_stmt(&mut self, s: Stmt) -> StmtWt
    {
        match s {
            Stmt::Assign(a, e) => self.p_s_assign(a, e),
            Stmt::Loooop(l) => self.p_s_loooop(l),
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
        /* On var shadowing:
        ** If `i` exists in self.curr, it overwrites `i` & its type.
        ** If `i` exists in some prev env wiþ a different type,
        ** `i` is declared in self.curr, & þe prev `i` gets shadowed.
        ** If `i` exists & is of þe same type, it's a normal assign to `i`.
        **
        ** Þe case when `i` of same type exists on an even furþer scope,
        ** but þer's an `i` of diff type closer, þe 1st one is ignored,
        ** so `i` gets declared as a new var.
        */
        if self.curr.has(&i) {
            // normal assign, overwriting type
            self.curr.set(i.clone(), e.t.clone());
            return StmtWt::VarAss(i, e, 0); // assign to current level (0)
        }
        // now let's see if it exists in a parent scope
        for (depth, env) in self.envs.iter().enumerate() {
            if let Some(it) = env.get(&i) {
                // as soon as it finds one, return
                if it == &e.t {
                    return StmtWt::VarAss(i, e, depth+1); // +1 adding curr
                } else {
                    self.curr.set(i.clone(), e.t.clone());
                    return StmtWt::Declar(i, e);
                }
            }
        }
        // declar
        self.curr.set(i.clone(), e.t.clone());
        return StmtWt::Declar(i, e);
    }

    fn p_s_loooop(&mut self, l: Loop) -> StmtWt
    {
        //self.init_scope(); // for preloads
        let res = match l {
            Loop::Inf(b) => self.p_s_loop_inf(b),
            Loop::Cdt(a, c, b) => self.p_s_loop_cdt(a, c, b),
        };
        //self.exit_scope();
        return StmtWt::Loooop(res);
    }

    fn p_s_loop_inf(&mut self, b: Block) -> LoopWt
    {
        LoopWt::Inf(self.p_block(b))
    }

    fn p_s_loop_cdt(&mut self, b0: Block, cd: Expr, b1: Block) -> LoopWt
    {
        self.init_scope();
        let b0_wt = self.no_env_block(b0);
        let cd_wt = self.p_expr(cd);
        if cd_wt.t != Type::B {
            panic!("condition in loop is not B%");
        }
        let b1_wt = self.no_env_block(b1);
        self.exit_scope();
        return LoopWt::Cdt(b0_wt, cd_wt, b1_wt);
    }

    fn p_expr(&mut self, e: Expr) -> ExprWt
    {
        match e {
            Expr::Const(v)       => Self::e_const(v),
            Expr::Ident(i)       => self.e_ident(i),
            Expr::UniOp(e, o)    => self.e_uniop(*e, o),
            Expr::BinOp(l, o, r) => self.e_binop(*l, o, *r),
            Expr::CmpOp(f, v)    => self.e_cmpop(*f, v),
            _ => todo!(),
        }
    }

    fn e_const(c: Val) -> ExprWt
    {
        // TODO LBT, LN0, etc
        let t = Type::from(&c);
        return ExprWt{e:ExprWte::Const(c), t:t};
    }

    fn e_ident(&mut self, i: Rc<DfStr>) -> ExprWt
    {
        if let Some(t) = self.curr.get(&i) {
            return ExprWt {e:ExprWte::Local(i, 0), t:*t};
        }
        for (dep, env) in self.envs.iter().enumerate() {
            if let Some(t) = env.get(&i) {
                return ExprWt {e:ExprWte::Local(i, dep+1), t:*t};
            }
        }
        // FUTURE: resolve also upval
        panic!("could not resolve ident {i}");
    }

    fn e_uniop(&mut self, e: Expr, o: UniOp) -> ExprWt
    {
        let e_wt = self.p_expr(e);
        let (o_wt, t) = uniop_types(&e_wt.t, &o);
        return ExprWt {e:ExprWte::UniOp(Box::new(e_wt), o_wt), t:t};
    }

    fn e_binop(&mut self, l: Expr, o: BinOp, r: Expr) -> ExprWt
    {
        if o == BinOp::Typ {
            return self.e_tcast(l, r);
        }
        let l_wt = self.p_expr(l);
        let r_wt = self.p_expr(r);
        let (o_wt, t) = binop_types(&l_wt.t, &o, &r_wt.t);
        return ExprWt {
            e: ExprWte::BinOp(Box::new(l_wt), o_wt, Box::new(r_wt)),
            t: t,
        };
    }

    fn e_tcast(&mut self, ex: Expr, ty: Expr) -> ExprWt
    {
        let ex_wt = self.p_expr(ex);
        let typ = match ty {
            Expr::Ident(i) => match i.as_bytes() {
                b"R" => Type::R,
                _ => todo!(),
            }
            _ => panic!(),
        };
        return ExprWt {e:ExprWte::Tcast(Box::new(ex_wt), typ), t:typ};
    }

    fn e_cmpop(&mut self, f: Expr, v: Vec<(CmpOp, Expr)>) -> ExprWt
    {
        match v.len() {
            0 => return self.p_expr(f),
            1 => {}, // simple CMP
            _ => todo!("multi CMP"),
        }
        // multiple cmp
        let f_wt = self.p_expr(f);
        let (o, g) = v.into_iter().nth(0).unwrap();
        let g_wt = self.p_expr(g);
        if &f_wt.t != &g_wt.t {
            panic!("cannot use {:?} with different types", &o);
        }
        let o_wt = cmpop_types(&f_wt.t, &o);
        return ExprWt{
            e:ExprWte::CmpOp(Box::new(f_wt), vec![(o_wt, g_wt)]),
            t:Type::B
        };
    }
}


fn binop_types(lt: &Type, op: &BinOp, rt: &Type) -> (BinOpWt, Type)
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

fn uniop_types(t: &Type, o: &UniOp) -> (UniOpWt, Type)
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

fn cmpop_types(t: &Type, o: &CmpOp) -> CmpOpWt
{
    match o {
        CmpOp::Equ(b) => CmpOpWt::Equ(EquOpWt(*b, t.try_into()
            .expect(&format!("{t} is cannot be Equals compared"))
        )),
        CmpOp::Ord(o) => CmpOpWt::Ord(OrdOpWt(*o, t.try_into()
            .expect(&format!("{t} is cannot be Order compared"))
        )),
    }
}

impl TryFrom<&Type> for EquTyp
{
    type Error = ();
    fn try_from(t: &Type) -> Result<Self, ()>
    {
        match t {
            Type::B => Ok(Self::B),
            Type::C => Ok(Self::C),
            Type::N => Ok(Self::N),
            Type::Z => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Type> for OrdTyp
{
    type Error = ();
    fn try_from(t: &Type) -> Result<Self, ()>
    {
        match t {
            Type::C => Ok(Self::C),
            Type::N => Ok(Self::N),
            Type::Z => Ok(Self::Z),
            Type::R => Ok(Self::R),
            _ => Err(()),
        }
    }
}
