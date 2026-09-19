// NONOS Operating System (AGPL-3.0-or-later)
//! One permutation over every wired cell, instead of one per packed group.
//!
//! `pack` cuts the wiring into groups narrow enough to stay inside the degree
//! budget and gives each its own sigma columns. Sound, and expensive: a column
//! wired in many classes lands in many groups and carries a partial sigma in
//! each. Column 8 is in 121 of them, and the settlement outer ends up with
//! 2,025 sigma columns for wiring over 368. That is three quarters of the
//! periodic set, which is what every chunk re-sends and re-hashes on chain.
//!
//! All the cut really buys is the degree bound, and chaining the accumulator
//! buys the same bound: `BLOCK` columns is degree `BLOCK + 1` whether the
//! product stands alone or continues from the block before. So this builds the
//! global permutation and the block structure, and leaves the AIR to the
//! caller.
//!
//! The gate is that these cycles are the classes the wiring declares, which
//! `the_global_permutation_has_the_declared_cycles` asserts, and that the
//! classes themselves did not move, which `emit_wiring`'s digest asserts.

use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// Columns per accumulator step. A step multiplies `BLOCK` factors into the
/// running product, so its constraint is degree `BLOCK + 1`; at eight that is
/// nine, the same degree the packed groups carry today and inside the outer's
/// budget of thirteen.
pub const BLOCK: usize = 8;

/// The wiring as one argument.
pub struct SinglePermutation {
    /// Every column the wiring touches, ascending. Slot `j` of a row means
    /// cell `(row, wired_cols[j])`.
    pub wired_cols: Vec<usize>,
    /// The permutation over `span * wired_cols.len()` slots, in row-major
    /// slot order: `sigma[r * k + j]` is the slot the cell at `(r, j)` is
    /// wired to. Cells in no class map to themselves.
    pub sigma: Vec<usize>,
    /// How many accumulator columns the chained product needs.
    pub blocks: usize,
}

impl SinglePermutation {
    /// The slots a given accumulator step covers, as a range of `j`.
    pub fn block_slots(&self, m: usize) -> core::ops::Range<usize> {
        let k = self.wired_cols.len();
        let lo = m * BLOCK;
        lo..core::cmp::min(lo + BLOCK, k)
    }
}

/// Build the global permutation from the wiring classes.
///
/// Each class becomes one cycle, in the class's own order, exactly as `pack`
/// builds a group's sigma; the only difference is that every class lands in
/// one permutation over one slot space instead of in whichever group the bin
/// packer put it in.
pub fn single(classes: &[Vec<usize>], span: usize, width: usize) -> SinglePermutation {
    let mut wired_cols: Vec<usize> = classes
        .iter()
        .flat_map(|c| c.iter().map(|&cell| cell % width))
        .collect();
    wired_cols.sort_unstable();
    wired_cols.dedup();

    let k = wired_cols.len();
    let mut slot = alloc::vec![usize::MAX; width];
    for (j, &c) in wired_cols.iter().enumerate() {
        slot[c] = j;
    }

    let mut sigma: Vec<usize> = (0..span * k).collect();
    for class in classes {
        let ids: Vec<usize> = class
            .iter()
            .map(|&cell| (cell / width) * k + slot[cell % width])
            .collect();
        for w in ids.windows(2) {
            sigma[w[0]] = w[1];
        }
        sigma[ids[ids.len() - 1]] = ids[0];
    }

    SinglePermutation {
        wired_cols,
        sigma,
        blocks: k.div_ceil(BLOCK),
    }
}

/// The permutation in the carrier the engine takes, at the packer's
/// challenges.
///
/// Those challenges being constants is unsound and not this function's doing:
/// `wired_challenge_tests` breaks a copy constraint and gets accepted. They
/// are repeated here rather than corrected so the two forms stay comparable
/// and the correction lands once, where the challenges are drawn.
pub fn single_group(classes: &[Vec<usize>], span: usize, width: usize) -> GpGroup {
    let p = single(classes, span, width);
    GpGroup {
        wired_cols: p.wired_cols,
        sigma: p.sigma,
        beta: Fp::from_u64(5),
        gamma: Fp::from_u64(7),
    }
}

#[cfg(test)]
mod tests {
    use super::super::collapse::wiring_classes;
    use super::*;

