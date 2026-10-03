/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `zkolang lsp` spoken to as an editor would: it answers `initialize`, publishes the
 * diagnostics of an opened document at the protocol's positions and clears them once the
 * document is fixed, lays a document out on request, and exits cleanly after `shutdown`.
 */

use std::io::BufReader;
use std::process::{Command, Stdio};

mod lsp_support;

use lsp_support::wire::{next, open, send, URI};

const LEAK: &str = "fn main(t: public u32, s: secret u32) -> bool {\n    s >= t\n}\n";
const FIXED: &str = "fn main(t: public u32, s: secret u32) -> bool {\n    declassify(s >= t)\n}\n";
const MESSY: &str = "fn main(x: public u32) -> u32 {\nx * x\n}\n";

#[test]
fn the_server_diagnoses_fixes_formats_and_exits() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("start zkolang lsp");
    let mut to = child.stdin.take().expect("stdin");
    let mut from = BufReader::new(child.stdout.take().expect("stdout"));
    send(
        &mut to,
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{}}",
    );
    let init = next(&mut from);
    assert!(
        init.contains("\"id\":1") && init.contains("documentFormattingProvider"),
        "{init}"
    );
    send(&mut to, &open(LEAK, "textDocument/didOpen"));
    let diags = next(&mut from);
    assert!(diags.contains("\"code\":\"E0600\""), "{diags}");
    let at = "\"start\":{\"line\":1,\"character\":4},\"end\":{\"line\":1,\"character\":10}";
    assert!(diags.contains(at), "{diags}");
    send(&mut to, &open(FIXED, "textDocument/didChange"));
    let cleared = next(&mut from);
    assert!(cleared.contains("\"diagnostics\":[]"), "{cleared}");
    send(&mut to, &open(MESSY, "textDocument/didChange"));
    let _ = next(&mut from);
    send(&mut to, &format!("{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"textDocument/formatting\",\"params\":{{\"textDocument\":{{\"uri\":\"{URI}\"}},\"options\":{{\"tabSize\":4,\"insertSpaces\":true}}}}}}"));
    let edits = next(&mut from);
    assert!(
        edits.contains("\"newText\":\"fn main(x: public u32) -> u32 {\\n    x * x\\n}\\n\""),
        "{edits}"
    );
    send(
        &mut to,
        "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"shutdown\"}",
    );
    assert!(next(&mut from).contains("\"id\":3"));
    send(&mut to, "{\"jsonrpc\":\"2.0\",\"method\":\"exit\"}");
    assert!(child.wait().expect("wait").success());
}
