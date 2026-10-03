/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The constructors a column of patterns tells apart. A range of integers is cut at the
 * ends of every range in the column, so each piece lies wholly inside or outside each.
 */

use alloc::vec::Vec;

use super::domain::Domain;
use super::pat::Ctor;

/** Each constructor of `d` as the heads `heads` tell them apart, and whether one covers it. */
pub(super) fn split(d: Domain, heads: &[Ctor]) -> Vec<(Ctor, bool)> {
    let has = |c: Ctor| heads.iter().any(|h| h.covers(c));
    match d {
        Domain::Single => alloc::vec![(Ctor::Single, has(Ctor::Single))],
        Domain::Bools => [false, true]
            .into_iter()
            .map(|b| (Ctor::Bool(b), has(Ctor::Bool(b))))
            .collect(),
        Domain::Variants(n) => (0..n)
            .map(|t| (Ctor::Variant(t), has(Ctor::Variant(t))))
            .collect(),
        Domain::Ints(lo, hi) => cut(lo, hi, heads),
        Domain::Opaque => Vec::new(),
    }
}

/** The integers `lo..=hi` cut at the ends of the ranges in `heads`, each piece with whether one covers it. */
pub(super) fn cut(lo: i128, hi: i128, heads: &[Ctor]) -> Vec<(Ctor, bool)> {
    let mut at: Vec<i128> = Vec::new();
    for h in heads {
        if let Ctor::Range(a, b) = *h {
            if lo < a && a <= hi {
                at.push(a);
            }
            if lo <= b && b < hi {
                at.push(b + 1);
            }
        }
    }
    at.sort_unstable();
    at.dedup();
    let mut out = Vec::with_capacity(at.len() + 1);
    let mut start = lo;
    for end in at.into_iter().chain(core::iter::once(hi + 1)) {
        let piece = Ctor::Range(start, end - 1);
        out.push((piece, heads.iter().any(|h| h.covers(piece))));
        start = end;
    }
    out
}
