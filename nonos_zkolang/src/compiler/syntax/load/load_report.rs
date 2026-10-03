/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The modules whose files cannot be loaded (E0203, E0204, E0207). */

use alloc::format;

use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Ident;

/** The diagnostic for the module `name`, whose files `paths` both exist if `both`, else neither. */
pub(super) fn missing(name: &Ident, (flat, nested): (&str, &str), both: bool) -> Diagnostic {
    let n = &name.name;
    if both {
        Diagnostic::error(
            Code::MODULE_AMBIGUOUS,
            format!("module `{n}` has two files"),
            name.span,
            "declared here",
        )
        .with_help(format!("remove `{flat}` or `{nested}`"))
    } else {
        Diagnostic::error(
            Code::MODULE_NOT_FOUND,
            format!("no file for module `{n}`"),
            name.span,
            "declared here",
        )
        .with_help(format!("create `{flat}` or `{nested}`"))
    }
}

/** The diagnostic for the module `name`, nested deeper than `max`. */
pub(super) fn too_deep(name: &Ident, max: u32) -> Diagnostic {
    let what = format!("module `{}` is nested more than {max} deep", name.name);
    Diagnostic::error(Code::MODULE_DEPTH, what, name.span, "too deep")
        .with_help("a directory that holds itself nests without end")
}
