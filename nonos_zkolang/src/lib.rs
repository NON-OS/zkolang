/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! zKolang verifiable-compute VM.
//!
//! A register VM over the Goldilocks field whose execution trace the NONOS in-kernel
//! transparent STARK proves. This crate is the VM, the instruction set, the trace
//! model, the step AIR, the language front-end, the native back-ends, and the NOX
//! fee. A register holds an `Fp`, the same scalar the STARK commits, so a run and its
//! proof are the same object with no second definition of the field.

#![no_std]

extern crate alloc;

mod air;
mod backend;
mod commit;
mod driver;
mod isa;
mod lang;
mod nox;
mod trace;
mod vkey;
mod vm;

pub use air::{BuildError, StepAir, TRACE_WIDTH};
pub use backend::{to_asm, to_c, to_python};
pub use commit::{commit, commit_limbs, serialize};
pub use driver::{
    evaluate, prove_program, prove_source, prove_source_with_inputs, prove_source_with_witness,
    Report, RunError,
};
pub use isa::{Op, Program, REGS};
pub use lang::{
    compile_source, compile_source_unoptimized, expand_includes, render_error, CompileError,
};
pub use nox::{quote, Quote, MICRONOX_PER_NOX};
pub use trace::{OpTag, Row, Trace};
pub use vkey::{
    periodic_root, program_log_t, registration_key, registration_root, verifier_key, KeyError,
    REGISTRATION_RATE,
};
pub use vm::{ProveError, Vm};
