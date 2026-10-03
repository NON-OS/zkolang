/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Paths over a primitive type (section 18.3), such as `u8::MAX` or `u32::checked_from`. */

use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{Path, PathRoot};
use crate::compiler::syntax::IntTy;

/** The primitive type a name spells, if it spells one. */
pub(crate) fn prim_type(name: &str) -> Option<TyId> {
    match name {
        "field" => Some(Types::FIELD),
        "bool" => Some(Types::BOOL),
        _ => IntTy::from_name(name).map(Types::int),
    }
}

/** The primitive type and the item name of a two-segment path over a primitive type. */
pub(crate) fn prim_path(p: &Path) -> Option<(TyId, &str)> {
    let [ty, item] = p.segments.as_slice() else {
        return None;
    };
    if p.root != PathRoot::Plain {
        return None;
    }
    Some((prim_type(&ty.ident.name)?, item.ident.name.as_str()))
}
