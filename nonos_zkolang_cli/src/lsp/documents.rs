/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the server does with a document: hold its text, publish its diagnostics, format it. */

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

use nonos_zkolang::compiler::fmt::format;

use super::check::diagnose;
use super::docs::Docs;
use super::json::Json;
use super::position::range;
use super::reply::notify;
use super::uri::{path_of, uri_of};

/** The open documents, and for each the other files it last published diagnostics for. */
#[derive(Default)]
pub(crate) struct Documents {
    pub(crate) docs: Docs,
    others: BTreeMap<String, BTreeSet<String>>,
}

/** The path of the document `params` names. */
pub(crate) fn doc_path(params: &Json) -> Option<String> {
    path_of(params.at(&["textDocument", "uri"])?.str()?)
}

impl Documents {
    /** Publish the diagnostics of the program rooted at `path`, clearing any now gone. */
    pub(crate) fn publish(&mut self, out: &mut impl Write, path: &str) -> io::Result<()> {
        let found = diagnose(&self.docs, path);
        let now: BTreeSet<String> = found.keys().filter(|f| *f != path).cloned().collect();
        let gone = self
            .others
            .insert(path.to_string(), now.clone())
            .unwrap_or_default();
        let cleared = gone.difference(&now).map(|f| (f.clone(), Vec::new()));
        for (file, list) in found.into_iter().chain(cleared.collect::<Vec<_>>()) {
            let params = Json::obj(vec![
                ("uri", Json::Str(uri_of(&file))),
                ("diagnostics", Json::Arr(list)),
            ]);
            notify(out, "textDocument/publishDiagnostics", params)?;
        }
        Ok(())
    }

    /** The edits that lay the document at `path` out; none if it is laid out or does not parse. */
    pub(crate) fn format(&self, path: &str) -> Json {
        let Some(text) = self.docs.0.get(path) else {
            return Json::Arr(Vec::new());
        };
        match format(text) {
            Ok(laid) if laid != *text => Json::Arr(vec![Json::obj(vec![
                ("range", range(text, 0, text.len())),
                ("newText", Json::Str(laid)),
            ])]),
            _ => Json::Arr(Vec::new()),
        }
    }
}
