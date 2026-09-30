/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Places (section 8.2): a local and a path into it, whose indices are evaluated once. A
 * constant index names its element; a run-time index is checked in bounds and selects
 * every element by whether it is the one, reading by selection and writing each element
 * under the guard and the index's equality (section 7.6).
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::ssa::V;
use crate::compiler::tir::{Proj, TExprKind, TLit, TPlace};

/** One evaluated step of a place's path. */
#[derive(Clone, Copy, Debug)]
pub(super) enum Step {
    Field(u32),
    At(usize),
    /** A run-time index, already checked in bounds. */
    Dyn(V),
}

impl<'p> Lower<'p> {
    /** The steps of `p`, each index evaluated and checked. */
    pub(super) fn steps(&mut self, p: &TPlace) -> L<Vec<Step>> {
        let mut out = Vec::with_capacity(p.proj.len());
        let mut ty = self.local_ty(p.root);
        for step in &p.proj {
            let (s, next) = match step {
                Proj::TupleField(i) => {
                    let (_, t) = super::layout::field_at(&self.p.types, ty, *i)
                        .ok_or(LowerError::Unsupported("a tuple field", p.span))?;
                    (Step::Field(*i), t)
                }
                Proj::Index(e) => {
                    let (el, _, n) = super::layout::elements(&self.p.types, ty)
                        .ok_or(LowerError::Unsupported("an index", p.span))?;
                    match e.kind {
                        TExprKind::Lit(TLit::Int(k)) => (Step::At(k as usize), el),
                        _ => {
                            let i = self
                                .expr(e)?
                                .first()
                                .copied()
                                .ok_or(LowerError::Unsupported("an index", e.span))?;
                            self.check_index(i, n);
                            (Step::Dyn(i), el)
                        }
                    }
                }
            };
            out.push(s);
            ty = next;
        }
        Ok(out)
    }
}
