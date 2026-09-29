/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The compiled form of every program in the tree, as a table: one line per `.zkl` under
 * circuits, examples and stdlib, with its instruction count and program commitment, or the
 * error it fails with. Includes resolve the way the command-line tool resolves them.
 */

use std::fs;
use std::path::{Path, PathBuf};

use nonos_zkolang::{commit, compile_source, expand_includes};

/** The repository root. */
pub(crate) fn repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    root
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "zkl") {
            out.push(path);
        }
    }
}

/** An include from the file's directory or a `stdlib` folder in any ancestor. */
fn resolve(dir: &Path, name: &str) -> Option<String> {
    let mut d = dir.to_path_buf();
    loop {
        for cand in [d.join(name), d.join("stdlib").join(name)] {
            if let Ok(s) = fs::read_to_string(&cand) {
                return Some(s);
            }
        }
        if !d.pop() {
            return None;
        }
    }
}

fn line_for(root: &Path, path: &Path) -> String {
    let rel = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let src = fs::read_to_string(path).unwrap_or_default();
    let dir = path.parent().map(PathBuf::from).unwrap_or_default();
    let mut lookup = |name: &str| resolve(&dir, name);
    match expand_includes(&src, &mut lookup).and_then(|s| compile_source(&s)) {
        Ok(ops) => {
            let hex: String = commit(&ops).iter().map(|x| format!("{x:02x}")).collect();
            format!("{rel} {} {hex}", ops.len())
        }
        Err(e) => format!("{rel} error {e:?}"),
    }
}

/** The table for the tree as it is now. */
pub(crate) fn corpus_table() -> String {
    let root = repo_root();
    let mut files = Vec::new();
    for dir in ["circuits", "examples", "stdlib"] {
        collect(&root.join(dir), &mut files);
    }
    files.sort();
    files.iter().map(|f| line_for(&root, f) + "\n").collect()
}
