/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `/`-separated paths with `.` and `..` resolved, so one directory has one spelling. */

use alloc::string::String;
use alloc::vec::Vec;

/** `path` with each `.` dropped and each `..` taking back the part before it where one is. */
pub fn normalize(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for p in path.split('/') {
        match p {
            "." => {}
            "" if !parts.is_empty() => {}
            ".." if parts.last().is_some_and(|l| !matches!(*l, ".." | "")) => {
                parts.pop();
            }
            _ => parts.push(p),
        }
    }
    parts.join("/")
}
