// NONOS Operating System (AGPL-3.0-or-later)
//! The assembled witness-mode recursive verifier, gated the only way that
//! counts: it accepts the real inner proof and rejects targeted forgeries.
//! Each tamper is internally consistent where possible, so the rejection
//! exercises the binding under attack.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::{assemble, Tamper};

fn gate(tamper: Tamper) -> bool {
    let asm = assemble(tamper);
    let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
    stark_verify_ext(&asm.wired, &proof, 32, 8)
}

#[test]
#[ignore]
fn the_assembly_accepts_the_real_inner_proof() {
    let asm = assemble(Tamper::None);
    std::println!(
        "assembly: trace_width {}, log_trace_len {}, degree {}, transitions {}",
        asm.wired.trace_width(),
        asm.wired.log_trace_len(),
        asm.wired.constraint_degree(),
        asm.wired.num_transition()
    );
    let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
    assert!(
        stark_verify_ext(&asm.wired, &proof, 32, 8),
        "the assembled recursive verifier rejected the real proof"
    );
}

#[test]
#[ignore]
fn the_assembly_rejects_a_rebound_trace_value() {
    assert!(
        !gate(Tamper::ReboundTraceValue),
        "a rebound trace value verified"
    );
}

#[test]
#[ignore]
fn the_assembly_rejects_a_swapped_root() {
    assert!(
        !gate(Tamper::SwappedRoot),
        "a swapped authentication root verified"
    );
}

#[test]
#[ignore]
fn the_assembly_rejects_an_off_transcript_coefficient() {
    assert!(
        !gate(Tamper::OffTranscriptCoeff),
        "an off-transcript DEEP coefficient verified"
    );
}

/// The recursion over the deployed join-split: witness satisfaction first,
/// because it is the cheap gate — every transition vanishes and every
/// boundary holds, or the assembly is wrong before FRI enters the picture.
#[test]
#[ignore]
fn the_real_inner_assembly_satisfies() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::None, 2);
    std::println!(
        "real assembly: trace_width {}, log_trace_len {}, degree {}, transitions {}, publics {}",
        asm.wired.trace_width(),
        asm.wired.log_trace_len(),
        asm.wired.constraint_degree(),
        asm.wired.num_transition(),
        asm.publics.len()
    );
    assert!(
        crate::witness_satisfies::satisfies(&asm.wired, &asm.witness),
        "the real-inner assembly does not satisfy its own constraints"
    );
}

/// A periodic recompute off the composed point must fail through the real
/// inner too: the same binding that rejected it at the fixture rejects it here.
#[test]
#[ignore]
fn the_real_inner_assembly_rejects_a_tamper() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::PeriodicOffPoint, 2);
    assert!(
        !crate::witness_satisfies::satisfies(&asm.wired, &asm.witness),
        "an off-point periodic recompute satisfied the real-inner assembly"
    );
}

/// The row and width budget of the real-inner assembly: which regions own the
/// rows, which groups own the width. The shrink starts from this table, the
/// way the shield's did.
#[test]
#[ignore]
fn probe_real_budget() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::None, 4);
    let off = &asm.region_offsets;
    std::println!(
        "regions={} span={} width={} log_len={}",
        off.len(),
        asm.lay.span,
        asm.wired.trace_width(),
        asm.wired.log_trace_len()
    );
    let names = ["transcript", "compose", "fri-transcript", "periodic"];
    for i in 0..off.len() {
        let end = if i + 1 < off.len() {
            off[i + 1]
        } else {
            asm.lay.span
        };
        let name = if i < 4 {
            names[i]
        } else {
            match (i - 4) % 5 {
                0 => "deep",
                1 => "fold",
                2 => "auth",
                3 => "ip",
                _ => "fp",
            }
        };
        std::println!("region {i:3} {name:14} rows={}", end - off[i]);
    }
    let mut widths = asm.wired.group_widths();
    widths.sort_unstable();
    std::println!("groups={} widths={:?}", widths.len(), widths);
}

