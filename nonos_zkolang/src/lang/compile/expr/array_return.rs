/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Compiling an expression that produces an array. A function returns a vector when its body
//! is an array literal, or a block or call that ends in one, so a whole vector can be named,
//! built, and returned. Whether an expression produces an array is a static property of its
//! shape, checked before a `let` binds it as an array rather than a scalar.

use alloc::vec::Vec;

use super::super::compiler::{Compiler, MAX_INLINE};
use super::args::Arg;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Whether an expression produces an array rather than a single value: an array literal, a
    /// block or call that ends in one, or a name bound to an array and not shadowed by a
    /// scalar. The depth bound mirrors the inline bound, so a recursive call is not chased.
    pub(crate) fn produces_array(&self, e: &Expr) -> bool {
        self.produces_array_at(e, 0)
    }

    fn produces_array_at(&self, e: &Expr, depth: usize) -> bool {
        if depth > MAX_INLINE {
            return false;
        }
        match e {
            Expr::Array(_) => true,
            Expr::Block(_, result) => self.produces_array_at(result, depth),
            Expr::Var(n) => {
                self.lookup(n).is_none()
                    && self.loop_const(n).is_none()
                    && self.scalar_const(n).is_none()
                    && self.lookup_array(n).is_some()
            }
            Expr::Call(name, _) => self
                .fns
                .iter()
                .find(|f| f.name.as_str() == name)
                .map(|f| self.produces_array_at(&f.body, depth + 1))
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Compile an array-producing expression to its element registers. An array literal builds
    /// its elements, a name passes an existing array through, a block yields its result, and a
    /// call inlines a function whose body produces an array.
    pub(crate) fn expr_array(&mut self, e: &Expr) -> Result<Vec<u8>, CompileError> {
        match e {
            Expr::Array(elems) => {
                let mut regs = Vec::with_capacity(elems.len());
                for el in elems {
                    regs.push(self.expr(el)?.reg);
                }
                Ok(regs)
            }
            Expr::Var(n) => self
                .lookup_array(n)
                .map(|r| r.to_vec())
                .ok_or(CompileError::ArrayNotScalar),
            Expr::Block(locals, result) => {
                let mark = self.open_block(locals)?;
                let regs = self.expr_array(result)?;
                self.close_block(mark, &regs);
                Ok(regs)
            }
            Expr::Call(name, args) => self.call_array(name, args),
            _ => Err(CompileError::ArrayNotScalar),
        }
    }

    /// A call whose body produces an array: resolve, check arity and depth, open the parameter
    /// scope, compile the body in array mode, then close the scope keeping the element
    /// registers the array carries out.
    fn call_array(&mut self, name: &str, args: &[Expr]) -> Result<Vec<u8>, CompileError> {
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
        let argv: Vec<Arg> = self.eval_args(args)?;
        let saved = self.open_params(&def.params, &argv);
        self.inline_depth += 1;
        let regs = self.expr_array(&def.body)?;
        self.inline_depth -= 1;
        self.close_params(saved, &argv, &regs);
        Ok(regs)
    }
}
