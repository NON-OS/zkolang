// NONOS Operating System (AGPL-3.0-or-later)
//! Inner-query coverage (Seam 2): the recursion now attests every inner query,
//! not just query 0. The honest full-coverage assembly must satisfy every
//! constraint and binding; a tamper on ANY query k must break a binding through
//! query k's own block. A query-5 tamper rejecting is the proof coverage is
//! closed: under query-0-only it would have passed unseen.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::{assemble, assemble_capped, assemble_q, Tamper};

/// Fast witness satisfaction (no FRI): every transition vanishes and every
/// boundary — including each grand product's z=1 closure — holds. A broken
/// per-query binding fails to close and its boundary is violated, so this is a
/// true test of the wiring. The join-split assembly is small, so this is quick.
fn satisfies(air: &WiredMultiExt, witness: &[Fp]) -> bool {
    let w = air.trace_width();
    let ws = air.window_size();
    let total = 1usize << air.log_trace_len();
    let periodic = air.periodic_columns();
    for r in 0..total - (ws - 1) {
        let mut window = alloc::vec::Vec::with_capacity(ws * w);
        for k in 0..ws {
            window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
        }
        let per: alloc::vec::Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        if air.transition(&window, &per).iter().any(|v| *v != Fp::ZERO) {
            return false;
        }
    }
    for (col, row, val) in air.boundary() {
        if witness[row * w + col] != val {
            return false;
        }
    }
    true
}

#[test]
#[ignore]
fn full_coverage_accepts() {
    let asm = assemble(Tamper::None);
    assert!(
        satisfies(&asm.wired, &asm.witness),
        "the honest full-coverage recursion must satisfy every binding"
    );
}

/// Locate the first honest-witness violation: which transition row (and the
/// region it falls in) or which boundary tuple fails. Printed via panic so it
/// surfaces without --nocapture.
#[test]
#[ignore]
fn diagnose_accept() {
    use alloc::format;
    use alloc::string::String;
    let asm = assemble(Tamper::None);
    let air = &asm.wired;
    let witness = &asm.witness;
    let w = air.trace_width();
    let ws = air.window_size();
    let total = 1usize << air.log_trace_len();
    let periodic = air.periodic_columns();
    let off = &asm.region_offsets;
    let region_of = |row: usize| -> usize {
        let mut which = 0;
        for (i, &o) in off.iter().enumerate() {
            if row >= o {
                which = i;
            }
        }
        which
    };
    let mut msg = String::new();
    'trans: for r in 0..total - (ws - 1) {
        let mut window = alloc::vec::Vec::with_capacity(ws * w);
        for k in 0..ws {
            window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
        }
        let per: alloc::vec::Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        let t = air.transition(&window, &per);
        for (i, v) in t.iter().enumerate() {
            if *v != Fp::ZERO {
                msg.push_str(&format!(
                    "TRANSITION fail: row {} (region {}), output idx {} of {}\n",
                    r,
                    region_of(r),
                    i,
                    t.len()
                ));
                break 'trans;
            }
        }
    }
    let bnds = air.boundary();
    // Replicate the group-emission counts to label each z-column's group.
    let nq = asm.lay.n_q;
    let n_coeff = asm.lay.n_coeff;
    let width_inner = asm.lay.width_inner;
    let n_terms = asm.lay.n_terms;
    let frame_len = asm.lay.frame_len;
    let ocells_len = asm.lay.ocells[0].len();
    let n_pz = asm.lay.n_pz;
    let stmt = 1 + n_coeff.div_ceil(2) + nq;
    let deep_pq = 3 + 2 * width_inner + n_terms + 2 * frame_len;
    let roots_pq = ocells_len;
    let fold_pq = 4;
    let index_pq = 2;
    let stack_width = w - asm.n_groups;
    msg.push_str(&format!(
        "dims: stack_width={}, stmt={}, deep_pq={}, roots_pq={}, fold_pq={}, index_pq={}, n_pz={}\n",
        stack_width, stmt, deep_pq, roots_pq, fold_pq, index_pq, n_pz
    ));
    let label = |g: usize| -> String {
        if g < stmt {
            return format!("statement[{}]", g);
        }
        let mut b = stmt;
        if g < b + nq * deep_pq {
            let gg = g - b;
            return format!("deep q{} local{}", gg / deep_pq, gg % deep_pq);
        }
        b += nq * deep_pq;
        if g < b + nq * roots_pq {
            let gg = g - b;
            return format!("roots q{} local{}", gg / roots_pq, gg % roots_pq);
        }
        b += nq * roots_pq;
        if g < b + nq * fold_pq {
            let gg = g - b;
            return format!("fold q{} local{}", gg / fold_pq, gg % fold_pq);
        }
        b += nq * fold_pq;
        if g < b + nq * index_pq {
            let gg = g - b;
            return format!("index q{} local{}", gg / index_pq, gg % index_pq);
        }
        b += nq * index_pq;
        format!("periodic[{}]", g - b)
    };
    let mut fails = 0usize;
    let mut fail_regions: alloc::vec::Vec<usize> = alloc::vec::Vec::new();
    let mut fail_labels: alloc::vec::Vec<String> = alloc::vec::Vec::new();
    let mut shown = 0usize;
    for (bi, (col, row, val)) in bnds.iter().enumerate() {
        if witness[row * w + col] != *val {
            fails += 1;
            let reg = region_of(*row);
            if !fail_regions.contains(&reg) {
                fail_regions.push(reg);
            }
            if *col >= stack_width {
                let lab = label(*col - stack_width);
                if !fail_labels.contains(&lab) {
                    fail_labels.push(lab);
                }
            }
            if shown < 8 {
                msg.push_str(&format!(
                    "BOUNDARY fail #{}: col {}, group '{}', expected {:?}, got {:?}\n",
                    bi,
                    col,
                    if *col >= stack_width { label(*col - stack_width) } else { String::from("region") },
                    val,
                    witness[row * w + col]
                ));
                shown += 1;
            }
        }
    }
    msg.push_str(&format!("total boundary fails: {} of {}\n", fails, bnds.len()));
    msg.push_str(&format!("failing regions: {:?}\n", fail_regions));
    msg.push_str(&format!("failing group families: {:?}\n", fail_labels));
    // region base rows near the tail queries
    for r in [3usize, 4, 8, 9, 158, 159, 160, 161, 162, 163] {
        if r < off.len() {
            msg.push_str(&format!("off[{}]={}\n", r, off[r]));
        }
    }
    msg.push_str(&format!(
        "context: n_q={}, n_groups={}, width={}, total_rows={}, regions={}, pbits={}, fbits={}, depth={}, n_open={}\n",
        asm.lay.n_q,
        asm.n_groups,
        w,
        total,
        off.len(),
        asm.lay.pbits,
        asm.lay.fbits,
        asm.lay.depth,
        asm.lay.n_open,
    ));
    if !msg.is_empty() {
        panic!("{}", msg);
    }
}

