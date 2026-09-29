/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A native program fails wherever the proof would. The C and assembly targets returned 0
 * for an inverse of zero, took any nonzero select condition as true, read a missing or
 * malformed argument as zero and reduced one past the modulus, so they printed outputs
 * for runs no proof exists for.
 */

use std::path::PathBuf;
use std::process::Command;

use nonos_zkolang::{compile_source, evaluate, to_asm, to_c};

const SRC: &str = "input x;\ninput y;\noutput sel(y, 5, 7) + 1 / x;";

/** Build the program for one target in its own directory and return the binary. */
fn binary(target: &str) -> PathBuf {
    let ops = compile_source(SRC).expect("compile");
    let (text, ext) = if target == "c" {
        (to_c(&ops), "c")
    } else {
        (to_asm(&ops), "S")
    };
    let dir = std::env::temp_dir().join(format!("zkolang-guard-{target}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let (src, bin) = (dir.join(format!("p.{ext}")), dir.join("p"));
    std::fs::write(&src, text).expect("write");
    let built = Command::new("cc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .expect("cc");
    assert!(built.success(), "{target} did not build");
    bin
}

#[test]
fn a_native_program_fails_where_the_proof_does() {
    let top = "18446744069414584320";
    let cases: [(&[&str], i32, &str); 10] = [
        (&["1", "1"], 0, "6"),
        (&[top, "1"], 0, "4"),
        (&["0", "1"], 4, ""),
        (&["1", "2"], 5, ""),
        (&["1"], 1, ""),
        (&["1", "1", "1"], 1, ""),
        (&["1", "abc"], 1, ""),
        (&["1", "-1"], 1, ""),
        (&["1", "18446744069414584321"], 1, ""),
        (&["1", ""], 1, ""),
    ];
    for target in ["c", "asm"] {
        let bin = binary(target);
        for (args, code, want) in cases {
            let out = Command::new(&bin).args(args).output().expect("run");
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            assert_eq!(
                (out.status.code(), text.as_str()),
                (Some(code), want),
                "{target} {args:?}"
            );
        }
    }
    /* The machine the prover runs refuses the same two runs. */
    let ops = compile_source(SRC).expect("compile");
    assert!(evaluate(&ops, &[0, 1], &[]).is_err());
    assert!(evaluate(&ops, &[1, 2], &[]).is_err());
}
