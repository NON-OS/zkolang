/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A constant table: a name bound to a fixed list of field values, read at compile
//! time by a static index, so it costs nothing at proof time beyond one immediate per
//! read.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct ConstDef {
    pub name: String,
    pub values: Vec<u64>,
}
