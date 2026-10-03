/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The placement check over items: signatures, declared types and bodies. */

use super::check::{check_block, check_expr};
use super::types::{check_generic_params, check_type};
use crate::compiler::diag::Diagnostics;
use crate::compiler::syntax::ast::{Fields, FnDecl, Item, ItemKind, Param};

/** Report every statement form in a file that stands inside a larger expression. */
pub(in crate::compiler::syntax) fn check_items(items: &[Item], diags: &mut Diagnostics) {
    for item in items {
        match &item.kind {
            ItemKind::Fn(f) => check_fn(f, diags),
            ItemKind::Const(c) => {
                check_type(&c.ty, diags);
                check_expr(&c.value, false, diags);
            }
            ItemKind::Struct(s) => {
                check_generic_params(&s.generics, diags);
                check_fields(&s.fields, diags);
            }
            ItemKind::Enum(e) => {
                check_generic_params(&e.generics, diags);
                e.variants
                    .iter()
                    .for_each(|v| check_fields(&v.fields, diags));
            }
            ItemKind::TypeAlias(a) => {
                check_generic_params(&a.generics, diags);
                check_type(&a.ty, diags);
            }
            ItemKind::Mod(m) => check_items(m.body.as_deref().unwrap_or(&[]), diags),
            ItemKind::Impl(i) => {
                check_generic_params(&i.generics, diags);
                check_type(&i.self_ty, diags);
                check_items(&i.items, diags);
            }
            ItemKind::Use(_) => {}
        }
    }
}

fn check_fn(f: &FnDecl, diags: &mut Diagnostics) {
    check_generic_params(&f.generics, diags);
    for p in &f.params {
        if let Param::Typed { ty, .. } = p {
            check_type(ty, diags);
        }
    }
    if let Some(r) = &f.ret {
        check_type(r, diags);
    }
    check_block(&f.body, diags);
}

fn check_fields(fields: &Fields, diags: &mut Diagnostics) {
    match fields {
        Fields::Tuple(fs) | Fields::Named(fs) => fs.iter().for_each(|f| check_type(&f.ty, diags)),
        Fields::Unit => {}
    }
}
