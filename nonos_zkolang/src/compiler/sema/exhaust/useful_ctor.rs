/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Usefulness where the first pattern is built with a constructor: for a range, each piece
 * of it the rows tell apart; the rows that take values built so are specialized to them.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::domain::parts;
use super::matrix::{heads, specialize};
use super::pat::{Ctor, Pat};
use super::split::cut;
use super::useful::{prepend, Cx, Found};
use crate::compiler::sema::ty::TyId;

impl Cx<'_> {
    /** Values that `c(subs)`, then `rest`, matches and no row does. */
    pub(super) fn ctor_head(
        &mut self,
        rows: &[Vec<Pat>],
        (c, subs): (Ctor, &[Pat]),
        rest: &[Pat],
        tys: (TyId, &[TyId]),
    ) -> Found {
        let pieces = match c {
            Ctor::Range(a, b) => cut(a, b, &heads(rows)).into_iter().map(|p| p.0).collect(),
            _ => vec![c],
        };
        let v: Vec<Pat> = subs.iter().chain(rest).cloned().collect();
        for piece in pieces {
            if let Some(found) = self.with_ctor(rows, piece, &v, tys)? {
                return Ok(Some(found));
            }
        }
        Ok(None)
    }

    /** Values built with `c` that `v`, the parts of `c` then the rest, matches and no row does. */
    pub(super) fn with_ctor(
        &mut self,
        rows: &[Vec<Pat>],
        c: Ctor,
        v: &[Pat],
        (ty, rest): (TyId, &[TyId]),
    ) -> Found {
        let tys: Vec<TyId> = parts(self.types, ty, c)
            .into_iter()
            .chain(rest.iter().copied())
            .collect();
        let arity = tys.len() - rest.len();
        let found = self.useful(&specialize(rows, c, arity), v, &tys)?;
        Ok(found.map(|w| {
            let (subs, rest) = w.split_at(arity.min(w.len()));
            prepend(Pat::Ctor(c, subs.to_vec()), rest)
        }))
    }
}
