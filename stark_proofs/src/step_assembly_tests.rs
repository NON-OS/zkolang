// NONOS Operating System (AGPL-3.0-or-later)
//! The zkolang step AIR assembled into the full recursion. The fast gate checks
//! the witness satisfies every region constraint and every grand-product binding
//! without a FRI prove: a wrong binding pairs unequal cells, the grand product
//! fails to close, and its z=1 boundary is violated, so witness satisfaction is a
//! true test of the wiring. The ignored gate then runs the money-grade
//! prove/verify for the end-to-end proof.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::{assemble_step, Tamper};

/// The first constraint or binding the witness violates, or None. Boundaries
/// (cheap, and where a broken grand-product binding surfaces as a failed z=1
/// closure) are checked before the row sweep.
fn region_of(offsets: &[usize], row: usize) -> (usize, usize) {
    let mut reg = 0;
    for (i, &o) in offsets.iter().enumerate() {
        if row >= o {
            reg = i;
        }
    }
    (reg, row - offsets[reg])
}

fn find_violation(
    air: &WiredMultiExt,
    witness: &[Fp],
    offsets: &[usize],
) -> Option<alloc::string::String> {
    use alloc::format;
    let w = air.trace_width();
    for (col, row, val) in air.boundary() {
        let got = witness[row * w + col];
        if got != val {
            return Some(format!("boundary col={col} row={row}: got {got:?} want {val:?}"));
        }
    }
    let ws = air.window_size();
    let total = 1usize << air.log_trace_len();
    let periodic = air.periodic_columns();
    for r in 0..total - (ws - 1) {
        let mut window = alloc::vec::Vec::with_capacity(ws * w);
        for k in 0..ws {
            window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
        }
        let per: alloc::vec::Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        for (i, v) in air.transition(&window, &per).iter().enumerate() {
            if *v != Fp::ZERO {
                let (reg, local) = region_of(offsets, r);
                return Some(format!(
                    "transition row={r} (region {reg} local row {local}) constraint={i}"
                ));
            }
        }
    }
    None
}

fn witness_satisfies(air: &WiredMultiExt, witness: &[Fp]) -> bool {
    // The offsets only sharpen the message; satisfaction does not need them.
    find_violation(air, witness, &[0]).is_none()
}

#[test]
fn step_assembly_witness_satisfies_every_binding() {
    let asm = assemble_step(Tamper::None);
    std::eprintln!("region offsets: {:?}", asm.region_offsets);
    if let Some(why) = find_violation(&asm.wired, &asm.witness, &asm.region_offsets) {
        panic!("the honest step assembly violates a binding: {why}");
    }
}

#[test]
fn step_assembly_rejects_the_tamper_set_fast() {
    for t in [Tamper::ReboundTraceValue, Tamper::SwappedRoot, Tamper::OffTranscriptCoeff] {
        let asm = assemble_step(t);
        assert!(
            !witness_satisfies(&asm.wired, &asm.witness),
            "a tampered step assembly must violate a binding"
        );
    }
}

#[test]
#[ignore]
fn step_assembly_accepts_the_real_inner_proof() {
    let asm = assemble_step(Tamper::None);
    let proof = stark_prove_ext(&asm.wired, &asm.witness, 32, 8);
    assert!(
        stark_verify_ext(&asm.wired, &proof, 32, 8),
        "the assembled step recursion rejected the real proof"
    );
}
