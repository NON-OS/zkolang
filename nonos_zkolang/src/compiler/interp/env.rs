/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the interpreter reads besides the expression it runs: bodies, constants, types. */

use super::Value;
use crate::compiler::sema::ty::Types;
use crate::compiler::tir::{ConstId, FnId, TFn};

/** The program around the code being run. */
pub trait Env {
    /** The body of function `f`. */
    fn body(&self, f: FnId) -> Option<&TFn>;
    /** The value of constant `c`, already evaluated. */
    fn konst(&self, c: ConstId) -> Option<&Value>;
    /** The program's types. */
    fn types(&self) -> &Types;
}
