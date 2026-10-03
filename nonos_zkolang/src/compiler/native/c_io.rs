/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The C target's inputs and result: a value per leaf read from the command line, checked
 * against its type and laid into slots; the output slots read back into values.
 */

use alloc::format;
use alloc::string::String;

use super::leaves::bound;
use crate::compiler::driver::abi::Leaf;

/** `v` as a C `i128`, written so that no literal overflows before it is negated. */
fn literal(v: i128) -> String {
    match v < 0 {
        true => format!("(-(i128){}ULL)", v.unsigned_abs()),
        false => format!("((i128){v}ULL)"),
    }
}

/** C that reads leaf `i` from `argv`, checks it, and lays it into the slots from `s`. */
pub(super) fn input(i: usize, leaf: Leaf) -> String {
    let b = bound(leaf);
    let lay = if b.wide {
        "u64 pat = (u64)x; in[s++] = pat & 0xFFFFFFFFULL; in[s++] = pat >> 32;"
    } else {
        "in[s++] = x < 0 ? (u64)(x + (i128)P) : (u64)x;"
    };
    format!(
        "{{ i128 x; if (!read_value(argv[{}], &x) || x < {} || x > {}) {{ fprintf(stderr, \"input {i} is not a value of its type\\n\"); return 1; }} {lay} }}",
        i + 1,
        literal(b.lo),
        literal(b.hi)
    )
}

/** C that prints output leaf `leaf` from the output slots from `s`. */
pub(super) fn output(leaf: Leaf) -> String {
    let b = bound(leaf);
    if b.wide {
        let sign = if b.signed {
            " if (x >= ((i128)1 << 63)) x -= (i128)1 << 64;"
        } else {
            ""
        };
        format!(
            "{{ i128 x = (i128)out[s] | ((i128)out[s + 1] << 32); s += 2;{sign} print_value(x); }}"
        )
    } else if b.signed {
        String::from("{ i128 x = out[s++]; if (x > (i128)(P / 2)) x -= (i128)P; print_value(x); }")
    } else {
        String::from("{ print_value((i128)out[s++]); }")
    }
}
