/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Tuples and arrays (section 7.11): their slots in order, and a field's slots among them. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use super::layout::{field_at, slots};
use crate::compiler::ssa::V;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'p> Lower<'p> {
    /** The slots of the tuple, struct literal, array, repeat or field `e`. */
    pub(super) fn compound(&mut self, e: &TExpr) -> L<Vec<V>> {
        Ok(match &e.kind {
            TExprKind::Tuple(es) | TExprKind::Array(es) => {
                let mut out = Vec::new();
                for x in es {
                    out.extend(self.expr(x)?);
                }
                out
            }
            TExprKind::Record(fs) => {
                let n = self.p.types.record(e.ty).map_or(0, |ts| ts.len());
                let mut parts = alloc::vec![Vec::new(); n];
                for (i, x) in fs {
                    let v = self.expr(x)?;
                    if let Some(p) = parts.get_mut(*i as usize) {
                        *p = v;
                    }
                }
                parts.concat()
            }
            TExprKind::Repeat(a, n) => {
                let v = self.expr(a)?;
                (0..*n).flat_map(|_| v.iter().copied()).collect()
            }
            TExprKind::TupleField(a, i) => {
                let v = self.expr(a)?;
                let (at, t) = field_at(&self.p.types, a.ty, *i)
                    .ok_or(LowerError::Unsupported("a tuple field", e.span))?;
                let n = slots(&self.p.types, t);
                v.get(at..at + n).unwrap_or(&[]).to_vec()
            }
            _ => Vec::new(),
        })
    }
}
