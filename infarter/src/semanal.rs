/* semanal.rs */

use crate::{
    asterix::*,
    dflib::tables::NatTb,
};

pub fn check(b: &mut Block)
{
    for s in b {
        check_stmt(s);
    }
}

fn check_stmt(s: &mut Stmt)
{
    match &mut *s {
        Stmt::Assign(x, e) => {check_expr(x); check_expr(e);},
        Stmt::IfStmt(c, b, e) => {
            check_expr(c);
            check(b);
            if let Some(m) = e { check(m); }
        },
        Stmt::PcCall(p, a) => {
            check_expr(p);
            for e in a {
                check_expr(e);
            }
        },
        _ => {},
    }
}

fn check_expr(e: &mut Expr)
{
    match e {
        Expr::Ident(i) => if i == "STD" {
            *e = Expr::Const(Val::from(NatTb::STD));
        },
        Expr::Tcast(_, b) => check_expr(b),
        Expr::BinOp(f, _, g) => {check_expr(f); check_expr(g);},
        Expr::TblFd(t, _) => check_expr(t),
        _ => {}
    }
}
