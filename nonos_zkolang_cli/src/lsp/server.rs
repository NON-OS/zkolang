/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The session: each message read, answered or acted on, until the client says `exit`. */

use std::io::{self, Write};

use super::documents::{doc_path, Documents};
use super::json::Json;
use super::reply::{capabilities, fail, reply};
use super::rpc::read;

/** Serve the protocol on standard input and output until the client says `exit`. */
pub(crate) fn serve() -> Result<(), String> {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut input, mut out) = (stdin.lock(), stdout.lock());
    let (mut docs, mut shut) = (Documents::default(), false);
    while let Some(m) = read(&mut input).map_err(|e| format!("lsp: {e}"))? {
        let method = m.get("method").and_then(Json::str).unwrap_or("");
        if method == "exit" {
            return if shut {
                Ok(())
            } else {
                Err(String::from("lsp: exit before shutdown"))
            };
        }
        shut |= method == "shutdown";
        handle(&mut docs, &m, method, &mut out).map_err(|e| format!("lsp: {e}"))?;
    }
    Ok(())
}

fn handle(d: &mut Documents, m: &Json, method: &str, out: &mut impl Write) -> io::Result<()> {
    let params = m.get("params").cloned().unwrap_or(Json::Null);
    let path = doc_path(&params);
    if let Some(id) = m.get("id").cloned() {
        return match method {
            "initialize" => reply(out, id, capabilities()),
            "shutdown" => reply(out, id, Json::Null),
            "textDocument/formatting" => reply(out, id, d.format(&path.unwrap_or_default())),
            _ => fail(out, id, "-32601", "the method is not served"),
        };
    }
    let Some(path) = path else {
        return Ok(());
    };
    match method {
        "textDocument/didOpen" => {
            let text = params.at(&["textDocument", "text"]).and_then(Json::str);
            d.docs
                .0
                .insert(path.clone(), text.unwrap_or_default().to_string());
        }
        "textDocument/didChange" => {
            let changes = match params.get("contentChanges") {
                Some(Json::Arr(c)) => c.last().and_then(|c| c.get("text")).and_then(Json::str),
                _ => None,
            };
            if let Some(text) = changes {
                d.docs.0.insert(path.clone(), text.to_string());
            }
        }
        "textDocument/didClose" => {
            d.docs.0.remove(&path);
        }
        "textDocument/didSave" => {}
        _ => return Ok(()),
    }
    d.publish(out, &path)
}
