// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

fn columns_of(class: &[usize], width: usize) -> Vec<usize> {
    let mut cols: Vec<usize> = class.iter().map(|c| c % width).collect();
    cols.sort_unstable();
    cols.dedup();
    cols
}

fn merged(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut u = a.to_vec();
    u.extend_from_slice(b);
    u.sort_unstable();
    u.dedup();
    u
}

/// Classes are placed into groups no wider than `cap`, first fit. A class wider
/// than the cap takes a group of its own rather than being split, because a class
/// cut across two products is no longer one equality.
pub fn pack(classes: Vec<Vec<usize>>, span: usize, width: usize, cap: usize) -> Vec<GpGroup> {
    let mut bins: Vec<(Vec<usize>, Vec<Vec<usize>>)> = Vec::new();
    for class in classes {
        let cols = columns_of(&class, width);
        let fit = bins
            .iter()
            .position(|(bc, _)| merged(bc, &cols).len() <= cap);
        match fit {
            Some(i) => {
                bins[i].0 = merged(&bins[i].0, &cols);
                bins[i].1.push(class);
            }
            None => bins.push((cols, alloc::vec![class])),
        }
    }
    bins.into_iter()
        .map(|(cols, cls)| build(cols, &cls, span, width))
        .collect()
}

fn build(cols: Vec<usize>, classes: &[Vec<usize>], span: usize, width: usize) -> GpGroup {
    let k = cols.len();
    let mut slot = alloc::vec![usize::MAX; width];
    for (i, &c) in cols.iter().enumerate() {
        slot[c] = i;
    }
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for class in classes {
        let ids: Vec<usize> = class
            .iter()
            .map(|&g| (g / width) * k + slot[g % width])
            .collect();
        for w in ids.windows(2) {
            sigma[w[0]] = w[1];
        }
        sigma[ids[ids.len() - 1]] = ids[0];
    }
    GpGroup {
        wired_cols: cols,
        sigma,
        beta: Fp::from_u64(5),
        gamma: Fp::from_u64(7),
    }
}
