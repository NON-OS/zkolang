/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Terminal output: colour only on a terminal, errors on standard error. */

use std::io::IsTerminal;

/**
 * Wrap text in an ANSI color, but only when standard output is a terminal, so piped or
 * captured output stays plain text.
 */
pub(crate) fn paint(s: &str, code: &str) -> String {
    if std::io::stdout().is_terminal() {
        format!("\x1b[{code}m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

/** Print an error to standard error and return the exit status for it. */
pub(crate) fn err(msg: &str) -> i32 {
    if std::io::stderr().is_terminal() {
        eprintln!("\x1b[1;31m{msg}\x1b[0m");
    } else {
        eprintln!("{msg}");
    }
    1
}

/** Bytes as lowercase hexadecimal. */
pub(crate) fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
