/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The lints `#[allow(...)]` names (section 17.1), and the warnings each covers. A warning
 * whose primary span lies in an item that allows its lint is dropped.
 */

use alloc::format;
use alloc::string::String;

use crate::compiler::diag::{Diagnostics, Severity};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Attr, AttrArg};

/** Each lint and the codes of the warnings it covers. */
pub(crate) const LINTS: [(&str, &[&str]); 4] = [
    ("unused", &["W0001", "W0002"]),
    ("unreachable", &["W0003", "W0004"]),
    ("declassify", &["W0006"]),
    ("cost", &["W0100", "W0101", "W0102"]),
];

/** The codes the lint `name` covers, if it is one. */
pub(crate) fn lint(name: &str) -> Option<&'static [&'static str]> {
    LINTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, codes)| *codes)
}

/** `diags` without the warnings an `allowed` span and lint covers. */
pub(crate) fn drop_allowed(diags: Diagnostics, allowed: &[(Span, &str)]) -> Diagnostics {
    let mut out = Diagnostics::new();
    for d in diags.into_vec() {
        let at = d.span();
        let covered = d.severity == Severity::Warning
            && allowed.iter().any(|(s, name)| {
                s.file == at.file
                    && s.lo <= at.lo
                    && at.hi <= s.hi
                    && lint(name).is_some_and(|codes| codes.contains(&d.code.0))
            });
        if !covered {
            out.push(d);
        }
    }
    out
}

/** Why the `allow` attribute `a` is wrong, if it is: it names one or more known lints. */
pub(super) fn allow_problem(a: &Attr) -> Option<String> {
    let args = match (&a.args, &a.value) {
        (Some(args), None) if !args.is_empty() => args,
        _ => return Some(String::from("`allow` takes lint names, such as `unused`")),
    };
    for arg in args {
        match arg {
            AttrArg::Named { name, value: None } if lint(&name.name).is_some() => {}
            AttrArg::Named { name, value: None } => {
                return Some(format!("unknown lint `{}`", name.name))
            }
            _ => return Some(String::from("`allow` takes lint names")),
        }
    }
    None
}
