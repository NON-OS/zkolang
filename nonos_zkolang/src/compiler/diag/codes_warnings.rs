/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Warning codes: `W00xx` code quality and `W01xx` cost. */

use super::codes::codes;

codes! {
    /* Warnings. */
    UNUSED_VARIABLE = "W0001", "A variable that is never read. Prefix it with `_` to keep it.";
    UNUSED_VALUE = "W0002", "An expression statement whose value is discarded.";
    UNREACHABLE = "W0003", "Code that can never run, after a `return`, `break` or `continue`.";
    UNREACHABLE_PATTERN = "W0004", "A match arm that no value can reach, because earlier arms cover it.";
    DEPRECATED = "W0005", "Use of an item marked `#[deprecated]`.";
    USELESS_DECLASSIFY = "W0006", "`declassify` of a value that is already public.";
    MISPLACED_DOC = "W0007", "A doc comment that documents nothing.";
    COST_UNROLL = "W0100", "A loop unrolls to more iterations than the warning threshold. Every iteration is trace rows.";
    COST_DYNAMIC_INDEX = "W0101", "A runtime index into a long array. Each dynamic read or write costs about five rows per element.";
    COST_FUNCTION = "W0102", "A function whose inlined cost exceeds half of the largest provable trace.";
}
