/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The patterns of `match_gen` as written, and the values each matches. */

use crate::match_gen::{Ev, Pat, Val};

impl Pat {
    /** The pattern as zKølang writes it. */
    pub(crate) fn text(&self) -> String {
        match self {
            Pat::Wild => String::from("_"),
            Pat::A => String::from("E::A"),
            Pat::B(p) => format!("E::B({})", p.text()),
            Pat::C(p) => format!("E::C({})", p.text()),
            Pat::Bool(b) => format!("{b}"),
            Pat::Range(a, b) if a == b => format!("{a}"),
            Pat::Range(a, b) => format!("{a}..={b}"),
            Pat::Pair(e, n) => format!("({}, {})", e.text(), n.text()),
            Pat::Or(p, q) => format!("{} | {}", p.text(), q.text()),
        }
    }

    /** Whether the value `v` matches the pattern. */
    pub(crate) fn matches(&self, v: Val) -> bool {
        match (self, v) {
            (Pat::Wild, _) => true,
            (Pat::Or(p, q), v) => p.matches(v) || q.matches(v),
            (Pat::A, Val::E(Ev::A)) => true,
            (Pat::B(p), Val::E(Ev::B(b))) => p.matches(Val::Bool(b)),
            (Pat::C(p), Val::E(Ev::C(n))) => p.matches(Val::Int(n)),
            (Pat::Bool(a), Val::Bool(b)) => *a == b,
            (Pat::Range(a, b), Val::Int(n)) => *a <= n && n <= *b,
            (Pat::Pair(e, n), Val::Pair(ev, x)) => e.matches(Val::E(ev)) && n.matches(Val::Int(x)),
            _ => false,
        }
    }
}

/** Every value of `(E, u8)`. */
pub(crate) fn every_value() -> impl Iterator<Item = Val> {
    let bs = [false, true].map(Ev::B);
    let es = core::iter::once(Ev::A)
        .chain(bs)
        .chain((0..=255u8).map(Ev::C));
    es.flat_map(|e| (0..=255u8).map(move |n| Val::Pair(e, n)))
}
