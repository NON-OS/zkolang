/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The characters a message names, and the ASCII characters some of them are mistaken for. */

/** A character's Unicode name and the ASCII character it looks like, if any. */
pub(super) const NAMED: &[(char, &str, Option<char>)] = &[
    ('\0', "NULL", None),
    ('`', "GRAVE ACCENT", None),
    ('\u{7f}', "DELETE", None),
    ('\u{a0}', "NO-BREAK SPACE", Some(' ')),
    ('\u{ad}', "SOFT HYPHEN", None),
    ('\u{d7}', "MULTIPLICATION SIGN", Some('*')),
    ('\u{37e}', "GREEK QUESTION MARK", Some(';')),
    ('\u{2009}', "THIN SPACE", Some(' ')),
    ('\u{200b}', "ZERO WIDTH SPACE", None),
    ('\u{200d}', "ZERO WIDTH JOINER", None),
    ('\u{2013}', "EN DASH", Some('-')),
    ('\u{2014}', "EM DASH", Some('-')),
    ('\u{2018}', "LEFT SINGLE QUOTATION MARK", Some('\'')),
    ('\u{2019}', "RIGHT SINGLE QUOTATION MARK", Some('\'')),
    ('\u{201c}', "LEFT DOUBLE QUOTATION MARK", Some('"')),
    ('\u{201d}', "RIGHT DOUBLE QUOTATION MARK", Some('"')),
    ('\u{202a}', "LEFT-TO-RIGHT EMBEDDING", None),
    ('\u{202b}', "RIGHT-TO-LEFT EMBEDDING", None),
    ('\u{202c}', "POP DIRECTIONAL FORMATTING", None),
    ('\u{202d}', "LEFT-TO-RIGHT OVERRIDE", None),
    ('\u{202e}', "RIGHT-TO-LEFT OVERRIDE", None),
    ('\u{2066}', "LEFT-TO-RIGHT ISOLATE", None),
    ('\u{2067}', "RIGHT-TO-LEFT ISOLATE", None),
    ('\u{2068}', "FIRST STRONG ISOLATE", None),
    ('\u{2069}', "POP DIRECTIONAL ISOLATE", None),
    ('\u{2212}', "MINUS SIGN", Some('-')),
    ('\u{3000}', "IDEOGRAPHIC SPACE", Some(' ')),
    ('\u{feff}', "ZERO WIDTH NO-BREAK SPACE", None),
    ('\u{ff08}', "FULLWIDTH LEFT PARENTHESIS", Some('(')),
    ('\u{ff09}', "FULLWIDTH RIGHT PARENTHESIS", Some(')')),
    ('\u{ff1b}', "FULLWIDTH SEMICOLON", Some(';')),
];
