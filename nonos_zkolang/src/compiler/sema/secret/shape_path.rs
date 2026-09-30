/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Changing the part of a value's labels that a label's path names. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::tir::Labels;

/** `v`, of type `ty`, with the part at `path` replaced by `f` of it and its type. */
pub(super) fn update(
    v: Shape,
    ty: TyId,
    path: &[u32],
    types: &Types,
    f: &mut dyn FnMut(Shape, TyId) -> Shape,
) -> Shape {
    let Some((&step, rest)) = path.split_first() else {
        return f(v, ty);
    };
    match (types.kind(ty), step) {
        (TyKind::Array(el, _), Labels::ELEMENT) => {
            Shape::Array(Box::new(update(v.elem(), *el, rest, types, f)))
        }
        (_, i) if types.record(ty).is_some() => {
            let ts = types.record(ty).unwrap_or_default();
            let mut parts: Vec<Shape> = (0..ts.len()).map(|j| v.child(j)).collect();
            if let (Some(p), Some(&t)) = (parts.get_mut(i as usize), ts.get(i as usize)) {
                let old = core::mem::replace(p, Shape::Leaf(Taint::PUBLIC));
                *p = update(old, t, rest, types, f);
            }
            Shape::Tuple(parts)
        }
        _ => v,
    }
}
