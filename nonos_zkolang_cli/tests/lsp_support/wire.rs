/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Speaking the protocol to a server: framing a message each way, and a document's text. */

use std::io::{BufRead, BufReader, Read, Write};
use std::process::ChildStdout;

/** The document the tests open. */
pub const URI: &str = "file:///tmp/zkolang-lsp-test/a.zkl";

pub fn send(to: &mut impl Write, body: &str) {
    write!(to, "Content-Length: {}\r\n\r\n{body}", body.len()).expect("write");
    to.flush().expect("flush");
}

pub fn next(from: &mut BufReader<ChildStdout>) -> String {
    let mut len = 0;
    loop {
        let mut line = String::new();
        assert!(
            from.read_line(&mut line).expect("read") > 0,
            "the server closed"
        );
        match line.trim_end().strip_prefix("Content-Length: ") {
            Some(n) => len = n.parse().expect("a length"),
            None if line.trim_end().is_empty() => break,
            None => {}
        }
    }
    let mut body = vec![0; len];
    from.read_exact(&mut body).expect("body");
    String::from_utf8(body).expect("utf-8")
}

pub fn open(text: &str, method: &str) -> String {
    let text = text.replace('\n', "\\n");
    let doc = match method {
        "textDocument/didOpen" => format!("\"textDocument\":{{\"uri\":\"{URI}\",\"languageId\":\"zkolang\",\"version\":1,\"text\":\"{text}\"}}"),
        _ => format!("\"textDocument\":{{\"uri\":\"{URI}\",\"version\":2}},\"contentChanges\":[{{\"text\":\"{text}\"}}]"),
    };
    format!("{{\"jsonrpc\":\"2.0\",\"method\":\"{method}\",\"params\":{{{doc}}}}}")
}
