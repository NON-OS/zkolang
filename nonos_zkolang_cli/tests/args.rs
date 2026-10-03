/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every command reads its arguments strictly. The file was the first argument not starting
 * with `-`, so a flag's value placed first was read as the program; a flag with no value
 * was ignored, a repeated one took its first value, and `check` passed unprovable lengths.
 */

mod args_support;

use args_support::run::zk;

#[test]
fn flags_may_come_before_the_file() {
    let (ok, text) = zk(&["run", "--input", "3", "sq.zkl"]);
    assert!(ok && text.contains("outputs [9]"), "{text}");
    let (ok, text) = zk(&["run", "--", "-sq.zkl"]);
    assert!(ok && text.contains("outputs [4]"), "{text}");
}

#[test]
fn a_flag_is_known_given_once_and_has_a_value() {
    let cases: [(&[&str], &str); 6] = [
        (&["build", "sq.zkl", "--target"], "--target needs a value"),
        (&["build", "sq.zkl", "--out"], "--out needs a value"),
        (
            &["run", "sq.zkl", "--input", "1", "--input", "2"],
            "given twice",
        ),
        (&["run", "sq.zkl", "--inputs", "1"], "unknown flag --inputs"),
        (&["check", "sq.zkl", "long.zkl"], "more than one file given"),
        (&["check"], "no file given"),
    ];
    for (args, want) in cases {
        let (ok, text) = zk(args);
        assert!(!ok && text.contains(want), "{args:?}: {text}");
    }
}

#[test]
fn check_refuses_a_program_too_long_to_prove() {
    let (ok, text) = zk(&["check", "long.zkl"]);
    assert!(!ok && text.contains("more than a proof can hold"), "{text}");
}
