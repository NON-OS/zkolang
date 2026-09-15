// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, ShieldRegion, RATE};
use crate::crypto::stark::field::Fp;
use super::assoc::assoc_membership;
use super::keys::key_hierarchies;
use crate::shield::key::Break;
use super::pool::{pool_membership, pool_membership_against, Witnessed};
use super::notes::note_regions;
use crate::shield::note::Note;
use alloc::vec::Vec;

pub struct Stack {
    pub regions: Vec<ShieldRegion>,
    pub traces: Vec<Vec<Fp>>,
    pub span_op: usize,
    pub leaf_col: Vec<usize>,
    pub key_span: Vec<usize>,
    pub root: [Fp; RATE],
    pub nf: [[Fp; RATE]; 2],
    pub out_cm: [[Fp; RATE]; 2],
    pub assoc_root: [Fp; RATE],
    pub assoc_col: Vec<usize>,
    pub depth: usize,
}

/// Where the two input notes are proven to live.
///
/// `Planted` builds a tree holding only those notes and proves against its own
/// root, which is what the fixtures want and what no deployment can accept.
/// `Published` proves against a root the pool already published, with the
/// openings supplied by whoever read the tree.
pub enum Anchor<'a> {
    Planted,
    Published { openings: [&'a Witnessed; 2], root: [Fp; RATE] },
}

/// Region order: balance, four notes, two memberships, two key hierarchies. The
/// bindings address regions by that order.
#[allow(clippy::too_many_arguments)]
pub fn stack_anchored(
    h: &Poseidon,
    notes: [&Note; 4],
    sks: [[Fp; RATE]; 2],
    brk: Break,
    bal: (ShieldRegion, Vec<Fp>),
    depth: usize,
    anchor: Anchor<'_>,
) -> Stack {
    let n = note_regions(notes, brk);
    let span_op = n.span_op;
    let cms = n.cms.clone();
    let mut regions: Vec<ShieldRegion> = alloc::vec![bal.0];
    let mut traces = alloc::vec![bal.1];
    regions.extend(n.regions);
    traces.extend(n.traces);

    let p = match anchor {
        Anchor::Planted => pool_membership(h, &[cms[0], cms[1]], depth),
        Anchor::Published { openings, root } => {
            /*
             * A path shorter or longer than the tree walks to a root at the
             * wrong height, and the walk would still be honest arithmetic, so
             * it would not fail as a constraint. It fails here instead, where
             * the caller can still see which input was wrong.
             */
            for (i, o) in openings.iter().enumerate() {
                assert_eq!(
                    o.siblings.len(),
                    depth,
                    "input {i} opening is {} levels against a depth {depth} tree",
                    o.siblings.len()
                );
            }
            pool_membership_against(h, &[cms[0], cms[1]], openings, root)
        }
    };
    let leaves = p.leaves.clone();
    let leaf_col = p.leaf_col.clone();
    regions.extend(p.regions);
    traces.extend(p.traces);

    // The position each membership authenticated, recovered as the scalar the
    // nullifier hashes. Placed after membership so a reader meets the position
    // where it is proven, then where it is consumed.
    let ix = super::index::positions(&leaves, depth, brk);
    regions.extend(ix.regions);
    traces.extend(ix.traces);

    let k = key_hierarchies(sks, &[cms[0], cms[1]], &leaves, brk);
    let key_span = k.key_span.clone();
    let nfs = k.nf;
    regions.extend(k.regions);
    traces.extend(k.traces);

    let a = assoc_membership(h, &[cms[0], cms[1]], &[900, 901, 902], brk == Break::Unlisted, depth);
    regions.extend(a.regions);
    traces.extend(a.traces);

    Stack {
        regions,
        traces,
        span_op,
        leaf_col,
        key_span,
        root: p.root,
        nf: [nfs[0], nfs[1]],
        out_cm: [cms[2], cms[3]],
        assoc_root: a.root,
        assoc_col: a.leaf_col,
        depth,
    }
}
