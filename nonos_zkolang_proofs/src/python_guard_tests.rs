/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The Python target raises wherever the proof would fail, under `python -O` too. Its
 * constraints were `assert` statements, which `-O` removes, and it returned 0 for an
 * inverse of zero, took any nonzero select condition as true, padded missing inputs with
 * zero and reduced negative or oversized ones.
 */

use std::process::Command;

use nonos_zkolang::{compile_source, to_python};

/** What `run(inputs)` returns or raises for each input list, under `python3 -O`. */
fn results(src: &str, inputs: &[&str]) -> Vec<String> {
    let ops = compile_source(src).expect("compile");
    let dir = std::env::temp_dir().join(format!("zkolang-py-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    std::fs::write(dir.join("m.py"), to_python(&ops)).expect("write");
    let cases = inputs.join(", ");
    let script = format!(
        "import m\nfor c in [{cases}]:\n    try:\n        print(m.run(c))\n    \
         except Exception as e:\n        print(type(e).__name__)\n"
    );
    let mut python = Command::new("python3");
    let out = python
        .current_dir(&dir)
        .args(["-O", "-c", &script])
        .output();
    let out = out.expect("python3");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(String::from)
        .collect()
}

#[test]
fn the_python_target_raises_where_the_proof_fails() {
    let src = "input x;\ninput y;\nassert x - 3;\noutput sel(y, 5, 7) + 3 / x;";
    let got = results(
        src,
        &["[3, 1]", "[4, 1]", "[3, 2]", "[3]", "[3, -1]", "[3, '1']"],
    );
    let want = [
        "[6]",
        "Unprovable",
        "Unprovable",
        "ValueError",
        "ValueError",
        "ValueError",
    ];
    assert_eq!(got, want);
    let inv = results(
        "input x;\noutput 1 / x;",
        &["[0]", "[18446744069414584321]"],
    );
    assert_eq!(inv, ["Unprovable", "ValueError"]);
}
