/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Usefulness: whether some list of values matches a list of patterns and no row, with such
 * a list as the witness. Each step specializes the rows by a constructor of the first
 * column, so the work stays within a budget of steps.
 */

use alloc::vec::Vec;

use super::matrix::expand;
use super::pat::Pat;
use crate::compiler::sema::ty::{TyId, Types};

/** The steps the check of one `match` may take. */
const STEPS: u32 = 200_000;

/** The check ran out of steps. */
pub(super) struct TooBig;

/** Values matched by a list of patterns and no row, or `None`; `Err` out of steps. */
pub(super) type Found = Result<Option<Vec<Pat>>, TooBig>;

/** The state of one check. */
pub(super) struct Cx<'t> {
    pub(super) types: &'t Types,
    pub(super) steps: u32,
}

impl Cx<'_> {
    /** Values that `v`, of types `tys`, matches and no row of `rows` does. */
    pub(super) fn useful(&mut self, rows: &[Vec<Pat>], v: &[Pat], tys: &[TyId]) -> Found {
        self.steps += 1;
        if self.steps > STEPS {
            return Err(TooBig);
        }
        let rows = expand(rows);
        let Some((head, rest)) = v.split_first() else {
            return Ok(rows.is_empty().then(Vec::new));
        };
        let (ty, rest_tys) = tys
            .split_first()
            .map_or((Types::ERROR, &[][..]), |(t, r)| (*t, r));
        match head {
            Pat::Or(alts) => {
                for a in alts {
                    if let Some(found) = self.useful(&rows, &prepend(a.clone(), rest), tys)? {
                        return Ok(Some(found));
                    }
                }
                Ok(None)
            }
            Pat::Ctor(c, subs) => self.ctor_head(&rows, (*c, subs), rest, (ty, rest_tys)),
            Pat::Wild => self.wild(&rows, rest, (ty, rest_tys)),
        }
    }
}

/** `p`, then `rest`. */
pub(super) fn prepend(p: Pat, rest: &[Pat]) -> Vec<Pat> {
    core::iter::once(p).chain(rest.iter().cloned()).collect()
}
