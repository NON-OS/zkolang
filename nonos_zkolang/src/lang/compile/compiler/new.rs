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
    /// input and secret counts already known so secrets index after the public prefix
    /// and comparison advice indexes after the secrets.
    pub(crate) fn new(
        consts: Vec<ConstDef>,
        fns: Vec<FnDef>,
        n_public: u16,
        n_secret: u16,
    ) -> Compiler {
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
            n_secret,
            next_advice: 0,
            advice: Vec::new(),
        }
    }
}
