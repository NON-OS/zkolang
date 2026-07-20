/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Construct a fresh compiler.

use alloc::vec::Vec;

use super::state::Compiler;
use crate::lang::parse::{ConstDef, FnDef};

impl Compiler {
    /// A fresh compiler over a program's constants and functions, with the public
    /// input count already known so secret inputs index after the public prefix.
    pub(crate) fn new(consts: Vec<ConstDef>, fns: Vec<FnDef>, n_public: u16) -> Compiler {
        Compiler {
            ops: Vec::new(),
            consts,
            fns,
            inline_depth: 0,
            syms: Vec::new(),
            loop_consts: Vec::new(),
            arrays: Vec::new(),
            next: 0,
            free: Vec::new(),
            n_public,
            next_public: 0,
            next_secret: 0,
            next_output: 0,
        }
    }
}
