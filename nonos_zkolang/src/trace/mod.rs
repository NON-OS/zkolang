/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The execution trace: one row per VM step, recording every value a transition
//! constraint references. The VM fills these rows as it runs and the AIR reads them,
//! so a run and its proof agree on the same object.

mod at;
mod op_tag;
mod row;
mod trace_data;

pub use op_tag::OpTag;
pub use row::Row;
pub use trace_data::Trace;
