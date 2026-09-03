// NONOS Operating System (AGPL-3.0-or-later)
//! Assembling the wired recursive verifier: prove the inner join-split, build the
//! shared regions and one dependent-region block per inner query, lay them out,
//! bind them, and place the witness. Closing inner-query coverage means every
//! inner query k gets its own DEEP / fold / auth / index-point block, each bound
//! only to its own openings and index. The wired AIR is always the honest one; a
//! tamper only alters the witness of one query, which its block must reject.

use super::inner::{Inner, LOG_ROUNDS};
use super::layout::{offsets, Layout};
use super::tamper::Tamper;
use super::{auth, compose, compose_step, deep, fri, groups, inner, periodic, points, transcript};
use crate::crypto::stark::air::{Air, AirExt, GenericTransition, GpGroup, Poseidon, WiredMultiExt};
use crate::crypto::stark::field::{Fp, Fp2};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Assembly {
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
pub fn assemble(tamper: Tamper) -> Assembly {
    assemble_q(tamper, 0)
}

/// The join-split recursion attesting every inner query, with `tamper` applied to
/// inner query `tamper_q`. A honest assembly (`Tamper::None`) accepts; a tamper on
/// any query must reject through that query's own block, which is the proof inner
/// coverage is closed rather than query-0-only.
pub fn assemble_q(tamper: Tamper, tamper_q: usize) -> Assembly {
    assemble_capped(tamper, tamper_q, usize::MAX)
}

/// The same assembly attesting only the first `cap` inner queries. Full coverage is
/// `cap >= n_queries`; a small `cap` builds the identical per-query machinery over a
/// far smaller trace, so a real FRI prove+verify (which the full 32-query trace is
/// too memory-heavy to run off a big-RAM box) can exercise the degree bounds and the
/// multi-query wiring end to end.
pub fn assemble_capped(tamper: Tamper, tamper_q: usize, cap: usize) -> Assembly {
    let h = inner::hasher();
    let inner = inner::join_split(&h);
    let n_q = inner.proof.queries.len().min(cap);

    // Shared regions: transcript (built after n_terms is known), compose, the FRI
    // transcript (drawing every FRI index), and the periodic recompute.
    let (cregion, ctrace) = compose::compose_region(&inner);
    let ft = fri::fri_transcript(&h, &inner);
    let with_sidecar = false;
    let pz = Some(periodic::periodic_region(&inner, tamper));

    // One dependent-region block per query, in [deep, fold, auth, tauth, ip, fp]
    // order. The per-query metadata is uniform, taken from query 0.
    let mut q_boxes: Vec<Box<dyn AirExt>> = Vec::new();
    let mut q_traces: Vec<Vec<Fp>> = Vec::new();
    let mut n_terms = 0usize;
    let mut ocells: Vec<Vec<(usize, usize)>> = Vec::with_capacity(n_q);
    let mut tchunk_cells: Vec<Vec<(usize, usize)>> = Vec::with_capacity(n_q);
    let (mut depth, mut n_open, mut pbits, mut fbits) = (0usize, 0usize, 0usize, 0usize);
    let mut i0 = 0usize;
    let mut ta_depth = 0usize;
    for k in 0..n_q {
        let tk = if k == tamper_q { tamper } else { Tamper::None };
        std::eprintln!("[asm] q{k} deep");
        let (dreg, dtr, nt) = deep::deep_region_k(&h, &inner, k, tk);
        std::eprintln!("[asm] q{k} fold");
        let fold = fri::fri_fold_k(&inner, &ft, k, tk);
        std::eprintln!("[asm] q{k} auth");
        let au = auth::auth_side_k(&h, &inner, fold.ik, k, tk);
        let ta = auth::trace_auth_k(&h, &inner, &au.cons_dirs, k);
        std::eprintln!("[asm] q{k} points");
        let pts = points::point_regions_k(&au.cons_dirs, fold.ik, ft.log_n, tk);
        if k == 0 {
            n_terms = nt;
            depth = au.depth;
            n_open = au.n_open;
            pbits = pts.pbits;
            fbits = pts.fbits;
            i0 = au
                .cons_dirs
                .iter()
                .enumerate()
                .fold(0, |a, (lv, &b)| a | ((b as usize) << lv));
            ta_depth = ta.depth;
        }
        // Each query's opened-cell columns depend on its own index parity.
        ocells.push(au.ocells.clone());
        tchunk_cells.push(ta.chunk_cells);
        q_boxes.push(Box::new(dreg));
        q_traces.push(dtr);
        q_boxes.push(Box::new(fold.fold));
        q_traces.push(fold.ftrace);
        q_boxes.push(Box::new(au.region));
        q_traces.push(au.trace);
        q_boxes.push(Box::new(ta.region));
        q_traces.push(ta.trace);
        q_boxes.push(Box::new(pts.ip));
        q_traces.push(pts.itrace);
        q_boxes.push(Box::new(pts.fp));
        q_traces.push(pts.fptrace);
    }

    std::eprintln!("[asm] queries done");
    let ts = transcript::stark_transcript(&h, &inner, n_terms);
    std::eprintln!("[asm] transcript");
    let width_inner = inner.air.trace_width();
    let t_inner = inner.t as usize;
    let (z_op, deep_coeff_op) = (ts.z_op, ts.deep_coeff_op);
    let pub_len = inner.publics.len();
    let ntr = 1usize;
    let ncoeff2 = inner.ci.coeffs.len() * 2;

    let mut regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(ts.region) as Box<dyn AirExt>,
        Box::new(cregion),
        Box::new(ft.transcript),
    ];
    let mut traces: Vec<Vec<Fp>> = alloc::vec![ts.trace, ctrace, ft.ttrace];
    if let Some((pzregion, pztrace)) = pz {
        regions.push(Box::new(pzregion));
        traces.push(pztrace);
    }
    regions.extend(q_boxes);
    traces.extend(q_traces);

    let (off, span) = offsets(&regions);
    let (c_off, ft_off) = (off[1], off[2]);
    // Shared count and per-query stride depend on which optional regions run.
    let base = if with_sidecar { 3 } else { 4 };
    let stride = if with_sidecar { 7 } else { 6 };
    let pz_off = if with_sidecar { 0 } else { off[3] };
    let d_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride]).collect();
    let f_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 1]).collect();
    let m_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 2]).collect();
    let ta_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 3]).collect();
    let pa_off: Vec<usize> = if with_sidecar {
        (0..n_q).map(|k| off[base + k * stride + 4]).collect()
    } else {
        Vec::new()
    };
    let i_off: Vec<usize> = (0..n_q)
        .map(|k| off[base + k * stride + stride - 2])
        .collect();
    let fp_off: Vec<usize> = (0..n_q)
        .map(|k| off[base + k * stride + stride - 1])
        .collect();

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
        ta_off,
        tchunk_cells,
        ta_depth,
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
        sidecar: with_sidecar,
        claim_op: ts.claim_op,
        pa_off,
        pchunk_cells: Vec::new(),
        pa_depth: 0,
        n_chunks: 0,
        frame_len: 6,
        n_coeff: 8,
        c_periodic_col: 12,
        c_z_col: 22,
        c_coeff_col: 24,
        c_comp_z_col: 54,
    };

    let gps = fuse(build_groups(&lay), &lay, &regions);
    let n_groups = gps.len();
    // Four shared regions, then a block of six per query in [deep, fold, auth,
    // tauth, ip, fp] order. Every region in a block carries the same periodic
    // pattern for every query, so the blocks are instances of six kinds rather
    // than 6 * n_q distinct regions.
    let kinds: Vec<usize> = (0..4).chain((0..n_q).flat_map(|_| 4..10)).collect();
    // The trace chain anchors to the one constant no region pins in witness
    // form: the zero leaf every chain starts from. Its root is the proof's,
    // bound to the transcript's absorb cells by the roots family.
    let mut pins: Vec<(usize, usize, Fp)> = Vec::new();
    for q in 0..n_q {
        for j in 0..crate::crypto::stark::air::RATE {
            pins.push((j, lay.ta_off[q], Fp::ZERO));
        }
    }
    let wired = WiredMultiExt::new_kinds_bounded(regions, &kinds, gps, pins);
    let witness = wired.trace(&traces);
    Assembly {
        wired,
        witness,
        lay,
        publics: inner.publics,
        n_groups,
        region_offsets: off,
    }
}

