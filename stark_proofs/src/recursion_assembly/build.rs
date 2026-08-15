// NONOS Operating System (AGPL-3.0-or-later)
//! Assembling the wired recursive verifier: prove the inner join-split, build the
//! shared regions and one dependent-region block per inner query, lay them out,
//! bind them, and place the witness. Closing inner-query coverage means every
//! inner query k gets its own DEEP / fold / auth / index-point block, each bound
//! only to its own openings and index. The wired AIR is always the honest one; a
//! tamper only alters the witness of one query, which its block must reject.

use super::layout::{offsets, Layout};
use super::tamper::Tamper;
use super::inner::LOG_ROUNDS;
use super::{auth, compose, compose_step, deep, fri, groups, inner, periodic, points, transcript};
use crate::crypto::stark::air::{Air, AirExt, GpGroup, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Assembly {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub lay: Layout,
    pub publics: Vec<Fp>,
    pub n_groups: usize,
    /// Each region's first row in the stacked trace, in region order.
    pub region_offsets: Vec<usize>,
}

/// The join-split recursion attesting every inner query, with `tamper` applied to
/// query 0 (the historic single-query reject cases).
pub(crate) fn assemble(tamper: Tamper) -> Assembly {
    assemble_q(tamper, 0)
}

/// The join-split recursion attesting every inner query, with `tamper` applied to
/// inner query `tamper_q`. A honest assembly (`Tamper::None`) accepts; a tamper on
/// any query must reject through that query's own block, which is the proof inner
/// coverage is closed rather than query-0-only.
pub(crate) fn assemble_q(tamper: Tamper, tamper_q: usize) -> Assembly {
    assemble_capped(tamper, tamper_q, usize::MAX)
}

/// The same assembly attesting only the first `cap` inner queries. Full coverage is
/// `cap >= n_queries`; a small `cap` builds the identical per-query machinery over a
/// far smaller trace, so a real FRI prove+verify (which the full 32-query trace is
/// too memory-heavy to run off a big-RAM box) can exercise the degree bounds and the
/// multi-query wiring end to end.
pub(crate) fn assemble_capped(tamper: Tamper, tamper_q: usize, cap: usize) -> Assembly {
    let h = inner::hasher();
    let inner = inner::join_split(&h);
    let n_q = inner.proof.queries.len().min(cap);

    // Shared regions: transcript (built after n_terms is known), compose, the FRI
    // transcript (drawing every FRI index), and the periodic recompute.
    let (cregion, ctrace) = compose::compose_region(&inner);
    let ft = fri::fri_transcript(&h, &inner);
    let (pzregion, pztrace) = periodic::periodic_region(&inner);

    // One dependent-region block per query, in [deep, fold, auth, ip, fp] order.
    // The per-query metadata is uniform, taken from query 0.
    let mut q_boxes: Vec<Box<dyn AirExt>> = Vec::new();
    let mut q_traces: Vec<Vec<Fp>> = Vec::new();
    let mut n_terms = 0usize;
    let mut ocells: Vec<Vec<(usize, usize)>> = Vec::with_capacity(n_q);
    let (mut depth, mut n_open, mut pbits, mut fbits) = (0usize, 0usize, 0usize, 0usize);
    let mut i0 = 0usize;
    for k in 0..n_q {
        let tk = if k == tamper_q { tamper } else { Tamper::None };
        let (dreg, dtr, nt) = deep::deep_region_k(&h, &inner, k, tk);
        let fold = fri::fri_fold_k(&inner, &ft, k);
        let au = auth::auth_side_k(&h, &inner, fold.ik, k, tk);
        let pts = points::point_regions_k(&au.cons_dirs, fold.ik, ft.log_n);
        if k == 0 {
            n_terms = nt;
            depth = au.depth;
            n_open = au.n_open;
            pbits = pts.pbits;
            fbits = pts.fbits;
            i0 = au.cons_dirs.iter().enumerate().fold(0, |a, (lv, &b)| a | ((b as usize) << lv));
        }
        // Each query's opened-cell columns depend on its own index parity.
        ocells.push(au.ocells.clone());
        q_boxes.push(Box::new(dreg));
        q_traces.push(dtr);
        q_boxes.push(Box::new(fold.fold));
        q_traces.push(fold.ftrace);
        q_boxes.push(Box::new(au.region));
        q_traces.push(au.trace);
        q_boxes.push(Box::new(pts.ip));
        q_traces.push(pts.itrace);
        q_boxes.push(Box::new(pts.fp));
        q_traces.push(pts.fptrace);
    }

    let ts = transcript::stark_transcript(&h, &inner, n_terms);
    let width_inner = ocells[0].len() - 3;
    let t_inner = inner.t as usize;
    let (z_op, deep_coeff_op) = (ts.z_op, ts.deep_coeff_op);
    let pub_len = inner.publics.len();
    let ntr = inner.proof.trace_roots.len();
    let ncoeff2 = inner.ci.coeffs.len() * 2;

    let mut regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(ts.region) as Box<dyn AirExt>,
        Box::new(cregion),
        Box::new(ft.transcript),
        Box::new(pzregion),
    ];
    let mut traces: Vec<Vec<Fp>> = alloc::vec![ts.trace, ctrace, ft.ttrace, pztrace];
    regions.extend(q_boxes);
    traces.extend(q_traces);

    let (off, span) = offsets(&regions);
    let (c_off, ft_off, pz_off) = (off[1], off[2], off[3]);
    let base = 4;
    let d_off: Vec<usize> = (0..n_q).map(|k| off[base + k * 5]).collect();
    let f_off: Vec<usize> = (0..n_q).map(|k| off[base + k * 5 + 1]).collect();
    let m_off: Vec<usize> = (0..n_q).map(|k| off[base + k * 5 + 2]).collect();
    let i_off: Vec<usize> = (0..n_q).map(|k| off[base + k * 5 + 3]).collect();
    let fp_off: Vec<usize> = (0..n_q).map(|k| off[base + k * 5 + 4]).collect();

    let lay = Layout {
        span,
        l: 1usize << LOG_ROUNDS,
        n_q,
        i0,
        c_off,
        ft_off,
        pz_off,
        d_off,
        f_off,
        m_off,
        i_off,
        fp_off,
        z_op,
        deep_coeff_op,
        pub_len,
        ntr,
        ncoeff2,
        n_terms,
        width_inner,
        window_inner: (n_terms - 1) / width_inner,
        ocells,
        depth,
        n_open,
        n_folds: ft.n_folds,
        log_n: ft.log_n,
        pbits,
        fbits,
        t_inner,
        n_pz: 5,
        frame_len: 6,
        n_coeff: 8,
        c_periodic_col: 12,
        c_z_col: 22,
        c_coeff_col: 24,
        c_comp_z_col: 54,
    };

    let gps = build_groups(&lay);
    let n_groups = gps.len();
    let wired = WiredMultiExt::new(regions, gps);
    let witness = wired.trace(&traces);
    Assembly { wired, witness, lay, publics: inner.publics, n_groups, region_offsets: off }
}

