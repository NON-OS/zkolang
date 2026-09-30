/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A value meeting the labels written where it goes (section 13.2): each `public` part must
 * be public, and each `secret` part becomes secret. A `public` part is public from there
 * on: where it was not, that is reported, or required of every caller, once.
 */

use alloc::vec::Vec;

use super::flow::Flow;
use super::shape::Shape;
use super::shape_path::update;
use super::taint::Taint;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Label;

impl<'p> Flow<'p> {
    /** `v`, of type `ty`, as it stands after meeting `labels`, from the value at `at`. */
    pub(super) fn labelled(
        &mut self,
        v: Shape,
        labels: &[(Vec<u32>, Label)],
        ty: TyId,
        at: Span,
    ) -> Shape {
        let mut v = v;
        for (path, label) in labels {
            v = self.meet(v, path, *label, ty, at);
        }
        v
    }

    fn meet(&mut self, v: Shape, path: &[u32], label: Label, ty: TyId, at: Span) -> Shape {
        let types = &self.program.types;
        let mut needs = Taint::PUBLIC;
        let out = update(v, ty, path, types, &mut |part, t| match label {
            Label::Public => {
                needs = needs.join(part.all());
                Shape::of(t, Taint::PUBLIC, types)
            }
            Label::Secret => part.raised(Taint::SECRET),
        });
        self.require_public(needs, at, "a place typed `public`");
        out
    }
}
