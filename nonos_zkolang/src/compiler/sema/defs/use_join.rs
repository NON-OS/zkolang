/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A path inside a `use` group, joined to the group's path. */

use super::use_flatten::Prefix;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::syntax::ast::{Path, PathRoot};

/** `path` appended to `prefix`; a root other than a plain name inside a group is reported. */
pub(super) fn join<'a>(
    prefix: &Prefix<'a>,
    path: &'a Path,
    diags: &mut Diagnostics,
) -> Option<Prefix<'a>> {
    let inner = !prefix.segs.is_empty() || prefix.root != PathRoot::Plain;
    let own = path.segments.iter().map(|s| &s.ident);
    match path.root {
        _ if !inner => Some(Prefix {
            root: path.root,
            segs: own.collect(),
        }),
        PathRoot::Plain => Some(Prefix {
            root: prefix.root,
            segs: prefix.segs.iter().copied().chain(own).collect(),
        }),
        PathRoot::SelfModule if path.segments.is_empty() => Some(Prefix {
            root: prefix.root,
            segs: prefix.segs.clone(),
        }),
        _ => {
            let d = Diagnostic::error(Code::UNRESOLVED_NAME, "a path inside a `use` group continues the group's path", path.span, "cannot start a path here")
                .with_help("`crate`, `super`, `self` and `Self` begin a path only at the start of a `use`; `self` alone names the group's own path");
            diags.push(d);
            None
        }
    }
}
