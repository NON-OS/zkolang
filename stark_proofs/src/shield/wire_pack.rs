// NONOS Operating System (AGPL-3.0-or-later)

use super::wire_class::Class;
use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// A running product over k wired columns carries degree k+1, and the evaluation
/// domain follows the degree, so one product over the whole trace pays in domain
/// what it saves in columns. The regions here reach degree ten, so nine is free:
/// it holds the product at that same ten while leaving the products as wide as
/// possible, which is what keeps the sigma column count down.
pub(crate) const CAP: usize = 9;

fn columns_of(class: &Class) -> Vec<usize> {
    let mut cols: Vec<usize> = class.iter().map(|c| c.col).collect();
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

/// The classes over products no wider than `cap`, first fit. A class wider than
/// the cap takes a product of its own: a class split across two products is no
/// longer one equality.
pub(crate) fn packed_groups(span: usize, classes: &[Class], cap: usize) -> Vec<GpGroup> {
    let mut bins: Vec<(Vec<usize>, Vec<&Class>)> = Vec::new();
    for class in classes.iter().filter(|c| c.len() >= 2) {
        let cols = columns_of(class);
        match bins
            .iter()
            .position(|(bc, _)| merged(bc, &cols).len() <= cap)
        {
            Some(i) => {
                bins[i].0 = merged(&bins[i].0, &cols);
                bins[i].1.push(class);
            }
            None => bins.push((cols, alloc::vec![class])),
        }
    }
    bins.into_iter()
        .map(|(cols, cls)| build(span, cols, &cls))
        .collect()
}

fn build(span: usize, cols: Vec<usize>, classes: &[&Class]) -> GpGroup {
    let k = cols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for class in classes {
        let idx: Vec<usize> = class
            .iter()
            .map(|c| c.row * k + cols.iter().position(|&x| x == c.col).unwrap())
            .collect();
        let first = sigma[idx[0]];
        for w in idx.windows(2) {
            sigma[w[0]] = sigma[w[1]];
        }
        sigma[idx[idx.len() - 1]] = first;
    }
    GpGroup {
        wired_cols: cols,
        sigma,
        beta: Fp::from_u64(5),
        gamma: Fp::from_u64(7),
    }
}