/// The recursion over the deployed join-split: every inner query attested, the
/// inner's own constraint code recomputed over the tower by the generic
/// compose. Same per-query machinery as the fixture assembly; what changes is
/// the inner and the compose region, which reads its layout from the gadget
/// instead of carrying the fixture's numbers.
pub fn assemble_real(tamper: Tamper) -> Assembly {
    assemble_real_capped(tamper, usize::MAX)
}

/// The real-inner assembly attesting only the first `cap` queries: the same
/// per-query machinery and every binding, over a trace a fraction of the
/// size. The wiring gate runs here; full coverage is cap >= n_queries.
pub fn assemble_real_capped(tamper: Tamper, cap: usize) -> Assembly {
    let h = inner::hasher();
    let inner = inner::shield_join_split(&h);
    assemble_over(&h, inner, tamper, cap)
}

/// The generic assembler: any inner whose transition the compose gadget can
/// recompute over the tower rides the full per-query recursion. The deployed
/// join-split comes through here, and so does any compiled zkolang program,
/// which is what makes writing a new circuit in the language enough to make
/// it aggregatable.
pub fn assemble_over<A: AirExt + GenericTransition + 'static>(
    h: &Poseidon,
    inner: Inner<A>,
    tamper: Tamper,
    cap: usize,
) -> Assembly {
    let n_q = inner.proof.queries.len().min(cap);
    let with_sidecar = inner.sidecar.is_some();

    // Row tampers stage before any region reads the sidecar; a forgery bent
    // after the regions are built tests nothing.
    let mut inner = inner;
    if with_sidecar && tamper == Tamper::BentOpenedRow {
        if let Some(sc) = inner.sidecar.as_mut() {
            sc.openings[0].row[0] = sc.openings[0].row[0] + Fp::ONE;
        }
    }
    if with_sidecar && tamper == Tamper::SwappedRowValues {
        if let Some(sc) = inner.sidecar.as_mut() {
            sc.openings[0].row.swap(0, 1);
        }
    }

    let ft = fri::fri_transcript(h, &inner);
    std::eprintln!("[asm] fri transcript");
    // A sidecar inner carries its schedule as a baked root; only the plain
    // path pays for the recompute region.
    let pz = if with_sidecar {
        None
    } else {
        Some(periodic::periodic_region(&inner, tamper))
    };
    std::eprintln!("[asm] periodic region");

    let mut q_boxes: Vec<Box<dyn AirExt>> = Vec::new();
    let mut q_traces: Vec<Vec<Fp>> = Vec::new();
    let mut n_terms = 0usize;
    let mut ocells: Vec<Vec<(usize, usize)>> = Vec::with_capacity(n_q);
    let mut tchunk_cells: Vec<Vec<(usize, usize)>> = Vec::with_capacity(n_q);
    let (mut depth, mut n_open, mut pbits, mut fbits) = (0usize, 0usize, 0usize, 0usize);
    let mut i0 = 0usize;
    let mut pa_depth = 0usize;
    let mut ta_depth = 0usize;
    let mut pchunk_cells: Vec<Vec<(usize, usize)>> = Vec::new();
    for k in 0..n_q {
        let tk = if k == 0 { tamper } else { Tamper::None };
        let (dreg, dtr, nt) = deep::deep_region_k(h, &inner, k, tk);
        std::eprintln!("[asm] q{k} fold");
        let fold = fri::fri_fold_k(&inner, &ft, k, tk);
        let au = auth::auth_side_k(h, &inner, fold.ik, k, tk);
        let ta = auth::trace_auth_k(h, &inner, &au.cons_dirs, k);
        std::eprintln!("[asm] q{k} points");
        let pts = points::point_regions_k(&au.cons_dirs, fold.ik, ft.log_n, tk);
        let pa = auth::periodic_auth_k(h, &inner, &au.cons_dirs, k);
        if k == 0 {
            n_terms = nt;
            depth = au.depth;
            n_open = au.n_open;
            pbits = pts.pbits;
            fbits = pts.fbits;
            i0 = au
                .cons_dirs
                .iter()
                .enumerate()
                .fold(0, |a, (lv, &b)| a | ((b as usize) << lv));
            pa_depth = pa.as_ref().map(|x| x.depth).unwrap_or(0);
            ta_depth = ta.depth;
        }
        ocells.push(au.ocells.clone());
        tchunk_cells.push(ta.chunk_cells);
        q_boxes.push(Box::new(dreg));
        q_traces.push(dtr);
        q_boxes.push(Box::new(fold.fold));
        q_traces.push(fold.ftrace);
        q_boxes.push(Box::new(au.region));
        q_traces.push(au.trace);
        q_boxes.push(Box::new(ta.region));
        q_traces.push(ta.trace);
        if let Some(x) = pa {
            pchunk_cells.push(x.chunk_cells);
            q_boxes.push(Box::new(x.region));
            q_traces.push(x.trace);
        }
        q_boxes.push(Box::new(pts.ip));
        q_traces.push(pts.itrace);
        q_boxes.push(Box::new(pts.fp));
        q_traces.push(pts.fptrace);
    }

    let ts = transcript::stark_transcript(h, &inner, n_terms);
    let width_inner = inner.air.trace_width();
    let pchunk_len = inner
        .sidecar
        .as_ref()
        .map(|sc| sc.periodic_z.len())
        .unwrap_or(0);
    let t_inner = inner.t as usize;
    let (z_op, deep_coeff_op) = (ts.z_op, ts.deep_coeff_op);
    let pub_len = inner.publics.len();
    let ntr = 1usize;
    let ncoeff2 = inner.ci.coeffs.len() * 2;
    let n_pz = inner.air.periodic_columns().len();
    let publics = inner.publics.clone();

    // The compose region takes the inner by value: it owns the AIR to recompute
    // the transitions during proving, so it is built last.
    // The sidecar tamper bends the claims the composition consumes and moves
    // comp_z with them, so the compose region is internally consistent and
    // lying. The transcript and deep regions keep the honest claims: exactly
    // one side of the three-way tie moves, and only the binding can catch it.
    let sidecar_root = inner.sidecar.as_ref().map(|sc| sc.root);
    let mut inner = inner;
    if with_sidecar && tamper == Tamper::PeriodicOffPoint {
        inner.ci.periodic_z[0] = inner.ci.periodic_z[0] + Fp2::ONE;
        inner.ci.comp_z = crate::crypto::stark::air::compose_ext(
            &inner.air,
            inner.g,
            inner.ci.z,
            &inner.proof.ood_frame,
            &inner.ci.periodic_z,
            &inner.ci.coeffs,
        );
    }
    std::eprintln!("[asm] compose gen");
    let (cregion, ctrace) = compose_step::compose_gen_region(inner);
    std::eprintln!("[asm] compose done");
    let frame_len = cregion.frame_len();
    let n_coeff = cregion.num_coeff();
    let c_periodic_col = cregion.periodic_col(0);
    let c_z_col = cregion.z_col();
    let c_coeff_col = cregion.coeff_col(0);
    let c_comp_z_col = cregion.comp_z_col();

    let mut regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(ts.region) as Box<dyn AirExt>,
        Box::new(cregion),
        Box::new(ft.transcript),
    ];
    let mut traces: Vec<Vec<Fp>> = alloc::vec![ts.trace, ctrace, ft.ttrace];
    if let Some((pzregion, pztrace)) = pz {
        regions.push(Box::new(pzregion));
        traces.push(pztrace);
    }
    regions.extend(q_boxes);
    traces.extend(q_traces);

    let (off, span) = offsets(&regions);
    let (c_off, ft_off) = (off[1], off[2]);
    // Shared count and per-query stride depend on which optional regions run.
    let base = if with_sidecar { 3 } else { 4 };
    let stride = if with_sidecar { 7 } else { 6 };
    let pz_off = if with_sidecar { 0 } else { off[3] };
    let d_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride]).collect();
    let f_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 1]).collect();
    let m_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 2]).collect();
    let ta_off: Vec<usize> = (0..n_q).map(|k| off[base + k * stride + 3]).collect();
    let pa_off: Vec<usize> = if with_sidecar {
        (0..n_q).map(|k| off[base + k * stride + 4]).collect()
    } else {
        Vec::new()
    };
    let i_off: Vec<usize> = (0..n_q)
        .map(|k| off[base + k * stride + stride - 2])
        .collect();
    let fp_off: Vec<usize> = (0..n_q)
        .map(|k| off[base + k * stride + stride - 1])
        .collect();

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
        ta_off,
        tchunk_cells,
        ta_depth,
        z_op,
        deep_coeff_op,
        pub_len,
        ntr,
        ncoeff2,
        n_terms,
        width_inner,
        // The sidecar appends one term per periodic column behind the frame
        // and composition terms; the window is what remains, and it divides
        // exactly or the term list is not what this layout thinks it is.
        window_inner: {
            let frame_terms = n_terms - 1 - pchunk_len;
            assert!(
                frame_terms % width_inner == 0,
                "deep terms do not tile the frame: {frame_terms} over width {width_inner}"
            );
            frame_terms / width_inner
        },
        ocells,
        depth,
        n_open,
        n_folds: ft.n_folds,
        log_n: ft.log_n,
        pbits,
        fbits,
        t_inner,
        n_pz,
        sidecar: with_sidecar,
        claim_op: ts.claim_op,
        pa_off,
        pchunk_cells,
        pa_depth,
        n_chunks: n_pz.div_ceil(crate::crypto::stark::air::RATE),
        frame_len,
        n_coeff,
        c_periodic_col,
        c_z_col,
        c_coeff_col,
        c_comp_z_col,
    };

    std::eprintln!("[asm] offsets/layout");
    let gps = fuse(build_groups(&lay), &lay, &regions);
    std::eprintln!("[asm] groups fused");
    let n_groups = gps.len();
    let shared = if with_sidecar { 3 } else { 4 };
    let per_q = if with_sidecar { 7 } else { 6 };
    let kinds: Vec<usize> = (0..shared)
        .chain((0..n_q).flat_map(|_| shared..shared + per_q))
        .collect();
    // The chain openings anchor to constants no region pins in witness form:
    // the zero leaf that starts every chain, and for the sidecar the baked
    // periodic root every chain must reach. The trace chain's root is the
    // proof's, bound to the transcript's absorb cells by the roots family, so
    // only its zero leaf pins here.
    let mut pins: Vec<(usize, usize, Fp)> = Vec::new();
    for q in 0..n_q {
        // Boundary tuples are (column, row, value).
        for j in 0..crate::crypto::stark::air::RATE {
            pins.push((j, lay.ta_off[q], Fp::ZERO));
        }
    }
    if let Some(root) = sidecar_root {
        for q in 0..n_q {
            let pa = lay.pa_off[q];
            for j in 0..crate::crypto::stark::air::RATE {
                pins.push((j, pa, Fp::ZERO));
                pins.push((j, pa + lay.pa_depth * lay.l, root[j]));
            }
        }
    }
    let wired = WiredMultiExt::new_kinds_bounded(regions, &kinds, gps, pins);
    std::eprintln!("[asm] engine built");
    let witness = wired.trace(&traces);
    std::eprintln!("[asm] witness placed");
    Assembly {
        wired,
        witness,
        lay,
        publics,
        n_groups,
        region_offsets: off,
    }
}

