/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Keywords, and the words reserved for later editions. */

use super::keyword_macro::keywords;

keywords! {
    As = "as",
    Assert = "assert",
    Bool = "bool",
    Break = "break",
    Const = "const",
    Continue = "continue",
    Crate = "crate",
    Declassify = "declassify",
    Else = "else",
    Enum = "enum",
    False = "false",
    Field = "field",
    Fn = "fn",
    For = "for",
    I8 = "i8",
    I16 = "i16",
    I32 = "i32",
    I64 = "i64",
    If = "if",
    Impl = "impl",
    In = "in",
    Let = "let",
    Limit = "limit",
    Match = "match",
    Mod = "mod",
    Mut = "mut",
    Pub = "pub",
    Public = "public",
    Return = "return",
    Secret = "secret",
    SelfValue = "self",
    SelfType = "Self",
    Struct = "struct",
    Super = "super",
    True = "true",
    Type = "type",
    U8 = "u8",
    U16 = "u16",
    U32 = "u32",
    U64 = "u64",
    Use = "use",
    Usize = "usize",
    While = "while",
}

/** Words reserved for a later edition: an error as a name. */
pub const RESERVED: &[&str] = &[
    "async", "await", "dyn", "extern", "include", "loop", "macro", "move", "ref", "static",
    "trait", "unsafe", "where", "yield",
];
