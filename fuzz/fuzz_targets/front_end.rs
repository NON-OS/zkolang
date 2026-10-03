/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 front end takes any text: the lexer and the parser return, and every
 * diagnostic they report points inside the text and has a primary label.
 */

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;
use nonos_zkolang_fuzz::text;

fuzz_target!(|data: &[u8]| {
    let src = text(data);
    let mut map = SourceMap::new();
    let id = map.add(String::from("f.zkl"), src.clone());
    let mut diags = Diagnostics::new();
    let lexed = lex(id, &src, &mut diags);
    parse_file(id, &src, &lexed, &mut diags, &mut 0);
    for d in diags.items() {
        let s = d.span();
        assert!(s.lo <= s.hi && s.hi as usize <= src.len(), "{s:?}");
        assert!(d.labels.iter().any(|l| l.primary), "{d:?}");
    }
});
