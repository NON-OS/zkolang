/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! An import waiting to be resolved: one name or glob of a `use`, with its full path. */

use alloc::vec::Vec;

use super::DefId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Ident, PathRoot, Visibility};

/** An import waiting to be resolved. */
#[derive(Clone, Debug)]
pub struct PendingImport<'a> {
    /** The module the `use` is in. */
    pub module: DefId,
    pub root: PathRoot,
    pub segs: Vec<&'a Ident>,
    /** Whether it imports every visible name of the module the path names. */
    pub glob: bool,
    pub alias: Option<&'a Ident>,
    pub vis: Visibility,
    pub span: Span,
}
