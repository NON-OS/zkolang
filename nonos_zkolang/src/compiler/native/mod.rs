/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Native targets of edition 2026: a built program's SSA, emitted as a C source file or a
 * Python module that computes what a proven run computes, without a prover. Each advice
 * value is found by its hint and each constraint is checked as the run goes, so a run
 * the proof would refuse fails natively too.
 */

mod c;
mod c_hint;
mod c_inst;
mod c_io;
mod c_live;
mod c_prelude;
mod leaves;
mod python;
mod python_inst;
mod python_io;
mod python_prelude;

pub use c::to_c;
pub use python::to_python;
