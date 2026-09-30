/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a reserved word that names a Rust feature is replaced by. */

/** What to write instead of a reserved word that names a Rust feature. */
pub(super) fn instead(w: &str) -> Option<&'static str> {
    Some(match w {
        "loop" => "every loop is bounded: write `while cond limit N { ... }` or `for i in a..b`",
        "static" => "write a constant with `const`",
        "trait" => "functions on a type go in an `impl Type { ... }` block",
        "ref" => "a binding pattern binds by value; drop `ref`",
        "where" => "there are no bounds to state: generic functions are checked per use",
        _ => return None,
    })
}