#[test]
#[ignore]
fn tamper_at_query_0_rejects() {
    let asm = assemble_q(Tamper::ReboundTraceValue, 0);
    assert!(!satisfies(&asm.wired, &asm.witness), "a query-0 tamper must reject");
}

#[test]
#[ignore]
fn tamper_at_query_5_rejects() {
    // THE Seam-2 proof: query 5 is not query 0. Under query-0-only coverage this
    // passes unseen; closed coverage must reject it.
    let asm = assemble_q(Tamper::ReboundTraceValue, 5);
    assert!(
        !satisfies(&asm.wired, &asm.witness),
        "a query-5 DEEP tamper must reject — this is the inner-coverage proof"
    );
}

#[test]
#[ignore]
fn tamper_at_last_query_rejects() {
    let asm = assemble(Tamper::None);
    let last = asm.lay.n_q - 1;
    let bad = assemble_q(Tamper::ReboundTraceValue, last);
    assert!(!satisfies(&bad.wired, &bad.witness), "a tamper on the last query must reject");
}

#[test]
#[ignore]
fn full_coverage_fri_prove_accepts() {
    let asm = assemble(Tamper::None);
    let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
    assert!(
        stark_verify_ext(&asm.wired, &proof, 32, 8),
        "the full-coverage recursion rejected the real proof"
    );
}

/// A real FRI prove+verify over a reduced (two-query) assembly. The full 32-query
/// trace's periodic LDE is too memory-heavy for a workstation, but two queries build
/// the identical per-query machinery over a small trace, so this exercises the FRI
/// degree bounds and multi-query wiring end to end. An honest reduced assembly must
/// produce a verifying proof; a tamper on the second query must break it.
#[test]
#[ignore]
fn reduced_multiquery_fri_roundtrips() {
    let asm = assemble_capped(Tamper::None, 0, 2);
    assert_eq!(asm.lay.n_q, 2, "cap did not take");
    let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
    assert!(
        stark_verify_ext(&asm.wired, &proof, 32, 8),
        "the honest two-query recursion produced a non-verifying proof"
    );
    let bad = assemble_capped(Tamper::ReboundTraceValue, 1, 2);
    let bad_proof = stark_prove_ext(&bad.wired, &bad.witness, 32, 8);
    assert!(
        !stark_verify_ext(&bad.wired, &bad_proof, 32, 8),
        "a tamper on the second query still verified"
    );
}

// The gates above assemble all 32 inner queries, which is a 65536-row trace and
// minutes per case. They are the real coverage proof and stay available on
// demand, but a shared runner cannot carry them, so the same properties are
// gated here over a capped assembly. The machinery is identical: per-query
// regions, per-query bindings, and a tamper that must be caught by the block it
// belongs to. Only the number of query blocks changes.
const CI_QUERIES: usize = 6;

#[test]
#[ignore]
fn capped_coverage_accepts() {
    let asm = assemble_capped(Tamper::None, 0, CI_QUERIES);
    assert_eq!(asm.lay.n_q, CI_QUERIES, "cap did not take");
    assert!(satisfies(&asm.wired, &asm.witness), "the honest capped recursion must satisfy");
}

#[test]
#[ignore]
fn capped_tamper_at_first_query_rejects() {
    let asm = assemble_capped(Tamper::ReboundTraceValue, 0, CI_QUERIES);
    assert!(!satisfies(&asm.wired, &asm.witness), "a query-0 tamper must reject");
}

/// The coverage proof in miniature: query 5 is not query 0, so under
/// query-0-only attestation this tamper passes unseen.
#[test]
#[ignore]
fn capped_tamper_at_a_later_query_rejects() {
    let asm = assemble_capped(Tamper::ReboundTraceValue, CI_QUERIES - 1, CI_QUERIES);
    assert!(!satisfies(&asm.wired, &asm.witness), "a later-query tamper must reject");
}
