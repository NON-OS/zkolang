/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Render diagnostics as JSON, for `--json` and the language server. One object per
 * diagnostic, with one-based lines and columns.
 */

use alloc::format;
use alloc::string::String;

use super::diagnostic::{Diagnostic, Severity};
use super::quote::quote;
use crate::compiler::source::SourceMap;

/** A JSON array of the diagnostics. */
pub fn to_json(map: &SourceMap, ds: &[Diagnostic]) -> String {
    let mut out = String::from("[");
    for (i, d) in ds.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let severity = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        out.push_str(&format!(
            "{{\"severity\":\"{severity}\",\"code\":\"{}\",\"message\":{},\"labels\":[",
            d.code.0,
            quote(&d.message)
        ));
        for (j, l) in d.labels.iter().enumerate() {
            if j > 0 {
                out.push(',');
            }
            let (file, line, col, end_line, end_col) = match map.file(l.span.file) {
                Some(f) => {
                    let (a, b) = f.line_col(l.span.lo);
                    let (c, e) = f.line_col(l.span.hi);
                    (f.name.as_str(), a, b, c, e)
                }
                None => ("", 0, 0, 0, 0),
            };
            out.push_str(&format!(
                "{{\"file\":{},\"line\":{line},\"column\":{col},\"end_line\":{end_line},\"end_column\":{end_col},\"primary\":{},\"message\":{}}}",
                quote(file),
                l.primary,
                quote(&l.message)
            ));
        }
        out.push_str("],\"notes\":[");
        for (j, n) in d.notes.iter().enumerate() {
            if j > 0 {
                out.push(',');
            }
            out.push_str(&quote(n));
        }
        out.push_str("],\"help\":");
        match &d.help {
            Some(h) => out.push_str(&quote(h)),
            None => out.push_str("null"),
        }
        out.push('}');
    }
    out.push(']');
    out
}
