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
use crate::compiler::tir::{ConstId, FnId};

pub use super::cx_state::{Instance, Lowered, Sig, State};
pub use super::info::{ConstInfo, FnInfo};

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
    /** Each instance of a generic function, by its template and arguments. */
    pub instances: BTreeMap<(FnId, Vec<GenArg>), FnId>,
    /** The functions of `impl` blocks, by the type and their name. */
    pub assoc: BTreeMap<(TyId, alloc::string::String), FnId>,
    /** The functions of generic `impl` blocks, by the struct or enum and their name. */
    pub generic_assoc: BTreeMap<(DefId, alloc::string::String), FnId>,
    /** The type `Self` names where a signature or body is being checked, if any. */
    pub self_ty: Option<TyId>,
    /** How many on-demand checks are open, one inside another. */
    pub depth: u32,
    /** Each span a lint is allowed in (section 17.1). */
    pub allowed: Vec<(Span, &'static str)>,
}
