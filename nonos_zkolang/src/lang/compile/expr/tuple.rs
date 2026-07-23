/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Compiling an expression in tuple mode, where it yields several values instead of one.
//! Because every call inlines, the arity of a function's return is not declared anywhere:
//! it is however many values its body produces when compiled this way. A destructuring
//! `let` and a function that returns a tuple are the only places this is reached.

use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val, MAX_INLINE};
use super::args::Arg;
use crate::lang::parse::{Expr, FnDef};
use crate::lang::CompileError;

impl Compiler {
    /// Compile an expression to the list of values it produces. A tuple produces its
    /// elements, a call produces whatever its inlined body produces, a block produces its
    /// result in tuple mode, and any scalar expression produces itself as a single value.
    pub(crate) fn expr_tuple(&mut self, e: &Expr) -> Result<Vec<Val>, CompileError> {
        match e {
            Expr::Tuple(elems) => {
                let mut out = Vec::with_capacity(elems.len());
                for el in elems {
                    out.push(self.expr(el)?);
                }
                Ok(out)
            }
            Expr::Call(name, args) => self.call_tuple(name, args),
            Expr::Block(locals, result) => self.block_tuple(locals, result),
            _ => Ok(alloc::vec![self.expr(e)?]),
        }
    }

    /// A call in tuple mode: resolve, check arity and inline depth, evaluate the arguments,
    /// then inline the body in tuple mode so a function that ends in a tuple returns several
    /// values.
    fn call_tuple(&mut self, name: &str, args: &[Expr]) -> Result<Vec<Val>, CompileError> {
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
        self.inline_body_tuple(&def, argv)
    }

    /// Inline a body in tuple mode: open the parameter scope from the arguments, compile the
    /// body to its list of values, then close the scope, keeping the registers the results
    /// carry out.
    fn inline_body_tuple(&mut self, def: &FnDef, args: Vec<Arg>) -> Result<Vec<Val>, CompileError> {
        let saved = self.open_params(&def.params, &args);
        self.inline_depth += 1;
        let results = self.expr_tuple(&def.body)?;
        self.inline_depth -= 1;
        let result_regs: Vec<u8> = results.iter().map(|v| v.reg).collect();
        self.close_params(saved, &args, &result_regs);
        Ok(results)
    }

    /// A block in tuple mode: open the local bindings, compile the result in tuple mode, then
    /// close the scope, keeping the registers the several results carry out.
    fn block_tuple(
        &mut self,
        locals: &[(alloc::vec::Vec<alloc::string::String>, Expr)],
        result: &Expr,
    ) -> Result<Vec<Val>, CompileError> {
        let mark = self.open_block(locals)?;
        let results = self.expr_tuple(result)?;
        let result_regs: Vec<u8> = results.iter().map(|v| v.reg).collect();
        self.close_block(mark, &result_regs);
        Ok(results)
    }
}
