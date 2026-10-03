/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The keys of `[package]`: a name is an identifier other than `std` and `crate`, a
 * version is `major.minor.patch`, an edition is `"2025"` or `"2026"`, and an entry is a
 * path.
 */

use alloc::format;

use super::manifest_check::Check;
use super::manifest_names::{crate_name, edition, version};
use super::toml::{Entry, Value};

impl Check<'_> {
    /** A key of `[package]`. */
    pub(super) fn package(&mut self, e: &Entry) {
        if !self.first(format!("package.{}", e.key), e.span) {
            return;
        }
        let Value::Str(s) = &e.value else {
            return self.bad(
                format!("`{}` is a string", e.key),
                e.value_span,
                "not a string",
            );
        };
        let s = s.clone();
        let done = match e.key.as_str() {
            "name" => crate_name(&s).map(|()| self.out.name = s),
            "version" => version(&s).map(|()| self.out.version = s),
            "edition" => edition(&s).map(|n| self.out.edition = n),
            "entry" => {
                self.out.entry = Some(s);
                Ok(())
            }
            other => {
                let what = format!("`[package]` has no key `{other}`");
                return self.bad(what, e.span, "unknown key");
            }
        };
        if let Err(p) = done {
            self.bad(p, e.value_span, "not accepted");
        }
    }
}
