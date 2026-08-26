// NONOS Operating System (AGPL-3.0-or-later)
//! Region 5: the batched authentication set for one query. The FRI leaf the
//! fold consumes (at the fold position `i_k`), then the whole consistency opening
//! set (deep, comp, every trace column) at the consistency index `p_k`, each
//! against its committed root, all equal depth. The swapped-root tamper
//! authenticates the deep and comp values under each other's commitment; the
//! honest AIR must reject the resulting trace.

use super::inner::{Inner, EXTRA, LOG_ROUNDS};
use super::tamper::Tamper;
use crate::crypto::stark::air::{query_openings_queryk, AirExt, MultiMembership, Opening, Poseidon};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::poseidon_merkle::pack_ext;
use alloc::vec::Vec;

pub struct AuthSide {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    /// Opening 0 is the FRI leaf, 1 deep, 2 comp, 3+c trace column c.
    pub ocells: Vec<(usize, usize)>,
    /// The consistency index p_k as the deep opening's path directions, LSB first.
    pub cons_dirs: Vec<bool>,
    pub depth: usize,
    pub n_open: usize,
}

/// The openings of query `k`: the FRI leaf at fold position `ik`, then the
/// consistency openings at index `p_k` (derived inside `query_openings_queryk`).
fn openings<A: AirExt>(h: &Poseidon, inner: &Inner<A>, ik: usize, query: usize) -> Vec<Opening> {
    let op0 = &inner.proof.fri.queries[query].layers[0];
    let sibs = op0.a_path.clone();
    let depth = sibs.len();
    let dirs: Vec<bool> = (0..depth).map(|lv| (ik >> lv) & 1 == 1).collect();
    let mut ops = alloc::vec![Opening {
        leaf: pack_ext(op0.a),
        root: inner.proof.fri.roots[0],
        siblings: sibs,
        directions: dirs,
    }];
    ops.extend(query_openings_queryk(&inner.air, &inner.proof, EXTRA, h, &inner.publics, query));
    ops
}

/// Query-0 form, preserved for the current single-query assembly.
pub fn auth_side<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    i0: usize,
    tamper: Tamper,
) -> AuthSide {
    auth_side_k(h, inner, i0, 0, tamper)
}

pub fn auth_side_k<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    ik: usize,
    query: usize,
    tamper: Tamper,
) -> AuthSide {
    let honest = openings(h, inner, ik, query);
    let depth = honest[0].siblings.len();
    let n_open = honest.len();
    let cons_dirs = honest[1].directions.clone();
    let region = MultiMembership::new_witness(h.clone(), LOG_ROUNDS, honest);
    let trace = if tamper == Tamper::SwappedRoot {
        let mut swapped = openings(h, inner, ik, query);
        swapped.swap(1, 2);
        MultiMembership::new_witness(h.clone(), LOG_ROUNDS, swapped).trace()
    } else {
        region.trace()
    };
    let ocells = region.opened_cells();
    AuthSide { region, trace, ocells, cons_dirs, depth, n_open }
}