/// Locate the first honest-witness violation in the sidecar assembly: which
/// transition row in which region, or which boundary tuple. The accept gate
/// says something is over-constrained; this says what.
#[test]
#[ignore]
fn diagnose_real_accept() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::None, 2);
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
    for r in 0..total - (ws - 1) {
        let mut window = std::vec::Vec::with_capacity(ws * w);
        for k in 0..ws {
            window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
        }
        let per: std::vec::Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        let t = air.transition(&window, &per);
        for (i, v) in t.iter().enumerate() {
            if *v != Fp::ZERO {
                panic!(
                    "TRANSITION row {} (region {} base {}) output {} of {}",
                    r,
                    region_of(r),
                    off[region_of(r)],
                    i,
                    t.len()
                );
            }
        }
    }
    let mut fails = 0usize;
    for (bi, (col, row, val)) in air.boundary().iter().enumerate() {
        if witness[row * w + col] != *val {
            std::println!(
                "BOUNDARY #{bi}: col {} row {} (region {} base {}) expected {:?} got {:?}",
                col,
                row,
                region_of(*row),
                off[region_of(*row)],
                val,
                witness[row * w + col]
            );
            fails += 1;
            if fails >= 5 {
                break;
            }
        }
    }
    assert!(fails == 0, "{fails} boundary violations");
}


/// Every pre-collapse bind checked directly: for each swap, the two cells
/// must hold equal honest values. Fails by builder label and coordinates, so
/// a wrong tie names itself instead of surfacing as a grand product off one.
#[test]
#[ignore]
fn probe_bind_truth() {
    use crate::recursion_assembly::{assemble_real_capped, build_groups_for};
    let (asm, binds) = build_groups_for(2);
    let w = asm.wired.trace_width();
    for (i, o) in asm.region_offsets.iter().enumerate() {
        std::println!("off[{i}]={o}");
    }
    std::println!("span={} width={}", asm.lay.span, w);
    std::println!(
        "width_inner={} window_inner={} n_open={} depth={} n_terms={} pa_depth={} n_chunks={}",
        asm.lay.width_inner, asm.lay.window_inner, asm.lay.n_open, asm.lay.depth,
        asm.lay.n_terms, asm.lay.pa_depth, asm.lay.n_chunks
    );
    let witness = &asm.witness;
    let mut fails = 0usize;
    for b in &binds {
        for &(ra, ia, rb, ib) in &b.swaps {
            let (ca, cb) = (b.wired_cols[ia], b.wired_cols[ib]);
            let (va, vb) = (witness[ra * w + ca], witness[rb * w + cb]);
            if va != vb {
                let reg = |r: usize| {
                    asm.region_offsets.iter().rposition(|&o| r >= o).unwrap_or(0)
                };
                std::println!(
                    "BIND '{}' r{}(reg{}) c{} = {:?}  !=  r{}(reg{}) c{} = {:?}",
                    b.label, ra, reg(ra), ca, va, rb, reg(rb), cb, vb
                );
                fails += 1;
                if fails >= 10 {
                    assert!(false, "10+ bind violations");
                }
            }
        }
    }
    let _ = assemble_real_capped;
    assert!(fails == 0, "{fails} bind violations");
}

/// A bent opened periodic value must break the compress chain to the baked
/// root: the same cells feed the deep quotients, so if this passed, an opened
/// row would be decoration.
#[test]
#[ignore]
fn a_bent_opened_row_rejects() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::BentOpenedRow, 2);
    assert!(
        !crate::witness_satisfies::satisfies(&asm.wired, &asm.witness),
        "a bent opened periodic value satisfied the assembly"
    );
}

/// Two row values swapped: same multiset, different chain digest. Ordering is
/// part of the commitment or the schedule is forgeable by permutation.
#[test]
#[ignore]
fn swapped_row_values_reject() {
    use crate::recursion_assembly::assemble_real_capped;
    let asm = assemble_real_capped(Tamper::SwappedRowValues, 2);
    assert!(
        !crate::witness_satisfies::satisfies(&asm.wired, &asm.witness),
        "a permuted opened row satisfied the assembly"
    );
}
