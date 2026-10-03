/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The keys a manifest must give. */

use alloc::format;
use alloc::string::String;

use super::manifest_check::Check;
use crate::compiler::source::Span;

impl Check<'_> {
    /** Report each key `[package]` needs and lacks, at `at`. */
    pub(super) fn required(&mut self, at: Span) {
        if !self.seen.iter().any(|s| s == "package") {
            let what = String::from("the manifest has no `[package]` section");
            return self.bad(what, at, "the manifest");
        }
        for key in ["name", "version", "edition"] {
            if !self.seen.iter().any(|s| *s == format!("package.{key}")) {
                let what = format!("the manifest gives no `{key}` in `[package]`");
                self.bad(what, at, "the manifest");
            }
        }
    }
}
