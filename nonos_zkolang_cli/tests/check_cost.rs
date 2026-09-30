/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `check --cost` reports each function's rows and the lines that take the most (section
 * 15.2), for edition 2026 only.
 */

use std::path::PathBuf;
use std::process::Command;

const PROGRAM: &str = "fn square_all(x: field) -> field {\n    let mut y = x;\n    for _ in 0..20 {\n        y = y * y;\n    }\n    y\n}\n\nfn main(a: public field) -> field {\n    square_all(a) + 1\n}\n";

fn program() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("zkolang-cli-cost-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("p.zkl");
    std::fs::write(&path, PROGRAM).expect("write program");
    path
}

fn zkolang(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .args(args)
        .output()
        .expect("run zkolang");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}

#[test]
fn cost_names_the_functions_and_the_busiest_line() {
    let p = program();
    let p = p.to_str().expect("utf-8");
    let (ok, text) = zkolang(&["check", p, "--edition", "2026", "--cost"]);
    assert!(ok, "{text}");
    let lines: Vec<&str> = text.lines().collect();
    let rows: usize = lines[0]
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .expect("rows");
    assert!(text.contains(&format!("ok  {rows} rows")), "{text}");
    assert!(
        lines[1].contains("inclusive exclusive  peak  function"),
        "{text}"
    );
    assert!(
        lines[2].ends_with("  main") && lines[3].ends_with("  square_all"),
        "{text}"
    );
    let busiest = lines
        .iter()
        .skip_while(|l| !l.starts_with("the lines"))
        .nth(1);
    assert!(
        busiest.is_some_and(|l| l.contains("p.zkl:4  y = y * y;")),
        "{text}"
    );
    let (ok, text) = zkolang(&["check", p, "--edition", "2025", "--cost"]);
    assert!(
        !ok && text.contains("--cost is for edition 2026 programs"),
        "{text}"
    );
}
