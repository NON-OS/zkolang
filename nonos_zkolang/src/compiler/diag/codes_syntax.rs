/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Codes `E00xx` to `E02xx`: lexical, syntax, and names and modules. */

use super::codes::codes;

codes! {
    /* Lexical. */
    UNEXPECTED_CHAR = "E0001", "A character that begins no token. Outside comments and string literals only ASCII is allowed.";
    UNTERMINATED_COMMENT = "E0002", "A block comment `/*` without its closing `*/`. Block comments nest, so every `/*` needs its own `*/`.";
    UNTERMINATED_STRING = "E0003", "A string literal without its closing quote.";
    BAD_ESCAPE = "E0004", "A backslash in a string literal must be followed by one of `\\\"`, `\\\\`, `n`, `t` or `0`.";
    BAD_NUMBER = "E0005", "A malformed integer literal: a radix prefix with no digits, a digit the radix does not have, or an unknown suffix.";
    NUMBER_TOO_LARGE = "E0006", "An integer literal larger than any type can hold. The largest literal is 2^64 - 1.";
    RESERVED_WORD = "E0007", "A word reserved for a future edition cannot be used as a name.";
    /* Syntax. */
    UNEXPECTED_TOKEN = "E0100", "The parser expected something else at this point. The label says what it was looking for.";
    UNCLOSED_DELIMITER = "E0101", "An opening `(`, `[` or `{` without its matching close.";
    NESTING_TOO_DEEP = "E0102", "The source nests deeper than the compiler's bound of 64 levels: brackets, blocks, prefix operators, casts, postfix chains, and operators of different precedence each nest. A run of one operator, such as a long sum, and an `else if` chain do not. Deeper input would build a tree the compiler cannot walk safely; name inner parts with `let` bindings or type aliases.";
    CHAINED_COMPARISON = "E0103", "Comparisons do not chain: `a < b < c` compares a `bool` with `c`. Write `a < b && b < c`.";
    INCLUDE_REMOVED = "E0104", "Edition 2026 has no textual include. Declare the file as a module with `mod name;` and import its items with `use`.";
    MISPLACED_STATEMENT = "E0105", "`return`, `break`, `continue` and assignment are statements and may not appear inside a larger expression.";
    STRUCT_LITERAL_HERE = "E0106", "A struct literal cannot appear directly in the condition of `if`, `while`, `match` or `for`; wrap it in parentheses.";
    /* Names and modules. */
    UNRESOLVED_NAME = "E0200", "A name that does not resolve to any variable, item, import or crate in scope.";
    DUPLICATE_ITEM = "E0201", "Two items of one module share a name. Each module has one namespace for items.";
    PRIVATE_ITEM = "E0202", "An item or field that is not visible here. Mark it `pub` in its module to export it.";
    MODULE_NOT_FOUND = "E0203", "`mod name;` names a file that does not exist: neither `name.zkl` nor `name/mod.zkl` beside the declaring file.";
    MODULE_AMBIGUOUS = "E0204", "Both `name.zkl` and `name/mod.zkl` exist for one module; remove one.";
    WRONG_KIND = "E0205", "A name that resolves to the wrong kind of item for this position, such as a function used as a type.";
}
