/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The rows of patterns the check works on, specialized one column at a time. */

use alloc::vec::Vec;

use super::pat::{Ctor, Pat};

/** The rows, each whose first pattern is alternatives spread into one row per alternative. */
pub(super) fn expand(rows: &[Vec<Pat>]) -> Vec<Vec<Pat>> {
    let mut out = Vec::with_capacity(rows.len());
    rows.iter().for_each(|r| push_expanded(r, &mut out));
    out
}

fn push_expanded(r: &[Pat], out: &mut Vec<Vec<Pat>>) {
    let Some((Pat::Or(alts), rest)) = r.split_first() else {
        out.push(r.to_vec());
        return;
    };
    for a in alts {
        let mut row = Vec::with_capacity(r.len());
        row.push(a.clone());
        row.extend_from_slice(rest);
        push_expanded(&row, out);
    }
}

/** The constructor at the head of each row that has one. */
pub(super) fn heads(rows: &[Vec<Pat>]) -> Vec<Ctor> {
    let head = |r: &Vec<Pat>| match r.first() {
        Some(Pat::Ctor(c, _)) => Some(*c),
        _ => None,
    };
    rows.iter().filter_map(head).collect()
}

/** The rows that take values built with `c`, of `arity` parts, those parts first. */
pub(super) fn specialize(rows: &[Vec<Pat>], c: Ctor, arity: usize) -> Vec<Vec<Pat>> {
    let mut out = Vec::new();
    for r in rows {
        let Some((head, rest)) = r.split_first() else {
            continue;
        };
        let mut row = match head {
            Pat::Wild => alloc::vec![Pat::Wild; arity],
            Pat::Ctor(d, subs) if d.covers(c) => subs.clone(),
            _ => continue,
        };
        row.extend_from_slice(rest);
        out.push(row);
    }
    out
}

/** The rows whose head takes any value, without it. */
pub(super) fn default(rows: &[Vec<Pat>]) -> Vec<Vec<Pat>> {
    let rest = |r: &Vec<Pat>| match r.split_first() {
        Some((Pat::Wild, rest)) => Some(rest.to_vec()),
        _ => None,
    };
    rows.iter().filter_map(rest).collect()
}
