/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Back-ends: the same compiled program, emitted to other targets. The front-end
//! turns zKolang source into a flat op list, and that list is target-independent, so
//! a program the STARK proves can equally be emitted as native x86_64 assembly, as
//! native C, or as Python and run without a prover. Every target computes over the
//! same field, so a program produces identical outputs whichever back-end runs it.

mod emit_asm;
mod emit_c;
mod emit_python;
mod outputs;
mod plan;

pub use emit_asm::to_asm;
pub use emit_c::to_c;
pub use emit_python::to_python;

pub(crate) use emit_c::PRELUDE as C_PRELUDE;
pub(crate) use emit_python::PRELUDE as PYTHON_PRELUDE;
pub(crate) use outputs::n_outputs;
pub(crate) use plan::Plan;
