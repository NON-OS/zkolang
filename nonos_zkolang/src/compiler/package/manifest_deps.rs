/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The keys of `[dependencies]`, each `name = { path = "dir" }`, and of `[cost]`. */

use alloc::format;
use alloc::string::String;

use super::manifest::Dep;
use super::manifest_check::Check;
use super::manifest_names::crate_name;
use super::toml::{Entry, Value};

impl Check<'_> {
    /** One dependency, `name = { path = "dir" }`. */
    pub(super) fn dependency(&mut self, e: &Entry) {
        if !self.first(format!("dependencies.{}", e.key), e.span) {
            return;
        }
        if let Err(p) = crate_name(&e.key) {
            return self.bad(p, e.span, "not a crate name");
        }
        let path = match &e.value {
            Value::Table(t) if t.len() == 1 => t.first().and_then(|p| match (&*p.key, &p.value) {
                ("path", Value::Str(s)) => Some(s.clone()),
                _ => None,
            }),
            _ => None,
        };
        let Some(path) = path else {
            let what = String::from("a dependency is `{ path = \"dir\" }`");
            return self.bad(what, e.value_span, "not a path dependency");
        };
        let (name, span) = (e.key.clone(), e.span);
        self.out.deps.push(Dep { name, path, span });
    }

    /** A threshold of `[cost]`. */
    pub(super) fn cost(&mut self, e: &Entry) {
        if !self.first(format!("cost.{}", e.key), e.span) {
            return;
        }
        let slot = match e.key.as_str() {
            "unroll_warn" => &mut self.out.unroll_warn,
            "dyn_index_warn" => &mut self.out.dyn_index_warn,
            other => return self.bad(format!("`[cost]` has no key `{other}`"), e.span, "unknown"),
        };
        match e.value {
            Value::Int(n) => *slot = Some(n),
            _ => self.bad(
                format!("`{}` is an integer", e.key),
                e.value_span,
                "not an integer",
            ),
        }
    }
}
