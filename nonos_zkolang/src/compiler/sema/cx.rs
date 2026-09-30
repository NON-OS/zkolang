/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The state of semantic analysis: the items, the types, the diagnostics, and each
 * function and constant, whose signature and body are checked on demand, so a constant
 * can use a function or constant declared after it.
 */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::compiler::diag::Diagnostics;
use crate::compiler::sema::defs::{DefId, Defs};
use crate::compiler::sema::ty::{GenArg, TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::tir::{ConstId, FnId, Labels};

pub use super::info::{ConstInfo, FnInfo};

/** Where the check of a body or constant stands. */
#[derive(Clone, Debug)]
pub enum State<T> {
    Unchecked,
    /** Being checked: meeting it again is a cycle. */
    Checking,
    Done(T),
    /** Checked with errors, which are reported. */
    Failed,
}

/** A function's parameter and result types, and the labels they write. */
#[derive(Clone, Debug)]
pub struct Sig {
    pub params: Vec<(TyId, Labels, bool)>,
    pub ret: TyId,
    pub ret_labels: Labels,
}

/** An item, and the generic arguments of one of its instances. */
pub type Instance = (DefId, Vec<GenArg>);

/** Each instance of a type's item lowered, with the labels it writes, or being lowered. */
pub type Lowered = BTreeMap<Instance, State<(TyId, Labels)>>;

/** Everything semantic analysis knows about one program. */
#[derive(Debug, Default)]
pub struct Sema<'a> {
    pub defs: Defs<'a>,
    pub types: Types,
    pub diags: Diagnostics,
    pub fns: Vec<FnInfo<'a>>,
    pub fn_of: BTreeMap<DefId, FnId>,
    pub consts: Vec<ConstInfo<'a>>,
    pub const_of: BTreeMap<DefId, ConstId>,
    /** Each instance of a type alias lowered, or being lowered. */
    pub aliases: Lowered,
    /** Each instance of a struct or enum lowered, or being lowered. */
    pub structs: Lowered,
    /** The generic parameters in scope and what each stands for, where one is. */
    pub generics: Vec<(alloc::string::String, GenArg)>,
    /** The functions of `impl` blocks, by the type and their name. */
    pub assoc: BTreeMap<(TyId, alloc::string::String), FnId>,
    /** The type `Self` names where a signature or body is being checked, if any. */
    pub self_ty: Option<TyId>,
    /** How many on-demand checks are open, one inside another. */
    pub depth: u32,
    /** Each span a lint is allowed in (section 17.1). */
    pub allowed: Vec<(Span, &'static str)>,
}
