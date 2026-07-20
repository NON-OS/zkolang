/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! One instruction: fill the trace row for a single opcode and update the register
//! file. The dispatcher and each opcode handler are their own file; the witnessed
//! opcodes record the auxiliary value the AIR checks, and a violated constraint
//! returns `Unprovable` rather than panicking.

mod arith;
mod assert;
mod bool_op;
mod dispatch;
mod eq;
mod imm;
mod inp;
mod inv;
mod is_bool;
mod out;
mod sel;
