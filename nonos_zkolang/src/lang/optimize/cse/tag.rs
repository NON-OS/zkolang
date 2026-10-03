/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The operator a node applies, as part of its structural key. */

use crate::lang::parse::Expr;

/** Which operator a node applies, for the nodes keyed by operator and children. */
pub(super) fn tag(e: &Expr) -> u8 {
    match e {
        Expr::Add(..) => 0,
        Expr::Sub(..) => 1,
        Expr::Mul(..) => 2,
        Expr::Div(..) => 3,
        Expr::Eq(..) => 4,
        Expr::Ne(..) => 5,
        Expr::Lt(..) => 6,
        Expr::Neg(..) => 7,
        Expr::Inv(..) => 8,
        Expr::Sel(..) => 9,
        Expr::If(..) => 10,
        Expr::Array(..) => 11,
        Expr::Tuple(..) => 12,
        Expr::Num(..) | Expr::Var(..) | Expr::Call(..) | Expr::Index(..) | Expr::Block(..) => 13,
    }
}
