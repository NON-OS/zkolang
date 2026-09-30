/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The check of one `match`: each arm, in order, against the arms before it without a
 * guard, and then a value that no arm without a guard covers.
 */

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::pat::Pat;
use super::useful::Cx;
use super::witness::show;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::tir::TPat;

/** What the check of a `match` found. */
pub(crate) struct Verdict {
    /** A value no arm covers, as the pattern that would cover it, if there is one. */
    pub(crate) missing: Option<String>,
    /** The index of each arm that no value reaches. */
    pub(crate) unreachable: Vec<usize>,
}

/**
 * Check the arms of a `match` on a value of type `ty`, each its pattern and whether it has
 * a guard; `None` if the check takes more steps than it may.
 */
pub(crate) fn check_match(types: &Types, ty: TyId, arms: &[(&TPat, bool)]) -> Option<Verdict> {
    let mut cx = Cx { types, steps: 0 };
    let mut rows: Vec<Vec<Pat>> = Vec::new();
    let mut unreachable = Vec::new();
    for (i, (p, guarded)) in arms.iter().enumerate() {
        let row = vec![Pat::of(types, p, ty)];
        if cx.useful(&rows, &row, &[ty]).ok()?.is_none() {
            unreachable.push(i);
        }
        if !guarded {
            rows.push(row);
        }
    }
    let missing = cx.useful(&rows, &[Pat::Wild], &[ty]).ok()?;
    let missing = missing.map(|w| w.first().map_or(String::from("_"), |p| show(types, ty, p)));
    Some(Verdict {
        missing,
        unreachable,
    })
}
