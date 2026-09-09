// NONOS Operating System (AGPL-3.0-or-later)
//! Region 5: the batched authentication set for one query. The FRI leaf the
//! fold consumes (at the fold position `i_k`), then the flat consistency
//! openings (deep, comp) at the consistency index `p_k`, each against its
//! committed root, all equal depth. The trace row authenticates beside these
//! as one chain-plus-path opening under the wide commitment. The swapped-root
//! tamper authenticates the deep and comp values under each other's
//! commitment; the honest AIR must reject the resulting trace.

use super::inner::{extra, Inner, LOG_ROUNDS};
use super::tamper::Tamper;
use crate::crypto::stark::air::{
    query_openings_pre_queryk, query_openings_queryk, AirExt, MultiMembership, Opening, Poseidon,
    RATE, WIDTH,
};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::poseidon_merkle::pack_ext;
use alloc::vec::Vec;

pub struct AuthSide {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    /// Opening 0 is the FRI leaf, 1 deep, 2 comp.
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
    match &inner.sidecar {
        Some(sc) => ops.extend(query_openings_pre_queryk(
            &inner.air,
            &inner.proof,
            &sc.periodic_z,
            extra(),
            h,
            &inner.publics,
            query,
        )),
        None => ops.extend(query_openings_queryk(
            &inner.air,
            &inner.proof,
            extra(),
            h,
            &inner.publics,
            query,
        )),
    }
    ops
}

/// An opened row as one membership opening: a compress chain from the zero
/// digest through the row's chunks, then the Merkle path to the root. The
/// chunks sit on the sibling cells of the chain steps, which is what lets the
/// wiring bind the row's values into the deep quotients with no new region
/// type. The periodic sidecar and the wide trace commitment both open this
/// shape; only the root and the row differ.
pub struct ChainAuth {
    pub region: MultiMembership,
    pub trace: Vec<Fp>,
    /// One `(row, col)` per row value, in row order: the chunk lane cells.
    pub chunk_cells: Vec<(usize, usize)>,
    pub depth: usize,
}

fn chain_auth(
    h: &Poseidon,
    row: &[Fp],
    path: &[[Fp; RATE]],
    root: [Fp; RATE],
    cons_dirs: &[bool],
) -> ChainAuth {
    let n_vals = row.len();
    let n_chunks = n_vals.div_ceil(RATE);
    let mut siblings: Vec<[Fp; RATE]> = Vec::with_capacity(n_chunks + path.len());
    for chunk in 0..n_chunks {
        let mut sib = [Fp::ZERO; RATE];
        for lane in 0..RATE {
            if let Some(v) = row.get(chunk * RATE + lane) {
                sib[lane] = *v;
            }
        }
        siblings.push(sib);
    }
    siblings.extend(path.iter().copied());
    let mut directions = alloc::vec![false; n_chunks];
    directions.extend(cons_dirs.iter().copied());
    let chain = Opening {
        leaf: [Fp::ZERO; RATE],
        root,
        siblings,
        directions,
    };
    let depth = chain.siblings.len();
    let region = MultiMembership::new_witness(h.clone(), LOG_ROUNDS, alloc::vec![chain]);
    let trace = region.trace();
    // Chunk 0 rides the initial state's high half at row 0; chunk m sits on
    // the sibling cells of slot boundary m, which the witness form writes at
    // row m*l - 1, columns WIDTH+1 onward.
    let l = 1usize << LOG_ROUNDS;
    let mut chunk_cells = Vec::with_capacity(n_vals);
    for j in 0..n_vals {
        let (m, lane) = (j / RATE, j % RATE);
        if m == 0 {
            chunk_cells.push((0, RATE + lane));
        } else {
            chunk_cells.push((m * l - 1, WIDTH + 1 + lane));
        }
    }
    ChainAuth {
        region,
        trace,
        chunk_cells,
        depth,
    }
}

pub fn periodic_auth_k<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    cons_dirs: &[bool],
    query: usize,
) -> Option<ChainAuth> {
    let sc = inner.sidecar.as_ref()?;
    let opening = &sc.openings[query];
    Some(chain_auth(
        h,
        &opening.row,
        &opening.path,
        sc.root,
        cons_dirs,
    ))
}

/// The wide trace commitment's opening for query `k`: the opened row is the
/// trace row itself, the root the one the transcript absorbed. Its terminal
/// digest binds to the transcript's absorb cells, not to a pin, because the
/// root is the proof's, not a baked constant.
pub fn trace_auth_k<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    cons_dirs: &[bool],
    query: usize,
) -> ChainAuth {
    let qd = &inner.proof.queries[query];
    chain_auth(
        h,
        &qd.trace,
        &qd.trace_path,
        inner.proof.trace_root,
        cons_dirs,
    )
}

/// Query-0 form, preserved for the current single-query assembly.
pub fn auth_side<A: AirExt>(h: &Poseidon, inner: &Inner<A>, i0: usize, tamper: Tamper) -> AuthSide {
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
    AuthSide {
        region,
        trace,
        ocells,
        cons_dirs,
        depth,
        n_open,
    }
}
