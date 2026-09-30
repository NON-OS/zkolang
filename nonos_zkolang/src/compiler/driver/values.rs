/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Typed values rebuilt from leaf values, and taken apart into them, for the reference run. */

use alloc::vec::Vec;

use crate::compiler::interp::Value;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::Label;
use crate::compiler::tir::{FnId, TProgram};

/** The arguments of `f` from its public and secret leaf values. */
pub(super) fn args_of(p: &TProgram, f: FnId, public: &[i128], secret: &[i128]) -> Vec<Value> {
    let (mut pubs, mut secs) = (public.iter().copied(), secret.iter().copied());
    let Some(body) = p.fns.get(f.0 as usize) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(body.params.len());
    for param in &body.params {
        let Some(l) = body.locals.get(param.local.0 as usize) else {
            continue;
        };
        let it: &mut dyn Iterator<Item = i128> = match l.labels.whole() {
            Some(Label::Secret) => &mut secs,
            _ => &mut pubs,
        };
        out.push(value_of(&p.types, l.ty, it));
    }
    out
}

/** The value of type `t` whose leaves come next from `it`. */
fn value_of(types: &Types, t: TyId, it: &mut dyn Iterator<Item = i128>) -> Value {
    if let Some(ts) = types.record(t) {
        return Value::Tuple(ts.iter().map(|&e| value_of(types, e, it)).collect());
    }
    match types.kind(t) {
        TyKind::Bool => Value::Bool(it.next().unwrap_or(0) == 1),
        TyKind::Field => Value::Field(u64::try_from(it.next().unwrap_or(0)).unwrap_or(0)),
        TyKind::Int(_) => Value::Int(it.next().unwrap_or(0)),
        TyKind::Array(e, n) => Value::Array((0..*n).map(|_| value_of(types, *e, it)).collect()),
        _ => Value::Unit,
    }
}

/** The leaf values of `v`, in order. */
pub(super) fn leaves_of(v: &Value, out: &mut Vec<i128>) {
    match v {
        Value::Unit => {}
        Value::Bool(b) => out.push(i128::from(*b)),
        Value::Int(i) => out.push(*i),
        Value::Field(f) => out.push(i128::from(*f)),
        Value::Tuple(vs) | Value::Array(vs) => vs.iter().for_each(|e| leaves_of(e, out)),
    }
}
