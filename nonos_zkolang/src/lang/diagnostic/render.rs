/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Render an error with its line, column and a caret under the offending place. */

use alloc::format;
use alloc::string::String;

use super::locate::{locate_name, unknown_name};
use super::message::message;
use super::span_of::span_of;
use crate::lang::CompileError;

/** Render an error as a diagnostic over its source. */
pub fn render(src: &str, err: &CompileError) -> String {
    let msg = message(err);
    let locate = || unknown_name(err).and_then(|n| locate_name(src, n));
    let Some(at) = span_of(err).or_else(locate) else {
        return format!("error: {msg}");
    };
    let at = at.min(src.len());
    let line_start = src[..at].rfind('\n').map(|p| p + 1).unwrap_or(0);
    let line_end = src[at..].find('\n').map(|p| at + p).unwrap_or(src.len());
    let line = src[..at].matches('\n').count() + 1;
    let col = src[line_start..at].chars().count() + 1;
    let text = &src[line_start..line_end];
    let mut caret = String::new();
    for _ in 1..col {
        caret.push(' ');
    }
    caret.push('^');
    format!("error: {msg}\n  --> {line}:{col}\n   |\n   | {text}\n   | {caret}")
}
