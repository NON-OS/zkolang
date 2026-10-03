/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The editor grammar keeps up with the language: the copy the VS Code extension ships is
 * the one under `grammars/`, and every keyword and reserved word of the lexer is named by
 * one of its rules.
 */

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push(rel);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

/** The words a source file of the lexer quotes: its keywords and its reserved words. */
fn quoted(src: &str) -> Vec<String> {
    src.split('"')
        .skip(1)
        .step_by(2)
        .filter(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_alphanumeric()))
        .map(String::from)
        .collect()
}

/** Whether `grammar` names `word` as one alternative of a rule. */
fn named(grammar: &str, word: &str) -> bool {
    ["(", "|"].iter().any(|before| {
        [")", "|"]
            .iter()
            .any(|after| grammar.contains(&format!("{before}{word}{after}")))
    })
}

#[test]
fn the_extension_ships_the_grammar() {
    assert_eq!(
        read("grammars/zkolang.tmLanguage.json"),
        read("editors/vscode/syntaxes/zkolang.tmLanguage.json"),
        "the two copies of the grammar differ"
    );
}

#[test]
fn every_keyword_and_reserved_word_is_highlighted() {
    let grammar = read("grammars/zkolang.tmLanguage.json");
    let words = quoted(&read("nonos_zkolang/src/compiler/syntax/keyword.rs"));
    assert!(words.len() > 50, "{} words", words.len());
    let missing: Vec<&String> = words.iter().filter(|w| !named(&grammar, w)).collect();
    assert!(missing.is_empty(), "no rule names {missing:?}");
}
