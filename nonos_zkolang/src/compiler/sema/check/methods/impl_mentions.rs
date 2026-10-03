/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Whether a written type names an `impl` block's parameters, and so cannot be lowered alone. */

use crate::compiler::syntax::ast::{GenericArg, Type, TypeKind};

/** Whether the written type `t` names any of the parameters `names`. */
pub(super) fn mentions(t: &Type, names: &[&str]) -> bool {
    match &t.kind {
        TypeKind::Path(p) => p.segments.iter().any(|s| {
            let arg = |g: &GenericArg| matches!(g, GenericArg::Type(t) if mentions(t, names));
            names.contains(&s.ident.name.as_str()) || s.generics.iter().flatten().any(arg)
        }),
        TypeKind::Tuple(ts) => ts.iter().any(|e| mentions(e, names)),
        TypeKind::Array(e, _) | TypeKind::Labelled(_, e) | TypeKind::RefMut(e) => {
            mentions(e, names)
        }
        _ => false,
    }
}
