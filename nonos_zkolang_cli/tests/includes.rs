/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An include resolves from the file that writes it, then a `stdlib` folder, then the
 * standard library in the binary. It used to resolve from the main file's directory only.
 */

use std::path::{Path, PathBuf};
use std::process::Command;

fn tree(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-inc-{name}-{}", std::process::id()));
    for (path, text) in files {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("dirs");
        std::fs::write(path, text).expect("write");
    }
    dir
}

fn run(cwd: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("run zkolang");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

#[test]
fn an_include_resolves_from_the_file_that_writes_it() {
    let a = "include \"b.zkl\";\nfn a(x) = b(x) + 1;";
    let files = [
        ("p.zkl", "include \"lib/a.zkl\";\ninput x;\noutput a(x);"),
        ("lib/a.zkl", a),
        ("lib/b.zkl", "fn b(x) = x * 3;"),
    ];
    let dir = tree("nested", &files);
    let (ok, text) = run(&dir, &["run", "p.zkl", "--input", "2"]);
    assert!(ok && text.contains("outputs [7]"), "{text}");
}

#[test]
fn a_library_keeps_its_own_includes() {
    let decoy = "const MIMC_C = [0, 0, 0, 0, 0, 0, 0, 0];\nfn mimc_round(s, c) = s + c;";
    let main = "include \"merkle.zkl\";\noutput compress2(1, 2);";
    let dir = tree("decoy", &[("p.zkl", main), ("hash.zkl", decoy)]);
    let ops = nonos_zkolang::check(main).expect("compile");
    let want = nonos_zkolang::evaluate(&ops, &[], &[]).expect("run");
    let (ok, text) = run(&dir, &["run", "p.zkl"]);
    assert!(ok, "{text}");
    assert!(text.contains(&format!("outputs {want:?}")), "{text}");
}

#[test]
fn an_escaping_or_missing_include_is_named() {
    let check = |path: &str| {
        let dir = tree(
            "paths",
            &[("p.zkl", &format!("include \"{path}\";\noutput 1;"))],
        );
        run(&dir, &["check", "p.zkl"])
    };
    for path in ["/etc/hostname", "../p.zkl", "lib/../../p.zkl"] {
        let (ok, text) = check(path);
        assert!(!ok && text.contains("absolute or climb"), "{path}: {text}");
    }
    let (ok, text) = check("no.zkl");
    assert!(!ok && text.contains("include \"no.zkl\" in ") && text.contains("not found"));
}
