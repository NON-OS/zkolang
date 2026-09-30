/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The constants and functions an expression names. */

use alloc::vec::Vec;

use crate::compiler::tir::{ConstId, FnId, TExpr, TExprKind};

/** The constants and functions `e` names, anywhere in it. */
pub(super) fn collect(e: &TExpr, out: &mut (Vec<ConstId>, Vec<FnId>)) {
    match &e.kind {
        TExprKind::Const(c) => out.0.push(*c),
        TExprKind::Call(f, _) => out.1.push(*f),
        _ => {}
    }
    e.each_child(&mut |c| collect(c, out));
}
