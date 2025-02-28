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

pub fn semanalize(p: Prog) -> ProgWt
{
    return SemAnal::semanalize(p);
}

// private stuff:

type Idf2Typ = util::VecMap<IdfIdx, Type>;

#[derive(Default)]
struct SemAnal
{
    idfs: Vec<DfStr>,
    envs: util::Stack<Idf2Typ>, // all parent Envs (for declared vars)
    curr: Idf2Typ, // innermost Env
}

impl SemAnal
{
    pub fn semanalize(mut p: Prog) -> ProgWt
    {
        let mut sa = Self::default();
        sa.idfs = std::mem::take(&mut p.idents);
        let b = sa.p_block(p.main);
        return ProgWt{idents:sa.idfs, main:b};
    }

    // private stuff

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
            Stmt::IfElse(i, f, e) => self.p_s_ifelse(i, f, e),
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

    fn p_s_varass(&mut self, i: IdfIdx, e: ExprWt) -> StmtWt
    {
        /* On var shadowing:
        ** If `i` exists in self.curr, it overwrites `i` & its type.
        ** If it has different type, it's considered a Declar.
        ** If `i` exists in some prev env wiþ a different type,
        ** `i` is declared in self.curr, & þe prev `i` gets shadowed.
        ** If `i` exists & is of þe same type, it's a normal assign to `i`.
        **
        ** Þe case when `i` of same type exists on an even furþer scope,
        ** but þer's an `i` of diff type closer, þe 1st one is ignored,
        ** so `i` gets declared as a new var.
        */
        if let Some(t) = self.curr.get(&i) {
            // normal assign if same type
            if t == &e.t {
                return StmtWt::VarAss(i, e, 0); // assign to current level (0)
            } else {
                // update type
                self.curr.set(i, e.t.clone());
                return StmtWt::Declar(i, e);
            }
        }
        // now let's see if it exists in a parent scope
        for (depth, env) in self.envs.iter().enumerate() {
            if let Some(it) = env.get(&i) {
                // as soon as it finds one, return
                if it == &e.t {
                    return StmtWt::VarAss(i, e, depth+1); // +1 adding curr
                } else {
                    self.curr.set(i, e.t.clone());
                    return StmtWt::Declar(i, e);
                }
            }
        }
        // declar
        self.curr.set(i, e.t.clone());
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
        if !cd_wt.t.is_b() {
            panic!("condition in loop is not B%");
        }
        let b1_wt = self.no_env_block(b1);
        self.exit_scope();
        return LoopWt::Cdt(b0_wt, cd_wt, b1_wt);
    }

    fn p_s_ifelse(
        &mut self,
        if0: IfCase,
        eis: Vec<IfCase>,
        els: Option<Block>
    ) -> StmtWt
    {
        let if0_wt = self.p_ifcase(if0);
        let eis_wt = eis
            .into_iter()
            .map(|ic| self.p_ifcase(ic))
            .collect::<Vec<_>>();
        let els_wt = els.map(|b| self.p_block(b));
        return StmtWt::IfElse(if0_wt, eis_wt, els_wt);
    }

    fn p_ifcase(&mut self, cas: IfCase) -> IfCaseWt
    {
        let IfCase {cond:c, blok:b} = cas;
        let c_wt = self.p_expr(c);
        if !c_wt.t.is_b() {
            panic!("condition is not B");
        }
        let b_wt = self.p_block(b);
        return IfCaseWt{cond:c_wt, blok:b_wt};
    }

    fn p_expr(&mut self, e: Expr) -> ExprWt
    {
        match e {
            Expr::Const(v)       => Self::e_const(v),
            Expr::Ident(i)       => self.e_ident(i),
            Expr::UniOp(e, o)    => self.e_uniop(*e, o),
            Expr::BinOp(l, o, r) => self.e_binop(*l, o, *r),
            Expr::CmpOp(f, v)    => self.e_cmpop(*f, v),
            Expr::Array(a)       => self.e_array(a),
            _ => todo!(),
        }
    }

    fn e_const(c: Val) -> ExprWt
    {
        // TODO LBT, LN0, etc
        let t = Type::from(&c);
        return ExprWt{e:ExprWte::Const(c), t:t};
    }

    fn e_ident(&mut self, i: IdfIdx) -> ExprWt
    {
        if let Some(t) = self.curr.get(&i) {
            return ExprWt {e:ExprWte::Local(i, 0), t:t.clone()};
        }
        for (dep, env) in self.envs.iter().enumerate() {
            if let Some(t) = env.get(&i) {
                return ExprWt {e:ExprWte::Local(i, dep+1), t:t.clone()};
            }
        }
        // FUTURE: resolve also upval
        panic!("could not resolve ident {i}");
    }