    /// The permutation's cycles are the wiring's classes, cell for cell.
    ///
    /// This is the whole soundness argument for replacing the packed groups.
    /// A grand product over a permutation proves that the multiset of
    /// (value, slot) pairs equals the multiset of (value, sigma(slot)) pairs,
    /// which forces equality exactly along the cycles. So if the cycles are
    /// the declared classes, the single argument proves what the 251
    /// arguments prove, and a cycle that merged two classes or split one
    /// would prove something else.
    fn cycles_match(classes: &[Vec<usize>], span: usize, width: usize) {
        let p = single(classes, span, width);
        let k = p.wired_cols.len();

        /*
         * Every slot has exactly one preimage: sigma is a permutation, not
         * merely a function. A repeated image would silently drop a copy
         * constraint.
         */
        let mut seen = alloc::vec![false; span * k];
        for &s in &p.sigma {
            assert!(!seen[s], "sigma maps two slots to {s}");
            seen[s] = true;
        }

        /*
         * Walk each declared class as a cycle and check it closes, visiting
         * exactly its own cells.
         */
        let mut visited = alloc::vec![false; span * k];
        for class in classes {
            let start =
                (class[0] / width) * k + p.wired_cols.binary_search(&(class[0] % width)).unwrap();
            let mut at = start;
            let mut walked = Vec::new();
            loop {
                assert!(!visited[at], "slot {at} is in two cycles");
                visited[at] = true;
                walked.push(at);
                at = p.sigma[at];
                if at == start {
                    break;
                }
                assert!(walked.len() <= class.len(), "the cycle overran its class");
            }
            let mut want: Vec<usize> = class
                .iter()
                .map(|&c| (c / width) * k + p.wired_cols.binary_search(&(c % width)).unwrap())
                .collect();
            want.sort_unstable();
            walked.sort_unstable();
            assert_eq!(walked, want, "the cycle is not the class");
        }

        /*
         * Every slot not in a class is a fixed point, so no cell is
         * constrained that the wiring did not constrain.
         */
        for (s, &image) in p.sigma.iter().enumerate() {
            if !visited[s] {
                assert_eq!(image, s, "slot {s} moves but belongs to no class");
            }
        }
    }

    #[test]
    fn the_global_permutation_has_the_declared_cycles() {
        /*
         * A small synthetic wiring first, so a failure here is readable
         * without building an outer: three classes over four columns of a
         * five row trace, including a three cycle and two pairs.
         */
        let width = 4;
        let span = 5;
        let classes = alloc::vec![
            alloc::vec![0 * width + 1, 2 * width + 1, 4 * width + 3],
            alloc::vec![1 * width + 0, 3 * width + 2],
            alloc::vec![0 * width + 3, 1 * width + 3],
        ];
        cycles_match(&classes, span, width);

        let p = single(&classes, span, width);
        assert_eq!(p.wired_cols, alloc::vec![0, 1, 2, 3]);
        assert_eq!(p.blocks, 1, "four columns fit one accumulator step");
    }

    /// The same property on the circuit that ships, which is the one whose
    /// wiring the redesign must preserve. Ignored because it assembles the
    /// settlement outer, which is minutes.
    #[test]
    #[ignore]
    fn the_settlement_wiring_survives_the_single_argument() {
        let asm = crate::recursion_assembly::build::assemble_real_capped(
            crate::recursion_assembly::Tamper::None,
            usize::MAX,
        );
        let binds = crate::recursion_assembly::build::binds_for(&asm.lay);
        let width = crate::crypto::stark::air::Air::trace_width(&asm.wired) - asm.n_groups;
        let span = asm.lay.span;
        let classes = wiring_classes(&binds, span, width);
        cycles_match(&classes, span, width);

        let p = single(&classes, span, width);
        let packed: usize = asm
            .wired
            .group_params()
            .iter()
            .map(|(cols, _, _)| cols.len())
            .sum();
        std::println!(
            "sigma columns {} -> {}, accumulators {} -> {}",
            packed,
            p.wired_cols.len(),
            asm.n_groups,
            p.blocks
        );
        assert!(
            p.wired_cols.len() < packed,
            "the single argument must cost fewer sigma columns than the packed groups"
        );
    }
}
