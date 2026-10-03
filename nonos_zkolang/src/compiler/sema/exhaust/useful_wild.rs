/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Usefulness where the first pattern is `_`. If the rows' constructors cover every one of
 * the type, each is tried in turn; if not, a missing one takes the values that only the
 * rows starting with `_` may match, and names them in the witness.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::domain::{domain, parts, Domain};
use super::matrix::{default, heads};
use super::pat::Pat;
use super::split::split;
use super::useful::{prepend, Cx, Found};
use crate::compiler::sema::ty::TyId;

impl Cx<'_> {
    /** Values that `_` then `rest` matches and no row does. */
    pub(super) fn wild(
        &mut self,
        rows: &[Vec<Pat>],
        rest: &[Pat],
        (ty, rest_tys): (TyId, &[TyId]),
    ) -> Found {
        let d = domain(self.types, ty);
        if d == Domain::Opaque {
            return Ok(None);
        }
        let hs = heads(rows);
        let pieces = split(d, &hs);
        let Some(missing) = pieces.iter().find(|p| !p.1).map(|p| p.0) else {
            for (c, _) in pieces {
                let wilds = vec![Pat::Wild; parts(self.types, ty, c).len()];
                let v: Vec<Pat> = wilds.into_iter().chain(rest.iter().cloned()).collect();
                if let Some(found) = self.with_ctor(rows, c, &v, (ty, rest_tys))? {
                    return Ok(Some(found));
                }
            }
            return Ok(None);
        };
        let Some(w) = self.useful(&default(rows), rest, rest_tys)? else {
            return Ok(None);
        };
        let head = match (hs.is_empty(), d) {
            (true, Domain::Ints(..) | Domain::Single) => Pat::Wild,
            _ => Pat::Ctor(
                missing,
                vec![Pat::Wild; parts(self.types, ty, missing).len()],
            ),
        };
        Ok(Some(prepend(head, &w)))
    }
}
