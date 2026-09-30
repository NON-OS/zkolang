/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The check of one body: a function's, a constant's, or a constant argument's. It holds
 * the locals and the names in scope, the types of integer literals not yet known, and
 * checks left for when every type is known.
 */

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::deferred::Deferred;
use super::matching::PatCx;
use super::vars::IntVars;
use crate::compiler::sema::cx::Sema;
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::{LocalId, TLocal};

/** The check of one body. */
pub struct FnCx<'s, 'a> {
    pub(crate) sema: &'s mut Sema<'a>,
    pub(crate) module: DefId,
    pub(crate) locals: Vec<TLocal>,
    /** For each local, whether its value is read somewhere. */
    pub(super) read: Vec<bool>,
    /** Each name in scope and the locals it has named, the innermost last. */
    pub(super) names: BTreeMap<String, Vec<LocalId>>,
    /** The names each open block declared, the innermost last. */
    pub(super) scopes: Vec<Vec<String>>,
    pub(crate) vars: IntVars,
    /** The type `return` checks against; `None` in a constant, where it may not stand. */
    pub(crate) ret: Option<TyId>,
    /** How many loops enclose the expression being checked. */
    pub(crate) loops: u32,
    pub(crate) deferred: Vec<Deferred>,
    pub(crate) pats: PatCx,
}

impl<'s, 'a> FnCx<'s, 'a> {
    /** A check in module `module`, of a body whose `return` takes `ret`. */
    pub(crate) fn new(sema: &'s mut Sema<'a>, module: DefId, ret: Option<TyId>) -> FnCx<'s, 'a> {
        FnCx {
            sema,
            module,
            locals: Vec::new(),
            read: Vec::new(),
            names: BTreeMap::new(),
            scopes: alloc::vec![Vec::new()],
            vars: IntVars::default(),
            ret,
            loops: 0,
            deferred: Vec::new(),
            pats: PatCx::default(),
        }
    }
}
