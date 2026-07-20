/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A constant declaration: a scalar value read by name, or a table read by a static
//! index. Both resolve at compile time, so a read costs one immediate and nothing
//! more; neither reaches the trace.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct ConstDef {
    pub name: String,
    pub values: Vec<u64>,
    /// A scalar holds one value read by name; a table holds several read by an index.
    pub scalar: bool,
}
