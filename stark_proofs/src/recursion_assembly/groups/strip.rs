// NONOS Operating System (AGPL-3.0-or-later)
//! The compose strip's bindings: every echo cell equals its producer and
//! every final accumulator equals its acc cell in the flat region. Pure
//! cycles over cells the layout already resolved to absolute coordinates.

use super::super::layout::Layout;
use super::helpers::{cycle, Bind};
use alloc::vec::Vec;

pub fn strip(lay: &Layout, out: &mut Vec<Bind>) {
    for (a, b) in &lay.strip_cycles {
        out.push(cycle(lay.span, &[*a, *b]));
    }
}
