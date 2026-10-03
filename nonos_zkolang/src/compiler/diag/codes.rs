/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The stable diagnostic codes and their long descriptions. A code, once published, keeps
 * its meaning for the edition; `zkolang explain CODE` prints the description.
 *
 * Ranges: `E00xx` lexical, `E01xx` syntax, `E02xx` names and modules, `E03xx` types,
 * `E04xx` patterns, `E05xx` constants, `E06xx` secret flow, `E07xx` generics and
 * recursion, `E08xx` limits and registers, `E09xx` programs and packages, `E99xx` internal.
 * Warnings: `W00xx` code quality, `W01xx` cost.
 */

use super::{codes_program, codes_syntax, codes_types, codes_warnings};

/** A diagnostic code, such as `E0301`. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Code(pub &'static str);

/**
 * Declare codes as associated constants of `Code`, and the table of their descriptions as
 * `TABLE` in the module that invokes it.
 */
macro_rules! codes {
    ($($name:ident = $code:literal, $explain:literal;)*) => {
        impl $crate::compiler::diag::Code {
            $(pub const $name: $crate::compiler::diag::Code =
                $crate::compiler::diag::Code($code);)*
        }
        pub(super) const TABLE: &[(&str, &str)] = &[$(($code, $explain)),*];
    };
}
pub(super) use codes;

/** Every table, in code order. */
const TABLES: [&[(&str, &str)]; 4] = [
    codes_syntax::TABLE,
    codes_types::TABLE,
    codes_program::TABLE,
    codes_warnings::TABLE,
];

/** The long description of a code, for `zkolang explain`. */
pub fn explain(code: &str) -> Option<&'static str> {
    TABLES
        .iter()
        .flat_map(|t| t.iter())
        .find(|(c, _)| *c == code)
        .map(|(_, e)| *e)
}
