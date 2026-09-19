// NONOS Operating System (AGPL-3.0-or-later)
//! The assembled trace geometry every binding family reads: region row
//! offsets, the padded span, and the per-region cell metadata. The query-
//! dependent regions (DEEP, fold, auth, the two index points) are laid out once
//! per inner query, so their offsets are vectors indexed by query; the shared
//! regions (transcript, compose, FRI transcript, periodic-z) keep single offsets.

use crate::crypto::stark::air::AirExt;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Layout {
    pub span: usize,
    /// Rounds per transcript operation.
    pub l: usize,
    /// Number of inner queries attested (the coverage).
    pub n_q: usize,
    /// Query-0 consistency index, kept for the parked single-query emit path.
    pub i0: usize,
    pub c_off: usize,
    pub ft_off: usize,
    pub pz_off: usize,
    /// Preprocessed-inner fields. `sidecar` false leaves every one inert.
    pub sidecar: bool,
    pub claim_op: usize,
    pub pa_off: Vec<usize>,
    pub pchunk_cells: Vec<Vec<(usize, usize)>>,
    pub pa_depth: usize,
    /// Poseidon absorb chunks over the inner's periodic claims at z: `n_pz`
    /// values at the sponge rate, rounded up. This is not a count of anything in
    /// the on chain walk. It was named `n_chunks`, and a reader who had the walk
    /// in mind took it for the walk's chunk count, which it never was: the walk
    /// carried the claims on every chunk and paid 32 where this said 18.
    pub n_pz_absorb_chunks: usize,
    /// Per-query region offsets: DEEP, fold, auth, consistency point, FRI point.
    pub d_off: Vec<usize>,
    pub f_off: Vec<usize>,
    pub m_off: Vec<usize>,
    pub i_off: Vec<usize>,
    pub fp_off: Vec<usize>,
    /// The wide trace commitment's chain opening, one region per query: its
    /// offsets, the chunk lane cell of every trace value, and its total depth
    /// (row chunks plus tree levels).
    pub ta_off: Vec<usize>,
    pub tchunk_cells: Vec<Vec<(usize, usize)>>,
    pub ta_depth: usize,
    /// The compose strip's cycles as absolute (row, col) pairs: echo cells to
    /// their producers and final accumulators to the flat acc cells. Empty
    /// when the assembly runs the flat recompute.
    pub strip_cycles: Vec<((usize, usize), (usize, usize))>,
    /// The strip's shape for the layout emit: zero everywhere the assembly
    /// runs the flat recompute.
    pub strip_off: usize,
    pub strip_k: usize,
    pub strip_echo_width: usize,
    /// Base lanes the strip produces, two per inner transition cell. Named for
    /// its unit because the count that reads naturally beside it,
    /// `inner_n_transitions`, is in cells: 32 lanes against 16 cells. A reader
    /// holding both and no unit on either can implement the wrong branch of
    /// the compose region and disagree on exactly those lanes.
    pub strip_n_out: usize,
    pub strip_rows: usize,
    /// Base column of the compose region's first accumulator cell, and zero on
    /// the flat path. The strip out pin is `out[i] - acc[i] - stmt[i]`, so a
    /// consumer cannot form it without this, and it sits past the vanishing
    /// tower where a slot map derived from the published `c_*_col` fields does
    /// not reach.
    pub compose_acc_base_col: usize,
    pub z_op: usize,
    pub deep_coeff_op: usize,
    pub pub_len: usize,
    pub ntr: usize,
    pub ncoeff2: usize,
    pub n_terms: usize,
    pub width_inner: usize,
    pub window_inner: usize,
    /// Opened-cell (row, col) per opening, per query: the committed scalar's
    /// column is 0 or RATE depending on that query's index LSB, so it is not
    /// shared across queries.
    pub ocells: Vec<Vec<(usize, usize)>>,
    pub depth: usize,
    pub n_open: usize,
    pub n_folds: usize,
    pub log_n: u32,
    pub pbits: usize,
    pub fbits: usize,
    pub t_inner: usize,
    pub n_pz: usize,
    // The compose region's own cell columns, reported by the region so the
    // bindings address it without re-deriving its slot layout. The frame is
    // always at column `2 * i`, so only its length is carried.
    pub frame_len: usize,
    pub n_coeff: usize,
    pub c_periodic_col: usize,
    pub c_z_col: usize,
    pub c_coeff_col: usize,
    pub c_comp_z_col: usize,
}

/// Each region's first row in the stacked trace, and the padded span.
pub fn offsets(regions: &[Box<dyn AirExt>]) -> (Vec<usize>, usize) {
    let mut off = Vec::with_capacity(regions.len());
    let mut r = 0usize;
    for reg in regions {
        off.push(r);
        r += reg.rows();
    }
    // The tight row count, matching Stack::closes_at: the padded form here was
    // a third copy of the sizing truth, and it drifted, so the closure and the
    // packed sigmas walked the padding.
    (off, r)
}
