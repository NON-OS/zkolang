/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What semantic analysis keeps of each function and constant item. */

use alloc::vec::Vec;

use super::cx::{Sig, State};
use crate::compiler::interp::Value;
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{ConstDecl, FnDecl, GenericParam, Type};
use crate::compiler::tir::{FnId, TConst, TFn};

/** A function of the program. */
#[derive(Clone, Debug)]
pub struct FnInfo<'a> {
    pub def: DefId,
    pub decl: &'a FnDecl,
    pub module: DefId,
    /** The type whose `impl` block declares it, if one does. */
    pub owner: Option<TyId>,
    pub sig: Option<Sig>,
    pub body: State<TFn>,
    /** Whether the body checked without an error, so a constant may run it. */
    pub clean: bool,
    /** Whether it is a generic function's template, checked only in its instances. */
    pub template: bool,
    /** An instance's generic arguments, in the order of its item's parameters. */
    pub args: Vec<GenArg>,
    /** For an instance, the function whose call made it, if one did, and the call. */
    pub origin: Option<(Option<FnId>, Span)>,
    /** The generic parameters of the `impl` block that declares it, before its own. */
    pub impl_generics: &'a [GenericParam],
    /** The type a generic `impl` block is for, as written, which names its parameters. */
    pub impl_self: Option<&'a Type>,
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
