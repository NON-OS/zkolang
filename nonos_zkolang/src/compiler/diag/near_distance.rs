/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The edit distance between two names. */

use alloc::vec::Vec;

/**
 * The fewest edits that turn `a` into `b`, where an edit inserts, removes or changes one
 * letter, or swaps two neighbouring letters.
 */
pub(super) fn distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let at = |row: &[usize], k: usize| row.get(k).copied().unwrap_or(usize::MAX);
    /* The distances from the first i - 1, i and i + 1 letters of `a`, in turn. */
    let mut before: Vec<usize> = Vec::new();
    let mut last: Vec<usize> = (0..=b.len()).collect();
    for (i, &x) in a.iter().enumerate() {
        let mut row = Vec::with_capacity(last.len());
        row.push(i.saturating_add(1));
        for (j, &y) in b.iter().enumerate() {
            let mut d = at(&last, j)
                .saturating_add(usize::from(x != y))
                .min(at(&last, j.saturating_add(1)).saturating_add(1))
                .min(at(&row, j).saturating_add(1));
            if let (Some(pi), Some(pj)) = (i.checked_sub(1), j.checked_sub(1)) {
                if a.get(pi) == Some(&y) && b.get(pj) == Some(&x) {
                    d = d.min(at(&before, pj).saturating_add(1));
                }
            }
            row.push(d);
        }
        before = core::mem::replace(&mut last, row);
    }
    last.last().copied().unwrap_or(0)
}
