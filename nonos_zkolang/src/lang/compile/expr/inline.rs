/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Inlining a function body in a parameter-only scope.

use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val};
use super::args::Arg;
use crate::lang::parse::FnDef;
use crate::lang::CompileError;

impl Compiler {
    /// Open the parameter scope from the arguments, compile the body to one value, then restore
    /// the caller's scope. A scalar-argument temporary is freed unless the result is that
    /// register, in which case the result carries it out for the caller to free.
    pub(crate) fn inline_body(&mut self, def: &FnDef, args: Vec<Arg>) -> Result<Val, CompileError> {
        let saved = self.open_params(&def.params, &args);
        self.inline_depth += 1;
        let result = self.expr(&def.body)?;
        self.inline_depth -= 1;
        self.close_params(saved, &args, &[result.reg]);
        let mut temp = result.temp;
        for a in &args {
            if let Arg::Scalar(v) = a {
                if v.temp && v.reg == result.reg {
                    temp = true;
                }
            }
        }
        Ok(Val {
            reg: result.reg,
            temp,
        })
    }
}
