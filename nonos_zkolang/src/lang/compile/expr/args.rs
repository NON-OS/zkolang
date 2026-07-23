/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Call arguments and the parameter scope they open. An argument is a single value or, when
//! it names an array, that whole array, so a function can take a vector as a parameter. The
//! scope a call opens is restored when its body is done, and an array parameter's registers,
//! which belong to the caller, are never freed by the callee.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val};
use crate::lang::parse::Expr;
use crate::lang::CompileError;

/// One evaluated argument: a single value, or a whole array passed by name.
pub(crate) enum Arg {
    Scalar(Val),
    Array(Vec<u8>),
}

/// The caller scope a call saved, to restore when the body is compiled.
pub(crate) struct SavedScope {
    syms: Vec<(String, u8)>,
    loops: Vec<(String, u64)>,
    arrays_len: usize,
}

impl Compiler {
    /// Evaluate each argument. A name bound to an array becomes that array's registers, so it
    /// passes as a vector; everything else is a scalar value.
    pub(crate) fn eval_args(&mut self, args: &[Expr]) -> Result<Vec<Arg>, CompileError> {
        let mut out = Vec::with_capacity(args.len());
        for a in args {
            if let Expr::Var(n) = a {
                // A scalar binding shadows an array of the same name, the same precedence a
                // bare reference uses, so a parameter named like a caller's array is passed by
                // value, not resolved to that array. Only a name that is an array and nothing
                // else passes as a vector.
                let shadowed = self.lookup(n).is_some()
                    || self.loop_const(n).is_some()
                    || self.scalar_const(n).is_some();
                if !shadowed {
                    if let Some(regs) = self.lookup_array(n) {
                        out.push(Arg::Array(regs.to_vec()));
                        continue;
                    }
                }
            }
            out.push(Arg::Scalar(self.expr(a)?));
        }
        Ok(out)
    }

    /// Open a parameter-only scope: bind each scalar parameter to its value's register and each
    /// array parameter to its argument's registers, then swap the scope in. The handle returned
    /// restores the caller's scope.
    pub(crate) fn open_params(&mut self, params: &[String], args: &[Arg]) -> SavedScope {
        let arrays_len = self.arrays.len();
        let mut scope: Vec<(String, u8)> = Vec::new();
        for (p, a) in params.iter().zip(args) {
            match a {
                Arg::Scalar(v) => scope.push((p.clone(), v.reg)),
                Arg::Array(regs) => self.arrays.push((p.clone(), regs.clone())),
            }
        }
        let syms = core::mem::replace(&mut self.syms, scope);
        let loops = core::mem::take(&mut self.loop_consts);
        SavedScope {
            syms,
            loops,
            arrays_len,
        }
    }

    /// Restore the caller's scope, drop the parameter arrays without freeing their registers,
    /// since an array argument's registers belong to the caller, then free the scalar-argument
    /// temporaries that no result carries out.
    pub(crate) fn close_params(&mut self, saved: SavedScope, args: &[Arg], result_regs: &[u8]) {
        self.syms = saved.syms;
        self.loop_consts = saved.loops;
        self.arrays.truncate(saved.arrays_len);
        for a in args {
            if let Arg::Scalar(v) = a {
                if v.temp && !result_regs.contains(&v.reg) && !self.free.contains(&v.reg) {
                    self.free.push(v.reg);
                }
            }
        }
    }
}
