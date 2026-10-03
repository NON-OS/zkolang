/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the interpreter reads while semantic analysis evaluates a constant. */

use super::cx::{Sema, State};
use crate::compiler::interp::{Env, Value};
use crate::compiler::sema::ty::Types;
use crate::compiler::tir::{ConstId, FnId, TFn};

impl<'a> Env for Sema<'a> {
    fn body(&self, f: FnId) -> Option<&TFn> {
        match &self.fns.get(f.0 as usize)?.body {
            State::Done(b) => Some(b),
            _ => None,
        }
    }

    fn konst(&self, c: ConstId) -> Option<&Value> {
        match &self.consts.get(c.0 as usize)?.state {
            State::Done((_, v)) => Some(v),
            _ => None,
        }
    }

    fn types(&self) -> &Types {
        &self.types
    }
}
