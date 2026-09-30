/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The slots of a function's parameters: each scalar part of each parameter, in order, is
 * one slot, and an array's elements share the slots of one element, as they share labels.
 * A summary is written in slots, so a call can give each part of an argument its own
 * label.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** A value of type `ty` whose every part is its own slot, the first being `next`. */
pub(super) fn numbered(ty: TyId, next: &mut usize, types: &Types) -> Shape {
    if let Some(ts) = types.record(ty) {
        return Shape::Tuple(ts.iter().map(|&t| numbered(t, next, types)).collect());
    }
    match types.kind(ty) {
        TyKind::Array(e, _) => Shape::Array(Box::new(numbered(*e, next, types))),
        _ => {
            let t = Taint::slot(*next);
            *next = next.saturating_add(1);
            Shape::Leaf(t)
        }
    }
}

/** The labels of the parts of `v`, of type `ty`, in the order `numbered` gives slots. */
pub(super) fn flatten(v: &Shape, ty: TyId, types: &Types, out: &mut Vec<Taint>) {
    if let Some(ts) = types.record(ty) {
        for (i, &t) in ts.iter().enumerate() {
            flatten(&v.child(i), t, types, out);
        }
        return;
    }
    match types.kind(ty) {
        TyKind::Array(e, _) => flatten(&v.elem(), *e, types, out),
        _ => out.push(v.all()),
    }
}
