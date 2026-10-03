/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Open a block's scope: bind its locals in order. */

use alloc::string::String;
use alloc::vec::Vec;

use super::super::compiler::Compiler;
use super::block_close::BlockMark;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /**
     * Bind a block's locals in order, each visible to the ones after it. A local names one
     * value, or several with `let (x, y) = e;`, in which case the right side is compiled in
     * tuple mode and its arity must match the names. A local hides an outer array of its name
     * until the block closes. Returns the mark to restore at the end of the block.
     */
    pub(crate) fn open_block(
        &mut self,
        locals: &[(Vec<String>, Expr)],
    ) -> Result<BlockMark, CompileError> {
        let mark = BlockMark {
            syms: self.syms.len(),
            hidden: self.hidden_arrays.len(),
        };
        for (names, value) in locals {
            if names.len() == 1 {
                let v = self.expr(value)?;
                self.hide_array(&names[0]);
                self.syms.push((names[0].clone(), v.reg));
            } else {
                let vals = self.expr_tuple(value)?;
                if vals.len() != names.len() {
                    return Err(CompileError::TupleArity {
                        names: names.len(),
                        values: vals.len(),
                    });
                }
                for (i, (n, v)) in names.iter().zip(&vals).enumerate() {
                    if n == "_" {
                        /*
                         * A wildcard binds nothing; free its register if no live name holds
                         * it and no later name of this destructure is about to.
                         */
                        let later = names[i + 1..]
                            .iter()
                            .zip(&vals[i + 1..])
                            .any(|(m, w)| m != "_" && w.reg == v.reg);
                        if !later && !self.reg_in_use(v.reg) && !self.free.contains(&v.reg) {
                            self.free_reg(v.reg);
                        }
                        continue;
                    }
                    self.hide_array(n);
                    self.syms.push((n.clone(), v.reg));
                }
            }
        }
        Ok(mark)
    }
}
