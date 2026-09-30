/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Parse any text and check what the front end reports about it. */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

pub(crate) const WORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "match", "for", "in", "while", "limit", "struct", "enum",
    "impl", "mod", "use", "pub", "const", "type", "return", "break", "assert", "secret", "public",
    "x", "y", "u32", "field", "Self", "self", "_", "0", "7u8", "0xff", "1_000", "\"s\"", "(", ")",
    "[", "]", "{", "}", "<", ">", ",", ";", ":", "::", ".", "..", "..=", "=>", "->", "=", "+=",
    "+", "-", "*", "/", "%", "&", "&&", "|", "||", "!", "#", "@", "/* c */", "\n", "'", "\u{e9}",
];

/** Parse `src`, checking every diagnostic points inside it. */
pub(crate) fn check(src: &str) {
    let mut map = SourceMap::new();
    let id = map.add(String::from("f.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    parse_file(id, src, &lexed, &mut diags, &mut 0);
    for d in diags.items() {
        let s = d.span();
        assert!(
            s.lo <= s.hi && s.hi as usize <= src.len(),
            "{s:?} in {src:?}"
        );
        assert!(d.labels.iter().any(|l| l.primary), "{d:?}");
    }
}
