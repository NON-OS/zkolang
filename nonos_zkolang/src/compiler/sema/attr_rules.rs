/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What each attribute takes and where it applies (section 17.1). `#[test]` and
 * `#[should_fail]` stand on a function, `#[cfg(test)]` and `#[allow(...)]` on any item,
 * `#[deprecated = "..."]` on a named item, and only `#![allow(...)]` inside a module.
 */

use alloc::format;
use alloc::string::String;

use super::attr_query::{has, is_test_arg};
use super::lints::allow_problem;
use crate::compiler::syntax::ast::{Attr, Item, ItemKind, Lit, Type, TypeKind};

/** Why `a`, on `item` or inside a module when `None`, is wrong, if it is. */
pub(super) fn attr_problem(a: &Attr, item: Option<&Item>) -> Option<String> {
    let name = a.name.name.as_str();
    let bare = a.args.is_none() && a.value.is_none();
    let fn_item = item.and_then(|i| match &i.kind {
        ItemKind::Fn(f) => Some(f),
        _ => None,
    });
    let problem = match (name, item) {
        ("allow", _) => return allow_problem(a),
        (_, None) => format!("`#![{name}]` does not apply to a module"),
        ("test", _) if !bare => String::from("`test` takes no arguments"),
        ("test", _) => match fn_item {
            None => String::from("`#[test]` applies to a function"),
            Some(f) if f.params.is_empty() && f.generics.is_empty() && unit(&f.ret) => return None,
            Some(_) => String::from("a test takes no parameters and returns `()`"),
        },
        ("should_fail", Some(i)) if bare && fn_item.is_some() && has(&i.attrs, "test") => {
            return None
        }
        ("should_fail", _) => String::from("`#[should_fail]` applies to a `#[test]` function"),
        ("cfg", _) if is_test_arg(a) => return None,
        ("cfg", _) => String::from("`cfg` takes `test`: `#[cfg(test)]`"),
        ("deprecated", Some(i)) => match (&a.value, a.args.is_none(), &i.kind) {
            (_, _, ItemKind::Use(_) | ItemKind::Impl(_)) => {
                String::from("`#[deprecated]` applies to a named item")
            }
            (Some(Lit::Str { .. }), true, _) => return None,
            _ => String::from("`deprecated` takes a message: `#[deprecated = \"...\"]`"),
        },
        _ => format!("unknown attribute `{name}`"),
    };
    Some(problem)
}

/** Whether a written result type is `()` or absent. */
fn unit(ret: &Option<Type>) -> bool {
    ret.as_ref()
        .is_none_or(|t| matches!(t.kind, TypeKind::Unit))
}
