/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading the attributes an item carries (section 17.1). */

use crate::compiler::syntax::ast::{Attr, AttrArg, Lit};

/** Whether `attrs` hold one named `name`. */
pub(crate) fn has(attrs: &[Attr], name: &str) -> bool {
    attrs.iter().any(|a| a.name.name == name)
}

/** Whether `attrs` hold `#[cfg(test)]`: the item is compiled only for tests. */
pub(crate) fn cfg_test(attrs: &[Attr]) -> bool {
    attrs.iter().any(|a| a.name.name == "cfg" && is_test_arg(a))
}

/** Whether `a`'s arguments are exactly `(test)`. */
pub(crate) fn is_test_arg(a: &Attr) -> bool {
    match a.args.as_deref() {
        Some([AttrArg::Named { name, value: None }]) => name.name == "test" && a.value.is_none(),
        _ => false,
    }
}

/** The message of `#[deprecated = "..."]` among `attrs`, if there is one. */
pub(crate) fn deprecated(attrs: &[Attr]) -> Option<&str> {
    attrs
        .iter()
        .find_map(|a| match (&a.value, a.name.name.as_str()) {
            (Some(Lit::Str { value, .. }), "deprecated") => Some(value.as_str()),
            _ => None,
        })
}
