/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What stands in the checked program for a function or constant that failed to check. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::cx::{ConstInfo, FnInfo};
use crate::compiler::sema::ty::Types;
use crate::compiler::source::Span;
use crate::compiler::tir::{Labels, TBlock, TConst, TExpr, TExprKind, TFn};

/** An expression that failed to check. */
fn error(span: Span) -> TExpr {
    let (kind, ty) = (TExprKind::Error, Types::ERROR);
    TExpr { kind, ty, span }
}

/** The function `info` declares, whose body failed to check. */
pub(super) fn failed_fn(info: &FnInfo) -> TFn {
    let span = info.decl.name.span;
    let tail = Some(Box::new(error(span)));
    TFn {
        def: info.def,
        name: info.decl.name.name.clone(),
        is_const: info.decl.is_const,
        params: Vec::new(),
        ret: Types::ERROR,
        ret_labels: Labels::default(),
        body: TBlock {
            stmts: Vec::new(),
            tail,
            span,
        },
        locals: Vec::new(),
        span,
    }
}

/** The constant `info` declares, which failed to check or evaluate. */
pub(super) fn failed_const(info: &ConstInfo) -> TConst {
    let span = info.decl.name.span;
    TConst {
        def: info.def,
        name: info.decl.name.name.clone(),
        ty: Types::ERROR,
        init: error(span),
        locals: Vec::new(),
        span,
    }
}
