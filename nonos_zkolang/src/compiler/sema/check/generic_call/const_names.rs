/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Whether a written type names a constant parameter, as a length or a constant argument;
 * and binding one to the value it stands at.
 */

use crate::compiler::syntax::ast::{ConstArg, GenericArg, Path, Type, TypeKind};

/** The constant parameters' names, and the value each has taken. */
pub(super) type Consts<'n> = (&'n [&'n str], &'n mut [Option<u32>]);

/** Whether the written type `t` names any of the constant parameters `names`. */
pub(super) fn names_const(t: &Type, names: &[&str]) -> bool {
    let named = |p: &Path| {
        p.as_ident()
            .is_some_and(|i| names.contains(&i.name.as_str()))
    };
    match &t.kind {
        TypeKind::Array(e, n) => {
            matches!(n, ConstArg::Path(p) if named(p)) || names_const(e, names)
        }
        TypeKind::Tuple(ts) => ts.iter().any(|e| names_const(e, names)),
        TypeKind::Labelled(_, e) | TypeKind::RefMut(e) => names_const(e, names),
        TypeKind::Path(p) => p
            .segments
            .iter()
            .flat_map(|s| s.generics.iter().flatten())
            .any(|g| match g {
                GenericArg::Type(t) => {
                    matches!(&t.kind, TypeKind::Path(q) if named(q)) || names_const(t, names)
                }
                GenericArg::Const(ConstArg::Path(q)) => named(q),
                GenericArg::Const(_) => false,
            }),
        _ => false,
    }
}

/** Bind the constant parameter `p` names, if it names one not bound yet, to `n`. */
pub(super) fn bind_one(p: &Path, n: u32, c: &mut Consts<'_>) {
    let k = p
        .as_ident()
        .and_then(|i| c.0.iter().position(|x| *x == i.name));
    if let Some(slot @ None) = k.and_then(|k| c.1.get_mut(k)) {
        *slot = Some(n);
    }
}
