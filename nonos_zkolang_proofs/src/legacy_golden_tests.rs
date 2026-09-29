/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The compiled form of every program in the tree, pinned. A registered circuit is fixed
//! by its program commitment, so a compiler change that moves the ops of a program that
//! already compiled correctly would silently move a registered verifier key. This walks
//! every `.zkl` under circuits, examples and stdlib, compiles it the way the command-line
//! tool does, and holds its commitment and instruction count against a committed table.
//! A change to the table is a reviewed change: set `ZKOLANG_BLESS=1` to rewrite it, and
//! the diff names every program whose compiled form moved.

use std::fs;
use std::path::{Path, PathBuf};

use nonos_zkolang::{commit, compile_source, expand_includes};

fn repo_root() -> PathBuf {
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

// Resolve an include from the file's directory or a `stdlib` folder in any ancestor, the
// command-line tool's rule, so the pinned form is the one a user builds.
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

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
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
    let compiled = expand_includes(&src, &mut lookup).and_then(|s| compile_source(&s));
    match compiled {
        Ok(ops) => format!("{rel} {} {}", ops.len(), hex(&commit(&ops))),
        Err(e) => format!("{rel} error {e:?}"),
    }
}

#[test]
fn every_program_compiles_to_its_pinned_form() {
    let root = repo_root();
    let mut files = Vec::new();
    for dir in ["circuits", "examples", "stdlib"] {
        collect(&root.join(dir), &mut files);
    }
    files.sort();
    let mut table = String::new();
    for f in &files {
        table.push_str(&line_for(&root, f));
        table.push('\n');
    }
    let golden = root.join("nonos_zkolang_proofs/golden/legacy-commitments.txt");
    if std::env::var_os("ZKOLANG_BLESS").is_some() {
        fs::write(&golden, &table).expect("write the golden table");
        return;
    }
    let pinned = fs::read_to_string(&golden).expect("read the golden table");
    for (want, got) in pinned.lines().zip(table.lines()) {
        assert_eq!(got, want, "a program's compiled form moved");
    }
    assert_eq!(
        pinned.lines().count(),
        table.lines().count(),
        "the set of programs changed"
    );
}
