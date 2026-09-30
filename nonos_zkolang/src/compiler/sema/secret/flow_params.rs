/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The labels a function's parameters start with. An unlabelled part is its own slot; a
 * `public` part is public, and callers must pass it a public value; a `secret` part is
 * secret. `main`'s parameters are labelled, so they have no slots.
 */

use super::flow::Flow;
use super::shape::Shape;
use super::shape_path::update;
use super::slots::numbered;
use super::taint::Taint;
use crate::compiler::syntax::ast::Label;

impl<'p> Flow<'p> {
    /** Give each parameter its labels; the slots whose arguments must be public. */
    pub(super) fn params(&mut self, is_main: bool) -> u128 {
        let (f, program) = (self.f, self.program);
        let types = &program.types;
        let (mut needs, mut next) = (0u128, 0usize);
        for p in &f.params {
            let Some(local) = f.locals.get(p.local.0 as usize) else {
                continue;
            };
            let mut shape = match is_main {
                true => Shape::of(local.ty, Taint::PUBLIC, types),
                false => numbered(local.ty, &mut next, types),
            };
            for (path, label) in &local.labels.0 {
                shape = update(shape, local.ty, path, types, &mut |part, ty| match label {
                    Label::Public => {
                        needs |= part.all().slots;
                        Shape::of(ty, Taint::PUBLIC, types)
                    }
                    Label::Secret => part.raised(Taint::SECRET),
                });
            }
            if let Some(slot) = self.env.get_mut(p.local.0 as usize) {
                *slot = shape.clone();
            }
            self.bind(&p.pat, shape, local.ty, local.span);
        }
        needs
    }

    /** The labels a `&mut` parameter's local may end with: its last, or one at a `return`. */
    pub(super) fn ref_final(&self, local: usize) -> Shape {
        let last = self
            .env
            .get(local)
            .cloned()
            .unwrap_or(Shape::Leaf(Taint::SECRET));
        match self.ref_ret.iter().find(|(l, _)| *l == local) {
            Some((_, seen)) => last.join(seen),
            None => last,
        }
    }
}
