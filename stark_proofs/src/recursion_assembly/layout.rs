// NONOS Operating System (AGPL-3.0-or-later)
//! The assembled trace geometry every binding family reads: region row
//! offsets, the padded span, and the per-region cell metadata. The query-
//! dependent regions (DEEP, fold, auth, the two index points) are laid out once
//! per inner query, so their offsets are vectors indexed by query; the shared
//! regions (transcript, compose, FRI transcript, periodic-z) keep single offsets.

use crate::crypto::stark::air::AirExt;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Layout {
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
    /// Per-query region offsets: DEEP, fold, auth, consistency point, FRI point.
    pub d_off: Vec<usize>,
    pub f_off: Vec<usize>,
    pub m_off: Vec<usize>,
    pub i_off: Vec<usize>,
    pub fp_off: Vec<usize>,
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
pub(crate) fn offsets(regions: &[Box<dyn AirExt>]) -> (Vec<usize>, usize) {
    let mut off = Vec::with_capacity(regions.len());
    let mut r = 0usize;
    for reg in regions {
        off.push(r);
        r += reg.rows();
    }
    (off, r.next_power_of_two())
}
