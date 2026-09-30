/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The states semantic analysis keeps: of a check, of a signature, of an item's instances. */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::{GenArg, TyId};
use crate::compiler::tir::Labels;

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
