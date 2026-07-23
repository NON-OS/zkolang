/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! An inlined function call: resolution and argument compilation.

use super::super::compiler::{Compiler, Val, MAX_INLINE};
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Resolve the function, check its arity and the inline depth, evaluate the
    /// arguments in the caller's scope, then hand off to the body inliner. Recursion
    /// is caught by the depth bound rather than looping forever.
    pub(crate) fn call(&mut self, name: &str, args: &[Expr]) -> Result<Val, CompileError> {
        let def = match self.fns.iter().find(|f| f.name.as_str() == name) {
            Some(f) => f.clone(),
            None => {
                return Err(CompileError::UnknownFunction {
                    name: alloc::string::String::from(name),
                })
            }
        };
        if def.params.len() != args.len() {
            return Err(CompileError::ArityMismatch {
                name: alloc::string::String::from(name),
                expected: def.params.len(),
                got: args.len(),
            });
        }
        if self.inline_depth >= MAX_INLINE {
            return Err(CompileError::RecursionTooDeep);
        }
        let argv = self.eval_args(args)?;
        self.inline_body(&def, argv)
    }
}
