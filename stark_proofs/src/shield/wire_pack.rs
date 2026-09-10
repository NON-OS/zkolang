// NONOS Operating System (AGPL-3.0-or-later)

use super::wire_class::Class;
use crate::crypto::stark::air::{Cell, GpGroup};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// A running product over k wired columns carries degree k+1, and the evaluation
/// domain follows the degree, so one product over the whole trace pays in domain
/// what it saves in columns. The regions here reach degree ten, so nine is free:
/// it holds the product at that same ten while leaving the products as wide as
/// possible, which is what keeps the sigma column count down.
pub const CAP: usize = 9;

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
pub fn packed_groups(span: usize, classes: &[Class], cap: usize) -> Vec<GpGroup> {
    let mut bins: Vec<(Vec<usize>, Vec<&Class>)> = Vec::new();
    for class in classes.iter().filter(|c| c.len() >= 2) {
        let cols = columns_of(class);
        match bins.iter().position(|(bc, _)| merged(bc, &cols).len() <= cap) {
            Some(i) => {
                bins[i].0 = merged(&bins[i].0, &cols);
                bins[i].1.push(class);
            }
            None => bins.push((cols, alloc::vec![class])),
        }
    }
    bins.into_iter().map(|(cols, cls)| build(span, cols, &cls)).collect()
}

/// The group that carries a class: the one whose wired columns cover all of the
/// class's, since packing bins a class into a group whose column set contains it.
fn group_of<'a>(groups: &'a [GpGroup], class: &Class) -> Option<&'a GpGroup> {
    let cols = columns_of(class);
    groups.iter().find(|g| cols.iter().all(|c| g.wired_cols.contains(c)))
}

/// Every binding is actually enforced: each class's cells lie on one cycle of the
/// group that carries them.
///
/// A grand product over a permutation forces the cells of each cycle equal, so a
/// binding holds exactly when its cells share a cycle. This is the property the
/// packing must deliver, and the one worth checking, disjointness of the raw
/// classes is one way to guarantee it but not the only one: overlapping classes
/// can still land every cell of each on a common cycle. Laying them can also drop
/// a cell to a fixed point and lose its binding, so the result is verified here
/// rather than assumed from a sufficient precondition on the input.
pub fn groups_enforce(groups: &[GpGroup], classes: &[Class]) -> bool {
    for class in classes.iter().filter(|c| c.len() >= 2) {
        let g = match group_of(groups, class) {
            Some(g) => g,
            None => return false,
        };
        let k = g.wired_cols.len();
        let index = |cell: &Cell| {
            cell.row * k + g.wired_cols.iter().position(|&x| x == cell.col).unwrap()
        };
        // Walk the cycle out of the first cell; every other cell of the class has
        // to be on it, or the product does not force it equal to the rest.
        let start = index(&class[0]);
        let mut cycle: Vec<usize> = alloc::vec![start];
        let mut cur = g.sigma[start];
        while cur != start {
            cycle.push(cur);
            cur = g.sigma[cur];
        }
        if !class.iter().all(|c| cycle.contains(&index(c))) {
            return false;
        }
    }
    true
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
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shield::wire_class::pair;

    /// A single binding lands both its cells on one cycle: enforced.
    #[test]
    fn one_binding_is_enforced() {
        let classes = alloc::vec![pair(0, 0, 1, 1)];
        let groups = packed_groups(4, &classes, CAP);
        assert!(groups_enforce(&groups, &classes));
    }

    /// The overlap that drops a binding. Three cells tied pairwise (X=Y, Y=Z,
    /// X=Z) are all one equal set, but laying the three pairs in sequence rotates
    /// X out to a fixed point: the last, redundant, pair rewrites the cycle and
    /// leaves X bound to nothing. `groups_enforce` has to catch that the enforced
    /// classes no longer hold, which the disjointness precondition could only
    /// forbid, never detect after the fact.
    #[test]
    fn a_redundant_overlap_that_drops_a_binding_is_caught() {
        let x = pair(0, 0, 1, 1); // X = Y
        let y = pair(1, 1, 2, 2); // Y = Z
        let z = pair(0, 0, 2, 2); // X = Z, redundant given the first two
        let classes = alloc::vec![x, y, z];
        let groups = packed_groups(4, &classes, CAP);
        assert!(
            !groups_enforce(&groups, &classes),
            "X was rotated to a fixed point and its binding lost, unnoticed"
        );
    }
}
