/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What the fuzz targets share: turning the fuzzer's bytes into source text, and into the
 * small input values a program is run on.
 */

/** Words and marks the token mode strings together, so a run reaches the parser's depths. */
const WORDS: &[&str] = &[
    "fn ", "let ", "mut ", "if ", "else ", "match ", "for ", "in ", "while ", "limit ", "struct ",
    "enum ", "impl ", "mod ", "use ", "pub ", "const ", "type ", "return ", "break ", "assert ",
    "secret ", "public ", "main", "x", "y", "T", "N", "u8", "u32", "u64", "i64", "usize", "field",
    "bool", "Self", "self", "Option", "Some", "None", "std", "_", "0", "1", "7", "255", "0xff",
    "1_000", "true", "false", "\"s\"", "(", ")", "[", "]", "{", "}", "<", ">", ",", ";", ":", "::",
    ".", "..", "..=", "=>", "->", "=", "+=", "+", "-", "*", "/", "%", "&", "&&", "|", "||", "^",
    "!", "==", "!=", "<=", ">=", "<<", ">>", "#[test]", "/* c */", "/// d\n", "\n", " ", "'",
    "\u{e9}",
];

/**
 * The text of `data`. An even first byte reads the rest as UTF-8, replacing what is not, so
 * a seed program mutates in place; an odd one reads each byte as a word of `WORDS`.
 */
pub fn text(data: &[u8]) -> String {
    match data.split_first() {
        None => String::new(),
        Some((mode, rest)) if mode % 2 == 0 => String::from_utf8_lossy(rest).into_owned(),
        Some((_, rest)) => rest
            .iter()
            .map(|b| WORDS[*b as usize % WORDS.len()])
            .collect(),
    }
}

/** `n` input values from `data`, read from its end, each below 256 so most fit their type. */
pub fn values(data: &[u8], n: usize) -> Vec<i128> {
    (0..n)
        .map(|k| match data.len().checked_sub(k + 1) {
            Some(i) => i128::from(data[i]),
            None => 0,
        })
        .collect()
}
