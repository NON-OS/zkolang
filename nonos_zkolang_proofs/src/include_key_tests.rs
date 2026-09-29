/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A file is spliced in once per file, not once per spelling, and an include resolves from
 * the file that writes it. Keyed on the text of the path, "lib/f.zkl" and "./lib/f.zkl"
 * spliced one file twice, doubling its inputs, and a library naming the program back
 * spliced the whole program in again.
 */

use std::collections::BTreeMap;

use nonos_zkolang::{compile_source, evaluate, expand_includes_from, Included};

/** A path with `.` and `..` segments folded away. */
fn normal(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for s in path.split('/') {
        match s {
            "" | "." => {}
            ".." => drop(parts.pop()),
            s => parts.push(s),
        }
    }
    format!("/{}", parts.join("/"))
}

/** Expand `/p/main.zkl` over an in-memory tree, resolving from each including file. */
fn outputs(files: &[(&str, &str)], public: &[u64]) -> Vec<u64> {
    let files: BTreeMap<&str, &str> = files.iter().copied().collect();
    let mut resolve = |from: &str, path: &str| {
        let dir = from.rsplit_once('/').map_or("", |(d, _)| d);
        let key = normal(&format!("{dir}/{path}"));
        let text = files.get(key.as_str())?.to_string();
        Some(Included { key, text })
    };
    let main = files["/p/main.zkl"];
    let src = expand_includes_from("/p/main.zkl", main, &mut resolve).expect("expand");
    evaluate(&compile_source(&src).expect("compile"), public, &[]).expect("run")
}

#[test]
fn an_include_resolves_from_the_file_that_writes_it() {
    let files = [
        (
            "/p/main.zkl",
            "include \"lib/a.zkl\";\ninput x;\noutput a(x);",
        ),
        ("/p/lib/a.zkl", "include \"b.zkl\";\nfn a(x) = b(x) + 1;"),
        ("/p/lib/b.zkl", "fn b(x) = x * 3;"),
        ("/p/b.zkl", "fn b(x) = x * 100;"),
    ];
    assert_eq!(outputs(&files, &[2]), vec![7]);
}

#[test]
fn a_file_is_spliced_once_however_it_is_named() {
    let main = "include \"lib/f.zkl\";\ninclude \"./lib/f.zkl\";\ninput x;\noutput x;";
    let files = [
        ("/p/main.zkl", main),
        ("/p/lib/f.zkl", "input y;\noutput y * 3;"),
    ];
    assert_eq!(outputs(&files, &[1, 9]), vec![3, 9]);
    /* A library that names the program back does not splice it in a second time. */
    let files = [
        (
            "/p/main.zkl",
            "include \"lib/c.zkl\";\ninput x;\noutput k(x);",
        ),
        ("/p/lib/c.zkl", "include \"../main.zkl\";\nfn k(v) = v + 1;"),
    ];
    assert_eq!(outputs(&files, &[4]), vec![5]);
}
