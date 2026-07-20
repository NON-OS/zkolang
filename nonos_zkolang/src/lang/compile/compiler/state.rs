/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The compiler state: the emitted op list, the constant, function, symbol, and
//! array tables, the inline-depth counter, the register allocator's high-water mark
//! and free pool, and the input and output index counters.

use alloc::string::String;
use alloc::vec::Vec;

use crate::isa::Op;
use crate::lang::parse::{ConstDef, FnDef};

pub(crate) struct Compiler {
    pub(crate) ops: Vec<Op>,
    pub(crate) consts: Vec<ConstDef>,
    pub(crate) fns: Vec<FnDef>,
    pub(crate) inline_depth: usize,
    pub(crate) syms: Vec<(String, u8)>,
    pub(crate) loop_consts: Vec<(String, u64)>,
    pub(crate) arrays: Vec<(String, Vec<u8>)>,
    pub(crate) next: u8,
    pub(crate) free: Vec<u8>,
    pub(crate) n_public: u16,
    pub(crate) next_public: u16,
    pub(crate) next_secret: u16,
    pub(crate) next_output: u16,
}