fn build_groups(lay: &Layout) -> Vec<GpGroup> {
    let mut gps: Vec<GpGroup> = Vec::new();
    groups::statement(lay, &mut gps);
    groups::deep(lay, &mut gps);
    groups::roots(lay, &mut gps);
    groups::fold(lay, &mut gps);
    groups::index(lay, &mut gps);
    groups::periodic(lay, &mut gps);
    gps
}

/// The parked step-AIR path: a single-query (query-0-only) recursion over a
/// zkolang inner, kept building against the per-query Layout with n_q = 1.
pub(crate) fn assemble_step(tamper: Tamper) -> Assembly {
    let h = inner::hasher();
    let inner = inner::step_air(&h);

    let (dregion, dtrace, n_terms) = deep::deep_region(&h, &inner, tamper);
    let ts = transcript::stark_transcript(&h, &inner, n_terms);
    let fs = fri::fri_side(&h, &inner);
    let au = auth::auth_side(&h, &inner, fs.i0, tamper);
    let pts = points::point_regions(&fs, &au);
    let (pzregion, pztrace) = periodic::periodic_region(&inner);

    let width_inner = au.ocells.len() - 3;
    let t_inner = inner.t as usize;
    let (z_op, deep_coeff_op) = (ts.z_op, ts.deep_coeff_op);
    let pub_len = inner.publics.len();
    let ntr = inner.proof.trace_roots.len();
    let ncoeff2 = inner.ci.coeffs.len() * 2;
    let n_pz = inner.air.periodic_columns().len();
    let publics = inner.publics.clone();

    let (cregion, ctrace) = compose_step::compose_step_region(inner);
    let frame_len = cregion.frame_len();
    let n_coeff = cregion.num_coeff();
    let c_periodic_col = cregion.periodic_col(0);
    let c_z_col = cregion.z_col();
    let c_coeff_col = cregion.coeff_col(0);
    let c_comp_z_col = cregion.comp_z_col();

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(ts.region) as Box<dyn AirExt>,
        Box::new(cregion),
        Box::new(dregion),
        Box::new(fs.transcript),
        Box::new(fs.fold),
        Box::new(au.region),
        Box::new(pts.ip),
        Box::new(pts.fp),
        Box::new(pzregion),
    ];
    let (off, span) = offsets(&regions);

    let lay = Layout {
        span,
        l: 1usize << LOG_ROUNDS,
        n_q: 1,
        i0: au.cons_dirs.iter().enumerate().fold(0, |a, (lv, &b)| a | ((b as usize) << lv)),
        c_off: off[1],
        ft_off: off[3],
        pz_off: off[8],
        d_off: alloc::vec![off[2]],
        f_off: alloc::vec![off[4]],
        m_off: alloc::vec![off[5]],
        i_off: alloc::vec![off[6]],
        fp_off: alloc::vec![off[7]],
        z_op,
        deep_coeff_op,
        pub_len,
        ntr,
        ncoeff2,
        n_terms,
        width_inner,
        window_inner: (n_terms - 1) / width_inner,
        ocells: alloc::vec![au.ocells],
        depth: au.depth,
        n_open: au.n_open,
        n_folds: fs.n_folds,
        log_n: fs.log_n,
        pbits: pts.pbits,
        fbits: pts.fbits,
        t_inner,
        n_pz,
        frame_len,
        n_coeff,
        c_periodic_col,
        c_z_col,
        c_coeff_col,
        c_comp_z_col,
    };

    let gps = build_groups(&lay);
    let n_groups = gps.len();
    let wired = WiredMultiExt::new(regions, gps);
    let witness = wired.trace(&[
        ts.trace, ctrace, dtrace, fs.ttrace, fs.ftrace, au.trace, pts.itrace, pts.fptrace, pztrace,
    ]);
    Assembly { wired, witness, lay, publics, n_groups, region_offsets: off }
}
