/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checking a manifest's sections, each known one once, and the keys `[package]` needs. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::manifest::Manifest;
use super::toml::Table;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;

/** A manifest being checked: what it says so far, and the sections and keys met. */
pub(super) struct Check<'d> {
    pub out: Manifest,
    pub diags: &'d mut Diagnostics,
    /** Each section and `[package]` key met, as `section.key`. */
    pub seen: Vec<String>,
}

impl Check<'_> {
    /** Report `what` at `at` (E0902). */
    pub(super) fn bad(&mut self, what: String, at: Span, label: &str) {
        self.diags
            .push(Diagnostic::error(Code::MANIFEST, what, at, label));
    }

    /** Whether `key` is met for the first time; a second time is reported at `at`. */
    pub(super) fn first(&mut self, key: String, at: Span) -> bool {
        if self.seen.contains(&key) {
            let what = match key.split_once('.') {
                Some((table, k)) => format!("`{k}` is given twice in `[{table}]`"),
                None => format!("`[{key}]` is given twice"),
            };
            self.bad(what, at, "given again");
            return false;
        }
        self.seen.push(key);
        true
    }

    /** One section. */
    pub(super) fn table(&mut self, t: &Table) {
        if t.name.is_empty() {
            for e in &t.entries {
                let what = format!("`{}` stands before any section", e.key);
                self.bad(what, e.span, "outside a section");
            }
            return;
        }
        if !self.first(t.name.clone(), t.span) {
            return;
        }
        match t.name.as_str() {
            "package" => t.entries.iter().for_each(|e| self.package(e)),
            "dependencies" => t.entries.iter().for_each(|e| self.dependency(e)),
            "cost" => t.entries.iter().for_each(|e| self.cost(e)),
            other => {
                let what = format!("unknown section `[{other}]`");
                self.bad(what, t.span, "not a manifest section");
            }
        }
    }
}