    fn e_uniop(&mut self, e: Expr, o: UniOp) -> ExprWt
    {
        let e_wt = self.p_expr(e);
        let Type::Fund(e_ft) = &e_wt.t else {todo!("uniop overload");};
        let (o_wt, t) = uniop_types(e_ft, &o);
        return ExprWt {e:ExprWte::UniOp(Box::new(e_wt), o_wt), t:Type::Fund(t)};
    }

    fn e_binop(&mut self, l: Expr, o: BinOp, r: Expr) -> ExprWt
    {
        if o == BinOp::Typ {
            return self.e_tcast(l, r);
        }
        let l_wt = self.p_expr(l);
        let r_wt = self.p_expr(r);
        let Type::Fund(l_ft) = &l_wt.t else {todo!("binop overload");};
        let Type::Fund(r_ft) = &r_wt.t else {todo!("binop overload");};
        let (o_wt, t) = binop_types(l_ft, &o, r_ft);
        return ExprWt {
            e: ExprWte::BinOp(Box::new(l_wt), o_wt, Box::new(r_wt)),
            t: t.into(),
        };
    }

    fn e_tcast(&mut self, ex: Expr, ty: Expr) -> ExprWt
    {
        let ex_wt = self.p_expr(ex);
        let typ: Type = match ty {
            Expr::Ident(i) => match self.idfs[i].as_bytes() {
                b"N" => FundTy::N.into(),
                b"Z" => FundTy::Z.into(),
                b"R" => FundTy::R.into(),
                _ => todo!(),
            }
            _ => panic!(),
        };
        return ExprWt {e:ExprWte::Tcast(Box::new(ex_wt), typ.clone()), t:typ};
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
        let Type::Fund(f_ft) = &f_wt.t else {todo!("<=>= overload");};
        let o_wt = cmpop_types(f_ft, &o);
        return ExprWt{
            e:ExprWte::CmpOp(Box::new(f_wt), vec![(o_wt, g_wt)]),
            t:FundTy::B.into(),
        };
    }

    fn e_array(&mut self, a: Vec<Expr>) -> ExprWt
    {
        if a.is_empty() {
            todo!("empty arrays");
        }
        let mut a_type = None;
        let mut a_wt = vec![];
        for x in a {
            let x_wt = self.p_expr(x);
            if let Some(t) = &a_type {
                if t != &x_wt.t {
                    panic!("different type elements in array");
                }
            } else { // 0st elem defines þe whole arr
                a_type = Some(x_wt.t.clone());
            }
            a_wt.push(x_wt);
        }
        return ExprWt{e:ExprWte::Array(a_wt), t:a_type.unwrap()};
    }
}

fn binop_types(lt: &FundTy, op: &BinOp, rt: &FundTy) -> (BinOpWt, FundTy)
{
    // BEWARE OF ÞE NASTY MACRO!
    macro_rules! binop {
        ($lt:expr, $rt:expr, $opexpr:expr, $($op:ident,
            $($ltt:ident, $rtt:ident => $resop:ident, $resty:ident,)+;)+) => {
            match $opexpr {
                $(BinOp::$op => match ($lt, $rt) {
                    $((FundTy::$ltt, FundTy::$rtt) =>
                        return (BinOpWt::$resop, FundTy::$resty),)+
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

fn uniop_types(t: &FundTy, o: &UniOp) -> (UniOpWt, FundTy)
{
    // no nasty macro since þer'r too few of þem
    match o {
        UniOp::Neg => match t {
            FundTy::Z => return (UniOpWt::NEZ, FundTy::Z),
            FundTy::R => return (UniOpWt::NER, FundTy::R),
            _ => {}
        },
        UniOp::Inv => match t {
            FundTy::R => return (UniOpWt::INR, FundTy::R),
            _ => {}
        },
        UniOp::Not => match t {
            FundTy::B => return (UniOpWt::NOB, FundTy::B),
            FundTy::C => return (UniOpWt::NOC, FundTy::C),
            FundTy::N => return (UniOpWt::NON, FundTy::N),
            _ => {}
        },
    }
    // any oþer combination, error
    panic!("unknown op: {o:?} {t}");
}

fn cmpop_types(t: &FundTy, o: &CmpOp) -> CmpOpWt
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

impl TryFrom<&FundTy> for EquTyp
{
    type Error = ();
    fn try_from(t: &FundTy) -> Result<Self, ()>
    {
        match t {
            FundTy::B => Ok(Self::B),
            FundTy::C => Ok(Self::C),
            FundTy::N => Ok(Self::N),
            FundTy::Z => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

impl TryFrom<&FundTy> for OrdTyp
{
    type Error = ();
    fn try_from(t: &FundTy) -> Result<Self, ()>
    {
        match t {
            FundTy::C => Ok(Self::C),
            FundTy::N => Ok(Self::N),
            FundTy::Z => Ok(Self::Z),
            FundTy::R => Ok(Self::R),
            _ => Err(()),
        }
    }
}
