/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostics of an open document: its program checked as edition 2026, file by file. */

use std::collections::BTreeMap;

use nonos_zkolang::compiler::driver::{check, Source};
use nonos_zkolang::compiler::source::SourceMap;

use super::diagnostics::lsp;
use super::docs::Docs;
use super::json::Json;
use crate::cmd::manifest_for;

/** Whether the manifest text `m` declares edition 2025. */
fn edition_2025(m: &str) -> bool {
    m.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with("edition") && t.contains("\"2025\"")
    })
}

/**
 * The diagnostics of the program rooted at the open document `path`, by the path of the
 * file each is in, `path` always among them so its old ones are cleared. A file whose
 * manifest declares edition 2025 is not checked here.
 */
pub(crate) fn diagnose(docs: &Docs, path: &str) -> BTreeMap<String, Vec<Json>> {
    let mut out = BTreeMap::from([(path.to_string(), Vec::new())]);
    let Some(text) = docs.0.get(path) else {
        return out;
    };
    let manifest = manifest_for(path);
    if manifest.as_ref().is_some_and(|(_, m)| edition_2025(m)) {
        return out;
    }
    let src = Source {
        root: (path, text.clone()),
        manifest: manifest.as_ref().map(|(p, t)| (p.as_str(), t.clone())),
    };
    let mut map = SourceMap::new();
    let diags = match check(&mut map, docs, src) {
        Ok((_, warnings)) => warnings,
        Err(errors) => errors,
    };
    for d in diags.items() {
        if let Some((file, j)) = lsp(&map, d) {
            out.entry(file).or_default().push(j);
        }
    }
    out
}
