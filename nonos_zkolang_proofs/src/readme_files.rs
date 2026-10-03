/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The files of the packages `README.md` shows, held in memory and read by path. */

use std::collections::BTreeMap;

use nonos_zkolang::compiler::syntax::load::Files;

/** The README's files by path. */
pub(crate) struct Shown(pub(crate) BTreeMap<String, String>);

/** `path` with each `.` dropped and each `..` taking back the step before it. */
fn normal(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for p in path.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(p),
        }
    }
    parts.join("/")
}

impl Files for Shown {
    fn read(&self, path: &str) -> Option<String> {
        self.0.get(&normal(path)).cloned()
    }
}

/** The manifest of the package `root` is in: `zkolang.toml` beside its `src`. */
pub(crate) fn manifest_of(root: &str) -> String {
    let dir = root.split_once("/src/").map_or("", |(d, _)| d);
    normal(&format!("{dir}/zkolang.toml"))
}
