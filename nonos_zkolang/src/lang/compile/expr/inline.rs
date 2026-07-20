/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Inlining a function body in a parameter-only scope.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val};
use crate::lang::parse::FnDef;
use crate::lang::CompileError;

impl Compiler {
    /// Swap in a parameter-only scope, compile the body, then restore. Dead argument
    /// temporaries are freed unless the result is one of them.
    pub(crate) fn inline_body(&mut self, def: &FnDef, argv: Vec<Val>) -> Result<Val, CompileError> {
        let scope: Vec<(String, u8)> = def
            .params
            .iter()
            .zip(&argv)
            .map(|(p, v)| (p.clone(), v.reg))
            .collect();
        let saved_syms = core::mem::replace(&mut self.syms, scope);
        let saved_loops = core::mem::take(&mut self.loop_consts);
        self.inline_depth += 1;
        let result = self.expr(&def.body)?;
        self.inline_depth -= 1;
        self.syms = saved_syms;
        self.loop_consts = saved_loops;
        let mut temp = result.temp;
        for v in &argv {
            if v.temp && v.reg != result.reg {
                self.free.push(v.reg);
            } else if v.temp {
                temp = true;
            }
        }
        Ok(Val {
            reg: result.reg,
            temp,
        })
    }
}
