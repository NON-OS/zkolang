/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * How `abi` words a layout: the leaves of a value, and a count of slots.
 */

use nonos_zkolang::compiler::driver::abi::Leaf;

/** The leaves `ls`, one per slot group, as a reader writes their values. */
pub(super) fn shown(ls: &[Leaf]) -> String {
    let each: Vec<String> = ls
        .iter()
        .map(|l| match l {
            Leaf::Bool => String::from("bool"),
            Leaf::Field => String::from("field"),
            Leaf::Int(t) => String::from(t.name()),
            Leaf::Tag(n) => format!("tag below {n}"),
            Leaf::Slot => String::from("slot"),
        })
        .collect();
    format!("[{}]", each.join(", "))
}

/** `n` slots, in words. */
pub(super) fn count(n: usize) -> String {
    match n {
        1 => String::from("1 slot"),
        n => format!("{n} slots"),
    }
}