/// The capped real assembly next to its raw binds, for the bind-truth probe.
pub fn build_groups_for(cap: usize) -> (Assembly, Vec<groups::Bind>) {
    let asm = assemble_real_capped(Tamper::None, cap);
    let binds = build_groups(&asm.lay);
    (asm, binds)
}

fn build_groups(lay: &Layout) -> Vec<groups::Bind> {
    let mut gps: Vec<groups::Bind> = Vec::new();
    groups::statement(lay, &mut gps);
    groups::deep(lay, &mut gps);
    groups::roots(lay, &mut gps);
    groups::fold(lay, &mut gps);
    groups::index(lay, &mut gps);
    groups::periodic(lay, &mut gps);
    gps
}

/// Regions stack vertically over shared columns, so the addressable width is the
/// widest region rather than the sum.
fn fuse(gps: Vec<groups::Bind>, lay: &Layout, regions: &[Box<dyn AirExt>]) -> Vec<GpGroup> {
    let width = regions.iter().map(|r| r.trace_width()).max().unwrap_or(1);
    groups::collapse(&gps, lay.span, width)
}

/// The parked step-AIR path: a single-query (query-0-only) recursion over a
/// zkolang inner, kept building against the per-query Layout with n_q = 1.
pub fn assemble_step(tamper: Tamper) -> Assembly {
    let h = inner::hasher();
    let inner = inner::step_air(&h);

    let (dregion, dtrace, n_terms) = deep::deep_region(&h, &inner, tamper);
    let ts = transcript::stark_transcript(&h, &inner, n_terms);
    let fs = fri::fri_side(&h, &inner, tamper);
    let au = auth::auth_side(&h, &inner, fs.i0, tamper);
    let ta = auth::trace_auth_k(&h, &inner, &au.cons_dirs, 0);
    let pts = points::point_regions(&fs, &au, tamper);
    let (pzregion, pztrace) = periodic::periodic_region(&inner, tamper);

    let width_inner = inner.air.trace_width();
    let t_inner = inner.t as usize;
    let (z_op, deep_coeff_op) = (ts.z_op, ts.deep_coeff_op);
    let pub_len = inner.publics.len();
    let ntr = 1usize;
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
        Box::new(ta.region),
        Box::new(pts.ip),
        Box::new(pts.fp),
        Box::new(pzregion),
    ];
    let (off, span) = offsets(&regions);

    let lay = Layout {
        span,
        l: 1usize << LOG_ROUNDS,
        n_q: 1,
        i0: au
            .cons_dirs
            .iter()
            .enumerate()
            .fold(0, |a, (lv, &b)| a | ((b as usize) << lv)),
        c_off: off[1],
        ft_off: off[3],
        pz_off: off[9],
        d_off: alloc::vec![off[2]],
        f_off: alloc::vec![off[4]],
        m_off: alloc::vec![off[5]],
        i_off: alloc::vec![off[7]],
        fp_off: alloc::vec![off[8]],
        ta_off: alloc::vec![off[6]],
        tchunk_cells: alloc::vec![ta.chunk_cells],
        ta_depth: ta.depth,
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
        sidecar: false,
        claim_op: 0,
        pa_off: Vec::new(),
        pchunk_cells: Vec::new(),
        pa_depth: 0,
        n_chunks: 0,
        frame_len,
        n_coeff,
        c_periodic_col,
        c_z_col,
        c_coeff_col,
        c_comp_z_col,
    };

    let gps = fuse(build_groups(&lay), &lay, &regions);
    let n_groups = gps.len();
    let kinds: Vec<usize> = (0..regions.len()).collect();
    let pins: Vec<(usize, usize, Fp)> = (0..crate::crypto::stark::air::RATE)
        .map(|j| (j, lay.ta_off[0], Fp::ZERO))
        .collect();
    let wired = WiredMultiExt::new_kinds_bounded(regions, &kinds, gps, pins);
    let witness = wired.trace(&[
        ts.trace,
        ctrace,
        dtrace,
        fs.ttrace,
        fs.ftrace,
        au.trace,
        ta.trace,
        pts.itrace,
        pts.fptrace,
        pztrace,
    ]);
    Assembly {
        wired,
        witness,
        lay,
        publics,
        n_groups,
        region_offsets: off,
    }
}
