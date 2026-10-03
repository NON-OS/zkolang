/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the interpreter reads while it runs a checked program. */

use crate::compiler::interp::{Env, Value};
use crate::compiler::sema::ty::Types;
use crate::compiler::tir::{ConstId, FnId, TFn, TProgram};

impl Env for TProgram {
    fn body(&self, f: FnId) -> Option<&TFn> {
        self.fns.get(f.0 as usize)
    }

    fn konst(&self, c: ConstId) -> Option<&Value> {
        self.values.get(c.0 as usize)
    }

    fn types(&self) -> &Types {
        &self.types
    }
}
