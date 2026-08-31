// NONOS Operating System (AGPL-3.0-or-later)
//! Group construction. A binding is its cells and its swaps, nothing more:
//! the identity part of a permutation carries no information, and building it
//! densely before the collapse allocated span * k slots per raw group, which
//! at the real inner's seven-hundred-column deep groups was a hundred and
//! thirty gigabytes of identity. The collapse only ever looked at the swaps.

use alloc::vec::Vec;

/// A raw binding before the collapse: the columns it touches and the swapped
/// cell pairs in (row, lane, row, lane) form. Dense sigma exists only for the
/// packed groups the collapse emits.
pub struct Bind {
    pub wired_cols: Vec<usize>,
    pub swaps: Vec<(usize, usize, usize, usize)>,
}

pub fn group(
    _span: usize,
    wcols: Vec<usize>,
    swaps: &[(usize, usize, usize, usize)],
) -> Bind {
    let lane = |c: usize| wcols.iter().position(|&x| x == c).unwrap();
    let swaps = swaps.iter().map(|&(ra, ca, rb, cb)| (ra, lane(ca), rb, lane(cb))).collect();
    Bind { wired_cols: wcols, swaps }
}

/// Chain consecutive cells into transpositions: composed, a cycle forcing all
/// of them equal.
pub fn chain(cells: &[(usize, usize)], out: &mut Vec<(usize, usize, usize, usize)>) {
    for w in cells.windows(2) {
        out.push((w[0].0, w[0].1, w[1].0, w[1].1));
    }
}

/// One binding holding one cycle of equal cells.
pub fn cycle(span: usize, cells: &[(usize, usize)]) -> Bind {
    let mut wcols: Vec<usize> = cells.iter().map(|c| c.1).collect();
    wcols.sort_unstable();
    wcols.dedup();
    let mut swaps = Vec::new();
    chain(cells, &mut swaps);
    group(span, wcols, &swaps)
}
