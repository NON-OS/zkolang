/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A value no arm covers, written as the pattern that would cover it. */

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::domain::parts;
use super::pat::{Ctor, Pat};
use crate::compiler::sema::ty::{Form, TyId, TyKind, Types};

/** The pattern `p`, which takes a value of type `ty`, as written. */
pub(super) fn show(types: &Types, ty: TyId, p: &Pat) -> String {
    let (c, subs) = match p {
        Pat::Wild => return String::from("_"),
        Pat::Or(alts) => {
            let each: Vec<String> = alts.iter().map(|a| show(types, ty, a)).collect();
            return each.join(" | ");
        }
        Pat::Ctor(c, subs) => (*c, subs),
    };
    let tys = parts(types, ty, c);
    let each: Vec<String> = subs
        .iter()
        .enumerate()
        .map(|(i, q)| show(types, tys.get(i).copied().unwrap_or(Types::ERROR), q))
        .collect();
    match (c, types.kind(ty)) {
        (Ctor::Bool(b), _) => b.to_string(),
        (Ctor::Range(a, b), _) if a == b => format!("{a}"),
        (Ctor::Range(a, b), _) => format!("{a}..={b}"),
        (Ctor::Single, TyKind::Unit) => String::from("()"),
        (Ctor::Single, TyKind::Tuple(_)) if each.len() == 1 => format!("({},)", each.join("")),
        (Ctor::Single, TyKind::Tuple(_)) => format!("({})", each.join(", ")),
        (Ctor::Single, TyKind::Array(..)) => format!("[{}]", each.join(", ")),
        (Ctor::Single, _) | (Ctor::Variant(_), _) => shape(types, ty, c, &each),
    }
}

/** A struct or variant pattern of `ty`, built with `c`, its fields written as `each`. */
fn shape(types: &Types, ty: TyId, c: Ctor, each: &[String]) -> String {
    let Some(adt) = types.adt(ty) else {
        return String::from("_");
    };
    let tag = if let Ctor::Variant(t) = c { t } else { 0 };
    let Some(v) = adt.variants.get(tag as usize) else {
        return String::from("_");
    };
    /* A pattern names a generic item without its arguments, which the scrutinee settles. */
    let name = match adt.is_enum {
        true => format!("{}::{}", adt.name, v.name),
        false => adt.name.clone(),
    };
    match v.form {
        Form::Unit => name,
        Form::Tuple => format!("{name}({})", each.join(", ")),
        Form::Named if each.iter().all(|e| e == "_") => format!("{name} {{ .. }}"),
        Form::Named => {
            let names = v.fields.iter().map(|f| f.name.as_deref().unwrap_or("_"));
            let fs: Vec<String> = names.zip(each).map(|(n, e)| format!("{n}: {e}")).collect();
            format!("{name} {{ {} }}", fs.join(", "))
        }
    }
}
