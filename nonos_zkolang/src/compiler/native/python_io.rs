/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The Python target's inputs and result: a value per leaf, checked against its type and
 * laid into slots; the output slots read back into values.
 */

use alloc::format;
use alloc::string::String;

use super::leaves::bound;
use crate::compiler::driver::abi::Leaf;

/** Python that checks leaf `i`, `values[i]`, and lays it into the slots `inp`. */
pub(super) fn input(i: usize, leaf: Leaf) -> String {
    let b = bound(leaf);
    let lay = if b.wide {
        "inp += [x % 2**64 & 0xFFFFFFFF, x % 2**64 >> 32]"
    } else {
        "inp.append(x % P)"
    };
    format!(
        "x = values[{i}]\n    if type(x) is not int or not {} <= x <= {}:\n        raise ValueError(\"input {i} is not a value of its type\")\n    {lay}",
        b.lo, b.hi
    )
}

/** Python that appends output leaf `leaf`, read from `out` at `s`, to `result`. */
pub(super) fn output(leaf: Leaf) -> String {
    let b = bound(leaf);
    if b.wide {
        let sign = if b.signed {
            "x - 2**64 if x >= 2**63 else x"
        } else {
            "x"
        };
        format!("x = out[s] | out[s + 1] << 32; s += 2; result.append({sign})")
    } else if b.signed {
        String::from("x = out[s]; s += 1; result.append(x - P if x > P // 2 else x)")
    } else {
        String::from("result.append(out[s]); s += 1")
    }
}
