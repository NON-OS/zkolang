/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A compiler diagnostic as the protocol publishes one. */

use nonos_zkolang::compiler::diag::{Diagnostic, Severity};
use nonos_zkolang::compiler::source::SourceMap;

use super::json::Json;
use super::position::range;

/**
 * `d` as a protocol diagnostic, with the path of the file it is in; `None` if its span
 * names no file. Its labels, notes and help follow its message, a line each.
 */
pub(crate) fn lsp(map: &SourceMap, d: &Diagnostic) -> Option<(String, Json)> {
    let span = d.span();
    let file = map.file(span.file)?;
    let mut message = d.message.clone();
    for l in d.labels.iter().filter(|l| !l.message.is_empty()) {
        message.push('\n');
        message.push_str(&l.message);
    }
    for n in &d.notes {
        message.push_str("\nnote: ");
        message.push_str(n);
    }
    if let Some(h) = &d.help {
        message.push_str("\nhelp: ");
        message.push_str(h);
    }
    let severity = match d.severity {
        Severity::Error => 1,
        Severity::Warning => 2,
    };
    let at = range(&file.text, span.lo as usize, span.hi as usize);
    let fields = vec![
        ("range", at),
        ("severity", Json::num(severity)),
        ("code", Json::text(d.code.0)),
        ("source", Json::text("zkolang")),
        ("message", Json::Str(message)),
    ];
    Some((file.name.clone(), Json::obj(fields)))
}
