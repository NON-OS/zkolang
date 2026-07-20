/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Constant tables and the compile-time index that reads them. A table is a fixed
//! list of field values named once and read by a static index; both are known at
//! compile time, so a read resolves to a single value and the table never reaches
//! the trace. One concern per file: the lookup, the index fold, and the resolve.

mod eval;
mod lookup;
mod resolve;
