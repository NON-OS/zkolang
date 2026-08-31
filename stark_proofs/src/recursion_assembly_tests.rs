// NONOS Operating System (AGPL-3.0-or-later)
//! The assembled witness-mode recursive verifier, gated the only way that
//! counts: it accepts the real inner proof and rejects targeted forgeries.
//! Each tamper is internally consistent where possible, so the rejection
//! exercises the binding under attack.

use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air};
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
    assert!(!gate(Tamper::ReboundTraceValue), "a rebound trace value verified");
}

#[test]
#[ignore]
fn the_assembly_rejects_a_swapped_root() {
    assert!(!gate(Tamper::SwappedRoot), "a swapped authentication root verified");
}

#[test]
#[ignore]
fn the_assembly_rejects_an_off_transcript_coefficient() {
    assert!(!gate(Tamper::OffTranscriptCoeff), "an off-transcript DEEP coefficient verified");
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
