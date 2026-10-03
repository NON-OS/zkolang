/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The ABI of a checked program's `main`. */

use alloc::vec::Vec;

use super::leaf::{leaves, Leaf};
use crate::compiler::syntax::ast::Label;
use crate::compiler::tir::TProgram;

/** The leaves of `main`'s public parameters, its secret ones, and its result. */
pub fn abi_of(p: &TProgram) -> (Vec<Leaf>, Vec<Leaf>, Vec<Leaf>) {
    let (mut public, mut secret, mut output) = (Vec::new(), Vec::new(), Vec::new());
    let Some(f) = p.main.and_then(|m| p.fns.get(m.0 as usize)) else {
        return (public, secret, output);
    };
    for param in &f.params {
        if let Some(l) = f.locals.get(param.local.0 as usize) {
            let into = match l.labels.whole() {
                Some(Label::Secret) => &mut secret,
                _ => &mut public,
            };
            leaves(&p.types, l.ty, into);
        }
    }
    leaves(&p.types, f.ret, &mut output);
    (public, secret, output)
}
