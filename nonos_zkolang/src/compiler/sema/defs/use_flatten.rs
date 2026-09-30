/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `use` trees flattened into single imports: `use a::{b, c::*}` is `use a::b` and
 * `use a::c::*`. Inside a group, `self` alone imports the group's prefix itself.
 */

use alloc::vec::Vec;

use super::use_join::join;
use super::{DefId, PendingImport};
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::{Ident, PathRoot, UseTree, Visibility};

/** The path a group's trees are relative to. */
pub(super) struct Prefix<'a> {
    pub(super) root: PathRoot,
    pub(super) segs: Vec<&'a Ident>,
}

/** Flatten `tree`, which follows `prefix`, into `out`. */
pub(super) fn flatten<'a>(
    tree: &'a UseTree,
    prefix: &Prefix<'a>,
    module: DefId,
    vis: Visibility,
    out: &mut Vec<PendingImport<'a>>,
    diags: &mut Diagnostics,
) {
    let (path, span) = match tree {
        UseTree::Single { path, span, .. }
        | UseTree::Glob { prefix: path, span }
        | UseTree::Nested {
            prefix: path, span, ..
        } => (path, *span),
    };
    let Some(full) = join(prefix, path, diags) else {
        return;
    };
    let alias = match tree {
        UseTree::Single { alias, .. } => alias.as_ref(),
        _ => None,
    };
    let import = |glob| PendingImport {
        module,
        root: full.root,
        segs: full.segs.clone(),
        glob,
        alias,
        vis,
        span,
    };
    match tree {
        UseTree::Single { .. } => out.push(import(false)),
        UseTree::Glob { .. } => out.push(import(true)),
        UseTree::Nested { trees, .. } => trees
            .iter()
            .for_each(|t| flatten(t, &full, module, vis, out, diags)),
    }
}
