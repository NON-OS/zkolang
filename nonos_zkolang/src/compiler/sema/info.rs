/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What semantic analysis keeps of each function and constant item. */

use super::cx::{Sig, State};
use crate::compiler::interp::Value;
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{ConstDecl, FnDecl};
use crate::compiler::tir::{TConst, TFn};

/** A function of the program. */
#[derive(Clone, Debug)]
pub struct FnInfo<'a> {
    pub def: DefId,
    pub decl: &'a FnDecl,
    pub module: DefId,
    pub sig: Option<Sig>,
    pub body: State<TFn>,
    /** Whether the body checked without an error, so a constant may run it. */
    pub clean: bool,
}

/** A constant item of the program: its checked initializer and, once evaluated, its value. */
#[derive(Clone, Debug)]
pub struct ConstInfo<'a> {
    pub def: DefId,
    pub decl: &'a ConstDecl,
    pub module: DefId,
    pub ty: Option<TyId>,
    pub state: State<(TConst, Value)>,
}
