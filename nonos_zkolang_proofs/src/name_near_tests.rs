/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A name that does not resolve is reported with the names close to it (section 4.4):
 * the items and imports in scope, locals, generic parameters, primitive types and the
 * prelude, and in a longer path the visible names of the module it has reached.
 */

use nonos_zkolang::compiler::diag::did_you_mean;

use crate::ui_run::report;

const PROGRAM: &str = include_str!("../ui/sema/near.zkl");

/** The suggestions among the help lines `src` reports, in order. */
fn suggestions(src: &str) -> Vec<String> {
    let got = report("sema/near.zkl", src);
    let helps = got.rendered.lines().map(str::trim_start);
    let helps = helps.filter_map(|l| l.strip_prefix("= help: did you mean "));
    helps.map(String::from).collect()
}

#[test]
fn the_closest_names_are_chosen_by_edit_distance() {
    let near = |name, names: &[&'static str]| did_you_mean(name, names.iter().copied());
    assert_eq!(
        near("cuont", &["count", "amount"]).as_deref(),
        Some("did you mean `count`?")
    );
    assert_eq!(
        near("point", &["pint", "Point"]).as_deref(),
        Some("did you mean `Point`?")
    );
    let four = ["xb", "ac", "bb", "ab_", "abcd"];
    let three = "did you mean `ab_`, `ac` or `bb`?";
    assert_eq!(near("ab", &four).as_deref(), Some(three));
    assert_eq!(near("abcdef", &["uvwxyz", "ab"]), None);
}

#[test]
fn an_unresolved_name_is_told_the_close_names_in_scope() {
    let want = [
        "`area`?",
        "`u32`?",
        "`Point`?",
        "`Option`?",
        "`count`?",
        "`LIMIT`?",
        "`area`?",
        "`total`?",
        "`Item`?",
    ];
    assert_eq!(suggestions(PROGRAM), want);
}
