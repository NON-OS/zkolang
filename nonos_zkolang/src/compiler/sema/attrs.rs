/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking every attribute of a program (E0903), and recording where each lint is
 * allowed. An item's attributes cover the item; a module's inner ones cover the module.
 */

use alloc::format;

use super::attr_rules::attr_problem;
use super::cx::Sema;
use super::lints::LINTS;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Attr, AttrArg, Item, ItemKind};

impl<'a> Sema<'a> {
    /** Check the attributes of `items`, and `inner` of their module, which spans `span`. */
    pub(super) fn check_attrs(&mut self, items: &'a [Item], inner: &'a [Attr], span: Span) {
        for a in inner {
            self.check_attr(a, None, span);
        }
        for item in items {
            for (i, a) in item.attrs.iter().enumerate() {
                let earlier = item.attrs.get(..i).unwrap_or(&[]);
                if a.name.name != "allow" && earlier.iter().any(|b| b.name.name == a.name.name) {
                    let what = format!("`{}` is given twice", a.name.name);
                    let d = Diagnostic::error(Code::BAD_ATTRIBUTE, what, a.span, "given again");
                    self.diags.push(d);
                    continue;
                }
                self.check_attr(a, Some(item), item.span);
            }
            if let ItemKind::Mod(m) = &item.kind {
                if let Some(body) = &m.body {
                    self.check_attrs(body, &m.inner_attrs, item.span);
                }
            }
        }
    }

    /** Check one attribute, on `item` or inside a module when `None`, covering `span`. */
    fn check_attr(&mut self, a: &'a Attr, item: Option<&'a Item>, span: Span) {
        if let Some(problem) = attr_problem(a, item) {
            let unknown = problem.starts_with("unknown attribute");
            let d = Diagnostic::error(Code::BAD_ATTRIBUTE, problem, a.span, "not applied");
            let d = match unknown {
                true => d.with_help("the attributes are `test`, `should_fail`, `cfg(test)`, `allow(..)` and `deprecated = \"..\"`"),
                false => d,
            };
            self.diags.push(d);
            return;
        }
        if a.name.name != "allow" {
            return;
        }
        for arg in a.args.iter().flatten() {
            if let AttrArg::Named { name, .. } = arg {
                if let Some((lint, _)) = LINTS.iter().find(|(n, _)| *n == name.name) {
                    self.allowed.push((span, lint));
                }
            }
        }
    }
}
